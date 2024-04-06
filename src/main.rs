use actix_web::{
    error, get,
    http::{header::ContentType, StatusCode},
    web,
    web::ServiceConfig,
    HttpResponse,
};
use derive_more::{Display, Error};
use shuttle_actix_web::ShuttleActixWeb;

#[derive(Debug, Display, Error)]
enum ServerError {
    #[display(fmt = "Internal Server Error")]
    InternalError,
}

impl error::ResponseError for ServerError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            ServerError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[get("/-1/error")]
async fn error_page() -> Result<&'static str, ServerError> {
    Err(ServerError::InternalError)
}

#[get("/1/{ids:.*}")]
async fn cube_bits(path: web::Path<String>) -> Result<HttpResponse, ServerError> {
    let ids = path.into_inner();

    print!("{}", ids);

    let nums: Vec<i64> = ids
        .split('/')
        .map(|n| return n.parse::<i64>().unwrap_or(0))
        .collect();

    let mut res = 0;

    for num in nums {
        res = res ^ num;
    }

    res = res.pow(3);

    Ok(HttpResponse::Ok().body(res.to_string()))
}


#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(error_page);
        cfg.service(cube_bits);
    };

    Ok(config.into())
}
