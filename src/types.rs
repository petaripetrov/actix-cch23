use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use derive_more::{Display, Error};
use serde_json::json;

#[derive(Debug, Display, Error)]
pub enum ServerError {
    #[display(fmt = "500 Internal Server Error")]
    InternalError,
    PasswordError(PasswordErrors),
}

#[derive(Debug, Display, Error)]
pub enum PasswordErrors {
    #[display(fmt = "8 chars")]
    EightChars,
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
                _ => StatusCode::BAD_REQUEST,
            },
        }
    }
}

pub type EndpointRet = Result<HttpResponse, ServerError>;
