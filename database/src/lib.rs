use core::panic;
use std::env;

use diesel::{
    associations::HasTable,
    connection,
    pg::Pg,
    r2d2::{self, ConnectionManager, Pool},
    result::{self, Error},
    select, Connection, ExpressionMethods, OptionalExtension, PgConnection, QueryDsl, RunQueryDsl,
    SelectableHelper,
};
use dotenvy::dotenv;
use models::{NewNote, Note};
use schema::notes::dsl::*;
use schema::{
    notes::{self, user_id},
    users::{self, table},
};

pub mod models;
pub mod schema;

use self::models::{NewUser, User};

pub type DBPool = Pool<ConnectionManager<PgConnection>>;

macro_rules! db_conn {
    ($self:expr) => {
        &mut $self.pool.get().unwrap()
    };
}

pub struct Database {
    pub pool: DBPool,
}



impl Database {
    pub fn new() -> Self {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let result = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool");

        Database { pool: result }
    }

    pub fn create_user(&self, username: &str, password: &str) -> User {
        let new_user = NewUser {
            username: username,
            password: password,
            id: None,
        };

        diesel::insert_into(users::table)
            .values(new_user)
            .returning(User::as_returning())
            .get_result(db_conn!(self))
            .expect("Error creating user")
    }

    pub fn get_users(&self) -> Option<Vec<User>> {
        use self::schema::users::dsl::*;

        match users
            .load::<User>(db_conn!(self))
            .optional()
            .expect("Error loading users")
        {
            Some(results) => return Some(results),
            None => {
                println!("No users in db");
                return None;
            }
        };
    }
    pub fn get_user_by_username(&self, _username: String) -> Result<User, Error> {
        use self::schema::users::dsl::*;
        users
            .filter(username.eq(_username))
            .first::<User>(&mut self.pool.get().unwrap())
    }

    pub fn create_new_note(&self, data: NewNote) -> Result<(), Error> {
        use self::schema::notes::dsl::*;
        match diesel::insert_into(notes)
            .values(data)
            .returning(Note::as_returning())
            .get_result(db_conn!(self))
            .optional()
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    pub fn update_note_data_only(&self, data: String, n_id: i32, u_id: i32) -> Result<(), Error> {
        match diesel::update(notes::table.find(n_id).filter(notes::user_id.eq(u_id)))
            .set(notes::note_data.eq(data))
            .execute(db_conn!(self))
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }
    pub fn get_note_titles(&self, u_id: i32) -> Result<Vec<(String, i32)>, Error> {
        match notes::table
            .filter(notes::user_id.eq(u_id)) // Assuming `user_id` is a column in your `notes` table
            .select((notes::note_name, notes::id)) // Selecting only title and id columns
            .load::<(String, i32)>(db_conn!(self))
        {
            Ok(data) => Ok(data),
            Err(err) => Err(err.into()),
        }
    }
    pub fn get_note_by_id(&self, n_id: i32, u_id: i32) -> Result<Note, Error> {
        match notes::table
            .find(n_id)
            .filter(notes::user_id.eq(u_id))
            .first::<Note>(db_conn!(self))
        {
            Ok(_note) => Ok(_note),
            Err(err) => Err(err),
        }
    }
}
