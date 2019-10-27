use actix_web::web::{block, Data, HttpResponse, Json};
use futures::future::Future;
use validator::Validate;

use crate::config::Config;
use crate::utils::{db::DbPool,jwt::create_token};
use crate::errors::AppError;
use crate::models::user::verify_password;
use crate::models::{ResponseMsg, Login};
use crate::repositories::PgConnect;

fn login(pool: &DbPool, config: &Config, payload: &Login) -> Result<ResponseMsg<VueToken>, AppError> {
    // println!("{:?},{}",file!(),line!());
    payload.validate()?;
    let user_repository = PgConnect::new(pool.get()?);
    let user = user_repository.find_by_account(&payload.account)?;
    if let Ok(true) = verify_password(&payload.password, &user.password) {
        let token = create_token(user, &config.secret_key)?;
        // Ok(AuthResponse::new(user, token))
        Ok(ResponseMsg::new_ok(token))
    } else {
        Err(AppError::Unauthorized)
    }
}

pub fn login_service(
    payload: Json<Login>,
    config: Data<Config>,
    pool: Data<DbPool>,
) -> impl Future<Item = HttpResponse, Error = AppError> {
    block(move || login(&pool, &config, &payload))
        .and_then(|auth_response| Ok(HttpResponse::Ok().json(auth_response)))
        .from_err()
}


