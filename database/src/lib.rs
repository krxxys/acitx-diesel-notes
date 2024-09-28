use core::panic;
use std::env;

use diesel::{connection, pg::Pg, r2d2::{self, ConnectionManager, Pool}, result::{self, Error}, Connection, ExpressionMethods, OptionalExtension, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};
use dotenvy::dotenv;
use schema::users;


pub mod models; 
pub mod schema;

use self::models::{User, NewUser};

pub type DBPool = Pool<ConnectionManager<PgConnection>>;



pub struct Database {
    pub pool: DBPool
}

impl Database {
    pub fn new() -> Self {
        dotenv().ok(); 
        let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let result = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    Database { pool: result }
    }

    pub fn create_user(&self, username: &str, password: &str) -> User {

        let new_user = NewUser { username: username, password: password, id: None };
        
        diesel::insert_into(users::table)
            .values(new_user)
            .returning(User::as_returning())
            .get_result(&mut self.pool.get().unwrap())
            .expect("Error creating user")
    }
    
    pub fn get_users(&self) -> Option<Vec<User>> {
        use self::schema::users::dsl::*;
    
        match users.load::<User>(&mut self.pool.get().unwrap())
            .optional()
            .expect("Error loading users")
        {
            Some(results) => {
                return Some(results)
            },
            None => {
                println!("No users in db");
                return None
            }
        };
    }
    pub fn get_user_by_username(&self, _username: String) -> Result<User, Error> {
        use self::schema::users::dsl::*; 
        users.filter(username.eq(_username)).first::<User>(&mut self.pool.get().unwrap())
    }
}