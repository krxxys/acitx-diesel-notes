use actix_web::{post, web, HttpResponse, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use bcrypt::verify;
use database::Database;
use serde::{Deserialize, Serialize};

use crate::{auth::create_token, Message}; 

#[derive(Deserialize, Serialize)]
pub struct UserData {
    pub username: String, 
    pub password: String
}

pub async fn login(db: web::Data<Database>, login_data: web::Json<UserData>) -> impl Responder {

    let user = db.get_user_by_username(login_data.username.clone());
    match user {
        Ok(user) => {
            let verify = verify(login_data.password.clone(), user.password.as_str());
            match verify {
                Ok(is_okay) => {
                    if is_okay {
                        //HttpResponse::Ok().json(Message {message: String::from("Logged"),status:  None})
                        match create_token(&user.username).await {
                            Ok(token) => {
                                return HttpResponse::Ok().json(token)
                            }, 
                            Err(err) => {
                                return  HttpResponse::InternalServerError().body("Error while creating auth token")
                            }
                        };
                    } else 
                    {
                        HttpResponse::Ok().json(Message {message: String::from("Not                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     Logged"),status:  None})
                    }
                }, 
                Err(err) => {
                    HttpResponse::InternalServerError().body("Error")
                }
            }
        },
        Err(_) => {
            HttpResponse::InternalServerError().body("Error")
        },
    }
}

pub async fn check_auth(auth: BearerAuth) -> impl Responder {
    format!("authenticated for token: {}", auth.token().to_owned())
}

