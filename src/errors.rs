use actix_web::error::{BlockingError, JsonPayloadError};
use actix_web::{http, HttpResponse, ResponseError};
use bcrypt;
use diesel;
use failure::Fail;
use jsonwebtoken;
use log::error;
use r2d2;
use reqwest;
use serde::Serialize;
use validator;

#[derive(Debug, Serialize)]
pub struct ApiError {
    message: String,
    errors: Option<validator::ValidationErrors>,
}

impl ApiError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_owned(),
            errors: None,
        }
    }

    pub fn with_errors(message: &str, errors: validator::ValidationErrors) -> Self {
        Self {
            message: message.to_owned(),
            errors: Some(errors),
        }
    }
}

#[derive(Fail, Debug)]
pub enum AppError {
    #[fail(display = "internal error")]
    Internal,
    #[fail(display = "bad request")]
    BadRequest(ApiError),
    #[fail(display = "not found")]
    NotFound,
    #[fail(display = "timeout")]
    Timeout,
    #[fail(display = "unauthorized")]
    Unauthorized,
}

impl ResponseError for AppError {
    fn render_response(&self) -> HttpResponse {
        match *self {
            AppError::Internal => HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR),
            AppError::BadRequest(ref api_error) => {
                HttpResponse::build(http::StatusCode::BAD_REQUEST).json(api_error)
            }
            AppError::NotFound => HttpResponse::new(http::StatusCode::NOT_FOUND),
            AppError::Timeout => HttpResponse::new(http::StatusCode::GATEWAY_TIMEOUT),
            AppError::Unauthorized => HttpResponse::new(http::StatusCode::UNAUTHORIZED),
        }
    }
}

impl From<diesel::result::Error> for AppError {
    fn from(error: diesel::result::Error) -> Self {
        error!("ERROR diesel = {:?}", error);
        match error {
            diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UniqueViolation,
                _,
            ) => AppError::BadRequest(ApiError::new("already.exist")),
            diesel::result::Error::NotFound => AppError::NotFound,
            _ => AppError::Internal,
        }
    }
}

impl From<JsonPayloadError> for AppError {
    fn from(error: JsonPayloadError) -> Self {
        error!("ERROR actix JsonPayloadError = {:?}", error);
        match error {
            JsonPayloadError::Deserialize(json_error) => {
                AppError::BadRequest(ApiError::new(&format!("{}", json_error)))
            }
            _ => AppError::BadRequest(ApiError::new("Json parsing error")),
        }
    }
}

impl From<BlockingError<AppError>> for AppError {
    fn from(error: BlockingError<Self>) -> Self {
        error!("ERROR actix BlockingError = {:?}", error);
        match error {
            BlockingError::Error(app_error) => app_error,
            BlockingError::Canceled => AppError::Internal,
        }
    }
}

impl From<r2d2::Error> for AppError {
    fn from(error: r2d2::Error) -> Self {
        error!("ERROR r2d2 = {:?}", error);
        AppError::Internal
    }
}

impl From<bcrypt::BcryptError> for AppError {
    fn from(error: bcrypt::BcryptError) -> Self {
        error!("ERROR bcrypt = {:?}", error);
        AppError::Internal
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(error: jsonwebtoken::errors::Error) -> Self {
        error!("ERROR jsonwebtoken = {:?}", error);
        AppError::Unauthorized
    }
}

impl From<reqwest::Error> for AppError {
    fn from(error: reqwest::Error) -> Self {
        error!("ERROR reqwest = {:?}", error);
        AppError::Internal
    }
}

impl From<validator::ValidationErrors> for AppError {
    fn from(error: validator::ValidationErrors) -> Self {
        error!("ERROR validator = {:?}", error);
        AppError::BadRequest(error.into())
    }
}

impl From<validator::ValidationErrors> for ApiError {
    fn from(errors: validator::ValidationErrors) -> Self {
        error!("ERROR validator = {:?}", errors);
        Self::with_errors("validation.error", errors)
    }
}
