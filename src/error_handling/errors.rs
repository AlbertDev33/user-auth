use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;
use diesel::result::{DatabaseErrorKind, Error as DBError};
use uuid::Error as ParseError;

#[derive(Debug, Display)]
pub enum ServiceError {
    #[display(fmt = "Internal Server Error")]
    InternaServerError,

    #[display(fmt = "BadRequest: {}", _0)]
    BadRequest(String),

    #[display(fmt = "Unauthorized")]
    Unauthorized,
}

// impl ResponseError trait allows to convert our errors into http responses with appropriate data
impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        return match self {
            ServiceError::InternaServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error, Please try later")
            }
            ServiceError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            ServiceError::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized"),
        };
    }
}

impl From<ParseError> for ServiceError {
    fn from(_: ParseError) -> Self {
        return ServiceError::BadRequest("Invalid UUID".into());
    }
}

impl From<r2d2::Error> for ServiceError {
    fn from(_: r2d2::Error) -> Self {
        return ServiceError::InternaServerError;
    }
}

impl From<DBError> for ServiceError {
    fn from(error: DBError) -> ServiceError {
        return match error {
            DBError::DatabaseError(kind, info) => {
                if let DatabaseErrorKind::UniqueViolation = kind {
                    let message = info.details().unwrap_or_else(|| info.message()).to_string();
                    return ServiceError::BadRequest(message);
                }
                ServiceError::InternaServerError
            }
            _ => ServiceError::InternaServerError,
        };
    }
}
