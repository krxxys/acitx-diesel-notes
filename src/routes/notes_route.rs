use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};
use actix_web_httpauth::{extractors::bearer::BearerAuth, headers::authorization::Bearer};
use database::{models::NewNote, Database};

use dotenvy::var;
use jsonwebtoken::{decode, DecodingKey, TokenData, Validation};
use serde::{Deserialize, Serialize};

use crate::auth::{self, Claims};

#[derive(Serialize, Deserialize)]
pub struct CreateNote {
    pub note_name: String,
    pub note_tag: String,
    pub note_data: String,
}

macro_rules! account_id {
    ($req:expr) => {
        match $req.extensions().get::<TokenData<Claims>>() {
            Some(token_data) => {
                token_data.claims.account_id
            },
            None => {
                return HttpResponse::Unauthorized().body("Heeelp me!!1://");
            },
        };
    };
}


pub async fn create_note(req: HttpRequest,
    db: web::Data<Database>,
    data: web::Json<CreateNote>,
    auth: BearerAuth,
) -> impl Responder {
 
    let new_note = NewNote {
        user_id: account_id!(req),
        note_name: data.note_name.as_str(),
        note_data: data.note_data.as_str(),
        note_tag: data.note_tag.as_str(),
    };
    match db.create_new_note(new_note) {
        Ok(()) => HttpResponse::Ok().body("Succefully created note"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}


pub async fn get_note_titles(req: HttpRequest,  db: web::Data<Database>, auth: BearerAuth) -> impl Responder {

    let account_id = account_id!(req);

    match db.get_note_titles(account_id) {
        Ok(result) => {
            return HttpResponse::Ok().json(result);
        }
        Err(err) => {
            return HttpResponse::InternalServerError().body(err.to_string());
        }
    };
}
#[derive(Serialize, Deserialize)]
pub struct DataUpdateStruct {
    pub id: i32,
    pub data: String,
}

pub async fn update_note_data(req: HttpRequest,
    db: web::Data<Database>,
    data: web::Json<DataUpdateStruct>,
    auth: BearerAuth,
) -> impl Responder {
    match db.update_note_data_only(data.data.clone(), data.id, account_id!(req)) {
        Ok(_) => {
            return HttpResponse::Ok().body("Succefuly updated");
        }
        Err(_) => {
            return HttpResponse::InternalServerError().body("Something gone wrong try again");
        }
    }
}
pub async fn get_note_by_id(req: HttpRequest,
    db: web::Data<Database>,
    note_id: web::Json<i32>,
    auth: BearerAuth,
) -> impl Responder {

    match db.get_note_by_id(note_id.clone(), account_id!(req)) {
        Ok(note) => {
            return HttpResponse::Ok().json(note);
        }
        Err(err) => {
            return HttpResponse::InternalServerError().body(err.to_string());
        }
    }
}