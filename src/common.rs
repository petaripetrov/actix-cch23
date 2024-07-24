use std::{collections::HashMap, sync::Mutex, time::Instant};

use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use derive_more::{Display, Error};
use serde_json::json;
use sqlx::PgPool;

#[derive(Debug, Display, Error)]
pub enum ServerError {
    #[display(fmt = "500 Internal Server Error")]
    InternalError,
    PasswordError(PasswordErrors),
}

#[derive(Debug, Display, Error)]
pub enum PasswordErrors {
    #[display(fmt = "8 chars")]
    LessEightChars,
    #[display(fmt = "more types of chars")]
    MissingCharacterTypes,
    #[display(fmt = "55555")]
    LessFiveDigits,
    #[display(fmt = "math is hard")]
    MathIsHard,
    #[display(fmt = "not joyful enough")]
    IOYOutOrder,
    #[display(fmt = "illegal: no sandwich")]
    MissingSandwich,
    #[display(fmt = "outranged")]
    UnicodeOutOfRange,
    #[display(fmt = "😳")]
    MissingEmoji,
    #[display(fmt = "not a coffee brewer")]
    ShaNotEndWithA,
}

impl error::ResponseError for ServerError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        if let ServerError::PasswordError(inner) = self {
            return HttpResponse::build(self.status_code())
                .insert_header(ContentType::html())
                .json(json!({
                    "result": "naughty",
                    "reason": inner.to_string(),
                }));
        }

        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            ServerError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            ServerError::PasswordError(inner) => match inner {
                PasswordErrors::IOYOutOrder => StatusCode::NOT_ACCEPTABLE,
                PasswordErrors::MissingSandwich => StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS,
                PasswordErrors::UnicodeOutOfRange => StatusCode::RANGE_NOT_SATISFIABLE,
                PasswordErrors::MissingEmoji => StatusCode::UPGRADE_REQUIRED,
                PasswordErrors::ShaNotEndWithA => StatusCode::IM_A_TEAPOT,
                _ => StatusCode::BAD_REQUEST,
            },
        }
    }
}

pub type EndpointRet = Result<HttpResponse, ServerError>;

pub struct AppState {
    pub log: Mutex<HashMap<String, Instant>>,
    pub pool: PgPool,
}
