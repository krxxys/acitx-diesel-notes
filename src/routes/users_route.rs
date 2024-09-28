use actix_web::{web, HttpResponse, Responder};
use bcrypt::hash;
use database::Database;
use serde::{Serialize, Deserialize};


pub async fn get_all_users(db: web::Data<Database>) -> impl Responder {
    match db.get_users() {
        Some(users) => {
            return HttpResponse::Ok().json(users);
        },
        None => {
            return HttpResponse::NotFound().body("Not found");
        },
    }    
}

#[derive(Deserialize, Serialize)]
pub struct UserData {
    username: String,
    password: Option<String>
}

#[derive(Serialize)]
pub struct Message {
    pub(crate) message: String, 
    pub(crate) status: Option<i32>
}


pub async fn get_user_by_username(db: web::Data<Database>,user_data:  web::Json<UserData>) -> impl Responder {
        let user = db.get_user_by_username(user_data.username.clone());
        match user {
            Ok(user) => {
                HttpResponse::Ok().json(user)
            }, 
            Err(_) => {
                 HttpResponse::NotFound().body("No user with this username")
            }
        } 
}
pub async fn create_user(db: web::Data<Database>, user_data: web::Json<UserData>) -> impl Responder {
    //check if user with gived username exist 
    let check = db.get_user_by_username(user_data.username.clone()); 

    match check {
        Ok(user) => {
            HttpResponse::InternalServerError().json(Message {message: "User with given username exist".to_owned(), status: None })
        }, 
        Err(_) => {
            let hashed_password = hash(user_data.password.clone().expect("msg"), 10).expect("Failed to hash password");
            let user = db.create_user(user_data.username.as_str(),&hashed_password);
            HttpResponse::Ok().json(user)
        }
    }
}

