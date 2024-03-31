use actix_web::{
    error, get, 
    http::{header::ContentType, StatusCode}, 
    web::ServiceConfig, 
    HttpResponse};
use shuttle_actix_web::ShuttleActixWeb;
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
enum ServerError {
    #[display(fmt = "Internal Server Error")]
    InternalError
}

impl error::ResponseError for ServerError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            ServerError::InternalError => StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

#[get("/")]
async fn hello_world() -> &'static str {
    "Hello World!"
}

#[get("/-1/error")]
async fn error_page() -> Result<&'static str, ServerError> {
    Err(ServerError::InternalError)
}

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello_world);
        cfg.service(error_page);
    };

    Ok(config.into())
}
