use actix_web::dev::Payload;
use actix_web::{Error, FromRequest, HttpRequest};
use derive_new::new;

use crate::config::Config;
use crate::errors::AppError;
use supper::jwt::{decode_token, Claime};

#[derive(Debug, new)]
pub struct Auth {
    pub claime: Claime,
}

impl FromRequest for Auth {
    type Error = Error;
    type Config = ();
    type Future = Result<Self, Self::Error>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let secret_key = req
            .app_data::<Config>()
            .expect("secret_key not found")
            .secret_key
            .clone();
        req.headers()
            .get("X-Token") //Authorization
            .ok_or_else(|| AppError::Unauthorized)
            .and_then(|token| token.to_str().map_err(|_| AppError::Unauthorized))
            .and_then(|token| decode_token(token, &secret_key))
            .map(Self::new)
            .map_err(Into::into)
    }
}
