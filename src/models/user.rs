use std::convert::TryFrom;

use bcrypt::{hash, verify, BcryptError, DEFAULT_COST};
use chrono::{NaiveDateTime}; //Local
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// use crate::errors::AppError;
// use crate::schema::users;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable)]
#[table_name = "sys_user"]
pub struct User {
    pub id: Uuid,
    pub account: String,
    //#[serde(skip_serializing, skip_deserializing)]
    pub password: String,
    pub name: String,
    pub org_id:Uuid,
    pub email: Option<String>,,
    pub sex:i32,
    pub phone: Option<String>,
    pub photo: Option<String>,
    pub note: Option<String>,
    pub status: i32,
    pub created: NaiveDateTime,    
}


// impl User {
//     fn new(
//         login: impl Into<String>,
//         email: impl Into<String>,
//         password: impl Into<String>,
//     ) -> Self {
//         Self {
//             uuid: Uuid::new_v4(),
//             login: login.into(),
//             email: email.into(),
//             password: password.into(),
//             created: Utc::now().naive_utc(),
//             updated: Utc::now().naive_utc(),
//             username:"admin".to_string(),
//         }
//     }

//     pub fn new_secure(
//         login: impl Into<String>,
//         email: impl Into<String>,
//         password: impl Into<String>,
//     ) -> Result<Self, AppError> {
//         let hashed_password = hash_password(&password.into())?;
//         let user = Self::new(login, email, hashed_password);
//         Ok(user)
//     }
// }



pub fn hash_password(password: &str) -> Result<String, BcryptError> {
    Ok(hash(password, DEFAULT_COST)?)
}

pub fn verify_password(password: &str, hashed_password: &str) -> Result<bool, BcryptError> {
    Ok(verify(password, hashed_password)?)
}
