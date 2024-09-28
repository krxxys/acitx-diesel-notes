use std::env;

use actix_web::{dev::ServiceRequest, error, Error};

use dotenvy::dotenv;
use serde::{Deserialize, Serialize}; 
use time::{OffsetDateTime, ext::NumericalDuration};


pub async fn validator(req: ServiceRequest, credentials: Option<actix_web_httpauth::extractors::bearer::BearerAuth>) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    
    let Some(credentials) = credentials else {
        return Err((error::ErrorBadRequest("No bearer header"), req)); 
    }; 
    
    match verify_token(credentials.token()).await {
        Ok(_) => (
            Ok(req)
        ),
        Err(_) => {
            return Err((error::ErrorUnauthorized(" Bad token"), req));
        },
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    exp: usize,
    sub: Option<String>
}

pub async fn create_token(username: &str) -> Result<String, jsonwebtoken::errors::Error>{
    use jsonwebtoken::{encode, errors::ErrorKind, EncodingKey, Header};

    dotenv().ok(); 
    let secret = env::var("SECRET_JWT").expect("Please add key for SECRET_JWT"); 
    
    let claims = Claims {
        exp: OffsetDateTime::now_utc().checked_add(1.days()).expect("Overflow when adding").unix_timestamp() as usize, 
        sub: Some(username.to_owned())
    };

  let token = match encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()) ){
    Ok(t) => return Ok(t),
    Err(err) => match *err.kind() {
        ErrorKind::InvalidSubject => {
            return Err(err);
        }, 
        _ => {
            return Err(err);
        }
    },
  };
}

pub async fn verify_token(token: &str) -> Result<(), jsonwebtoken::errors::Error> {
    use jsonwebtoken::{decode, DecodingKey, Validation};

    dotenv().ok(); 
    let secret = env::var("SECRET_JWT").expect("Please add key for SECRET_JWT"); 
    match decode::<Claims>(token, &DecodingKey::from_secret(secret.as_bytes()), &Validation::default()) {
        Ok(_token) => {
            return Ok(())
        },
        Err(err) => {
            return Err(err)
        },
    }
}



