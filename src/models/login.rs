use serde::{Deserialize,Serialize};
use validator::Validate;
use validator_derive::Validate;

#[derive(Debug, Clone, Validate, Deserialize)]
pub struct Login {
    // #[validate(email(message = "validation.email"))]
    // pub email: String,
    #[validate(length(min = 5, message = "validate.account.short"))]
    pub account: String,
    #[validate(length(min = 6, message = "validation.password.short"))]
    pub password: String,
}


#[derive(Debug, Serialize)]
pub struct LoginUserInfo {
	pub roles:Vec<String>,
	pub introduction: Option<String>,
	pub avatar:Option<String>,
	pub name:String,
}