use derive_new::new;
use jsonwebtoken::{decode, encode, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::errors::AppError;
use crate::models::user::User;

#[derive(Debug, new, Serialize, Deserialize)]
pub struct Claime {
    pub user: User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Token(String);

#[derive(Debug,Serialize, Deserialize)]
pub struct VueToken {
    pub token: String,
}



pub fn create_token(user: User, secret_key: &str) -> Result<VueToken, AppError> {
    let claims = Claime::new(user);
    let token = encode(&Header::default(), &claims, secret_key.as_bytes())?;
    Ok(VueToken{token,})
}

pub fn decode_token(token: &str, secret_key: &str) -> Result<Claime, AppError> {
    let validation = Validation {
        validate_exp: false,
        ..Validation::default()
    };
    let claims = decode::<Claime>(token, secret_key.as_bytes(), &validation)?;
    let claims = claims.claims;
    Ok(claims)
}
