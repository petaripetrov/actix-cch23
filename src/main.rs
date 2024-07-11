use actix_web::{
    error, get,
    http::{header::ContentType, StatusCode},
    post,
    web::{self, ServiceConfig},
    HttpResponse,
};
use derive_more::{Display, Error};
use serde::{Deserialize, Serialize};
use shuttle_actix_web::ShuttleActixWeb;

#[derive(PartialEq, PartialOrd, Deserialize, Clone, Copy)]
struct Speed(f64);

impl Default for Speed {
    fn default() -> Self {
        Speed(0.0)
    }
}

impl Eq for Speed {}

impl Ord for Speed {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl std::fmt::Display for Speed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Deserialize)]
struct Deer {
    name: String,
    strength: i64,
    #[serde(default)]
    speed: Speed,
    #[serde(default)]
    height: i64,
    #[serde(default)]
    antler_width: i64,
    #[serde(default)]
    snow_magic_power: i64,
    #[serde(default)]
    favorite_food: String,
    #[serde(default)]
    #[serde(rename(serialize = "cAnD13s_3ATeN-yesT3rdAy"))]
    candies_eaten_yesterday: i64,
}

#[derive(Serialize)]
struct DeerResponse {
    fastest: String,
    tallest: String,
    magician: String,
    consumer: String,
}

#[derive(Debug, Display, Error)]
enum ServerError {
    #[display(fmt = "500 Internal Server Error")]
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

#[get("/")]
async fn index_page() -> Result<HttpResponse, ServerError> {
    Ok(HttpResponse::Ok().finish())
}

#[get("/-1/error")]
async fn error_page() -> Result<HttpResponse, ServerError> {
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

#[post("/4/strength")]
async fn strength(deer: web::Json<Vec<Deer>>) -> Result<HttpResponse, ServerError> {
    let strength: i64 = deer.iter().map(|d| d.strength).sum();

    Ok(HttpResponse::Ok().body(strength.to_string()))
}

#[post("/4/contest")]
async fn contest(deer: web::Json<Vec<Deer>>) -> Result<HttpResponse, ServerError> {
    let deer_iter = deer.iter();

    let fastest = deer_iter.clone().max_by_key(|d| d.speed).unwrap();
    let tallest = deer_iter.clone().max_by_key(|d| d.height).unwrap();
    let magician = deer_iter
        .clone()
        .max_by_key(|d| d.snow_magic_power)
        .unwrap();
    let consumer = deer_iter.max_by_key(|d| d.candies_eaten_yesterday).unwrap();

    let response = DeerResponse {
        fastest: format!(
            "Speeding past the finish line with a strength of {0} is {1}",
            fastest.strength, fastest.name
        ),
        tallest: format!(
            "{0} is standing tall with his {1} cm wide antlers",
            tallest.name, tallest.antler_width
        ),
        magician: format!(
            "{0} could blast you away with a snow magic power of {1}",
            magician.name, magician.snow_magic_power
        ),
        consumer: format!(
            "{0} ate lots of candies, but also some {1}",
            consumer.name, consumer.favorite_food
        ),
    };

    Ok(HttpResponse::Ok().json(response))
}

#[derive(Deserialize)]
struct NamesListParam {
    offset: Option<usize>,
    limit: Option<usize>,
    split: Option<usize>,
}

// http://localhost:8000/5?offset=3&limit=5
#[post("/5")]
async fn names_list(
    params: web::Query<NamesListParam>,
    kids: web::Json<Vec<String>>,
) -> Result<HttpResponse, ServerError> {
    let offset = params.offset.unwrap_or_default();
    let limit = params.limit.unwrap_or(kids.len());

    if let Some(split) = params.split {
        print!("{:?}", split);
        Ok(HttpResponse::Ok().json(
            kids[offset..]
                .chunks(split)
                // we limit the amount of CHUNKS, not the amount of results
                .take(limit)
                .collect::<Vec<_>>()
        ))
    } else {
        print!("nothing");
        Ok(HttpResponse::Ok().json(
            kids.iter()
                .skip(offset)
                .take(limit)
                .collect::<Vec<&String>>(),
        ))
    }
}

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(index_page); // maybe replace this with a page with links to the various tasks
        cfg.service(error_page);
        cfg.service(cube_bits);
        cfg.service(strength);
        cfg.service(contest);
        cfg.service(names_list);
    };

    Ok(config.into())
}
