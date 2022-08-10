use actix_web::HttpResponse;
use log::error;
use thiserror::Error;

use crate::context;
use crate::utils::template::TEMPLATE_ENGINE;

enum ErrorSeverity {
    None,
    Severe,
}

#[derive(Debug, Error)]
pub enum YarmsError {
    #[error("Not Found")]
    NotFound,
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Forbidden")]
    Forbidden,
    #[error("Internal server error")]
    Internal,
    #[error("Internal database error: {0}")]
    InternalDatabase(sea_orm::DbErr),
    #[error("Internal template error")]
    InternalTemplate,
}

impl YarmsError {
    fn message(&self) -> &str {
        match &self {
            Self::NotFound => "Not found",
            Self::Unauthorized => "Unauthorized",
            Self::Forbidden => "Forbidden",
            Self::Internal => "Internal server error",
            Self::InternalDatabase(_) => "Internal database error",
            Self::InternalTemplate => "Internal template error",
        }
    }
    
    fn severity(&self) -> ErrorSeverity {
        match &self {
            Self::NotFound | Self::Unauthorized | Self::Forbidden => ErrorSeverity::None,
            Self::Internal | Self::InternalDatabase(_) | Self::InternalTemplate => ErrorSeverity::Severe,
        }
    }
}

impl actix_web::error::ResponseError for YarmsError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match &self {
            Self::NotFound => actix_web::http::StatusCode::NOT_FOUND,
            Self::Unauthorized => actix_web::http::StatusCode::UNAUTHORIZED,
            Self::Forbidden => actix_web::http::StatusCode::FORBIDDEN,
            Self::Internal | 
            Self::InternalDatabase(_) |
            Self::InternalTemplate => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        match self.severity() {
            ErrorSeverity::Severe => log::error!("A severe error ocurred when trying to handle a request: {}", &self),
            _ => {},
        }

        let context = context!(
            "status_code" => self.status_code().as_str(),
            "message" => self.message(),
        );

        match TEMPLATE_ENGINE.render("error.html.j2", &context) {
            Ok(body) => {
                HttpResponse::build(self.status_code()).body(body)
            },
            Err(err) => {
                error!("Error ocurred while rendering error page: {:?}", err);
                HttpResponse::new(self.status_code())
            },
        }
    }
}

macro_rules! to_yarms_error {
    ($from:ty, $to:expr) => {
        impl From<$from> for YarmsError {
            fn from(error: $from) -> Self {
                $to(error)
            }
        }
    };
}

to_yarms_error!(anyhow::Error, |_| YarmsError::Internal);
to_yarms_error!(pbkdf2::password_hash::Error, |_| YarmsError::Internal);
to_yarms_error!(sea_orm::DbErr, YarmsError::InternalDatabase);
to_yarms_error!(tera::Error, |_| YarmsError::InternalTemplate);
to_yarms_error!(std::num::ParseIntError, |_| YarmsError::Internal);
to_yarms_error!(serde_json::Error, |_| YarmsError::Internal);

pub type YarmsResult<T> = std::result::Result<T, YarmsError>;
