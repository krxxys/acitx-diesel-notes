use std::env;

use actix_web::{dev::ServiceRequest, error, Error, HttpMessage};

use database::models::User;
use dotenvy::dotenv;
use jsonwebtoken::TokenData;
use serde::{Deserialize, Serialize};
use time::{ext::NumericalDuration, OffsetDateTime};

pub async fn validator(
    req: ServiceRequest,
    credentials: Option<actix_web_httpauth::extractors::bearer::BearerAuth>,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let Some(credentials) = credentials else {
        return Err((error::ErrorBadRequest("No bearer header"), req));
    };

    match verify_token(credentials.token()).await {
        Ok(token_data) => {
            req.extensions_mut().insert(token_data);  
            Ok(req)
        },
        Err(_) => {
            return Err((error::ErrorUnauthorized(" Bad token"), req));
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub sub: Option<String>,
    pub account_id: i32,
}

pub async fn create_token(user: User) -> Result<String, jsonwebtoken::errors::Error> {
    use jsonwebtoken::{encode, errors::ErrorKind, EncodingKey, Header};

    dotenv().ok();
    let secret = env::var("SECRET_JWT").expect("Please add key for SECRET_JWT");

    let claims = Claims {
        exp: OffsetDateTime::now_utc()
            .checked_add(1.days())
            .expect("Overflow when adding")
            .unix_timestamp() as usize,
        sub: Some(user.username.to_owned()),
        account_id: user.id,
    };

    let token = match encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    ) {
        Ok(t) => return Ok(t),
        Err(err) => match *err.kind() {
            ErrorKind::InvalidSubject => {
                return Err(err);
            }
            _ => {
                return Err(err);
            }
        },
    };
}

pub async fn verify_token(token: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    use jsonwebtoken::{decode, DecodingKey, Validation};

    dotenv().ok();
    let secret = env::var("SECRET_JWT").expect("Please add key for SECRET_JWT");
    match decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    ) {
        Ok(_token) => return Ok(_token),
        Err(err) => return Err(err),
    }
}
