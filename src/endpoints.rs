use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{
    error, get,
    http::{header::ContentType, StatusCode},
    post, web, HttpRequest, HttpResponse,
};
use base64::{engine::general_purpose, Engine};
use chrono::{DateTime, Datelike, Utc};
use derive_more::{Display, Error};
use image::{io::Reader as ImageRader, Rgb};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;
use std::{collections::HashMap, sync::Mutex, time::Instant};
use tinytemplate::TinyTemplate;
use ulid::Ulid;
use uuid::Uuid;

pub struct AppState {
    pub log: Mutex<HashMap<String, Instant>>,
    pub pool: PgPool,
}

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

    // Probably can replace this with only one loop
    let fastest = deer_iter.clone().max_by_key(|d| d.speed).unwrap();
    let tallest = deer_iter.clone().max_by_key(|d| d.height).unwrap();
    let magician = deer_iter
        .clone()
        .max_by_key(|d| d.snow_magic_power)
        .unwrap();
    let consumer = deer_iter.max_by_key(|d| d.candies_eaten_yesterday).unwrap();

    let res = json!({
        "fastest": format!(
            "Speeding past the finish line with a strength of {0} is {1}",
            fastest.strength, fastest.name
        ),
        "tallest": format!(
            "{0} is standing tall with his {1} cm wide antlers",
            tallest.name, tallest.antler_width
        ),
        "magician": format!(
            "{0} could blast you away with a snow magic power of {1}",
            magician.name, magician.snow_magic_power
        ),
        "consumer": format!(
            "{0} ate lots of candies, but also some {1}",
            consumer.name, consumer.favorite_food
        ),
    });

    Ok(HttpResponse::Ok().json(res))
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
                .collect::<Vec<_>>(),
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

// TODO figure out how to properly document this
///
/// > curl -X POST http://localhost:8000/6 \
/// >  -H 'Content-Type: text/plain' \
/// >  -d 'The mischievous elf peeked out from behind the toy workshop,
/// >      and another elf joined in the festive dance.
/// >      Look, there is also an elf on that shelf!'
///
/// {"elf":4}
///
#[post("/6")]
async fn elf_on_shelf(text: String) -> Result<HttpResponse, ServerError> {
    let elf_on_a_shelf = b"elf on a shelf";
    let shelf_count = text.matches("shelf").count();
    let elf_on_shelf = text
        .as_bytes()
        .windows(elf_on_a_shelf.len())
        .filter(|window| window == elf_on_a_shelf)
        .count();

    let res = json!({
        "elf": text.matches("elf").count(),
        "elf on a shelf": elf_on_shelf,
        "shelf with no elf on it": shelf_count - elf_on_shelf
    });

    Ok(HttpResponse::Ok().json(res))
}

#[get("/7/decode")]
async fn decode(req: HttpRequest) -> Result<HttpResponse, ServerError> {
    let cookie = match req.cookie("recipe") {
        Some(val) => val,
        None => return Err(ServerError::InternalError),
    };

    let val = cookie.value();
    let decoded = general_purpose::STANDARD.decode(val).unwrap();

    Ok(HttpResponse::Ok().body(decoded))
}

#[derive(Deserialize, Serialize)]
struct BakingData {
    recipe: HashMap<String, usize>,
    pantry: HashMap<String, usize>,
}

#[get("/7/bake")]
async fn bake(req: HttpRequest) -> Result<HttpResponse, ServerError> {
    let cookie = match req.cookie("recipe") {
        Some(val) => val,
        None => return Err(ServerError::InternalError),
    };

    let val = cookie.value();
    let decoded = general_purpose::STANDARD.decode(val).unwrap();

    let baking_data: BakingData = match serde_json::from_slice(&decoded) {
        Ok(val) => val,
        Err(e) => {
            println!("{}", e);
            return Err(ServerError::InternalError);
        }
    };

    let mut cookie_count: usize = usize::MAX;

    // TODO possibly refactor this with fold left
    for (ingredient, amount) in baking_data.recipe.iter() {
        // Skip 0 amount to save time
        // not really necessary, but good practice
        if *amount <= 0 {
            continue;
        }

        cookie_count = Ord::min(
            cookie_count,
            baking_data.pantry.get(ingredient).unwrap_or(&0) / amount,
        )
    }

    let mut pantry = baking_data.pantry;

    // TODO possibly refactor this with a mapping function or something similar
    for (ingrdient, amount) in pantry.iter_mut() {
        if let Some(needed) = baking_data.recipe.get(ingrdient) {
            *amount -= cookie_count * needed;
        }
    }

    let res = json!({
        "cookies": cookie_count,
        "pantry": pantry
    });

    Ok(HttpResponse::Ok().json(res))
}

#[derive(Deserialize, Serialize)]
struct PokeData {
    weight: i32,
}

async fn get_poke_weight(id: usize) -> Result<i32, ServerError> {
    match reqwest::get(format!("https://pokeapi.co/api/v2/pokemon/{id}/")).await {
        Ok(data) => match data.json::<PokeData>().await {
            Ok(body) => return Ok(body.weight),
            Err(_) => return Err(ServerError::InternalError), // maybe add a different error here
        },
        Err(_) => return Err(ServerError::InternalError),
    }
}

#[get("/8/weight/{id}")]
async fn poke_weigth(path: web::Path<usize>) -> Result<HttpResponse, ServerError> {
    let id = path.into_inner();

    match get_poke_weight(id).await {
        Ok(weight) => Ok(HttpResponse::Ok()
            .body(((weight as f32) / 10.0/* convert hectograms to kg */).to_string())),
        Err(_) => Err(ServerError::InternalError),
    }
}

#[get("/8/drop/{id}")]
async fn poke_drop(path: web::Path<usize>) -> Result<HttpResponse, ServerError> {
    const G: f32 = 9.825;
    const HEIGHT: f32 = 10.0;

    let id = path.into_inner();
    let velocity = f32::sqrt(2.0 * HEIGHT * G); // We calculate

    match get_poke_weight(id).await {
        Ok(weight) => Ok(HttpResponse::Ok().body(
            (((weight as f32) / 10.0) * velocity/* convert hectograms to kg and apply velocity*/)
                .to_string(),
        )),
        Err(_) => Err(ServerError::InternalError),
    }
}

#[derive(Debug, MultipartForm)]
struct UploadForm {
    image: TempFile,
}

#[post("11/red_pixels")]
async fn red_pixels(
    MultipartForm(form): MultipartForm<UploadForm>,
) -> Result<HttpResponse, ServerError> {
    // A beautiful nest of match statements and error handling
    // Cant use the shorthand '?' because that can't be mapped to ServerError
    // TODO figure out why '?' can't map to ServerError
    let img = match ImageRader::open(form.image.file) {
        Ok(reader) => match reader.with_guessed_format() {
            Ok(file) => match file.decode() {
                Ok(image) => image,
                Err(_) => return Err(ServerError::InternalError),
            },
            Err(_) => return Err(ServerError::InternalError),
        },
        Err(_) => return Err(ServerError::InternalError),
    };

    // Have to take the image as rgb8, so we also have to cast
    // all of the numbers to usize to get around overflows
    let pixels = match img.as_rgb8() {
        Some(img) => img.pixels(),
        None => return Err(ServerError::InternalError),
    };

    let count = pixels
        .filter(|Rgb([red, green, blue])| {
            let r = *red as usize;
            let g = *green as usize;
            let b = *blue as usize;

            return r > g + b;
        })
        .count();

    Ok(HttpResponse::Ok().body(count.to_string()))
}

#[post("12/save/{string}")]
async fn set_time(
    path: web::Path<String>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, ServerError> {
    let id = path.into_inner();
    let mut log = data.log.lock().unwrap(); // match and handle the error
    let start = Instant::now();

    log.insert(id, start);

    Ok(HttpResponse::Ok().finish())
}

#[get("12/load/{string}")]
async fn get_elapsed(
    path: web::Path<String>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, ServerError> {
    let id = path.into_inner();
    let log = data.log.lock().unwrap(); // match and handle

    let elapsed = log.get(&id).unwrap().elapsed();

    Ok(HttpResponse::Ok().body(elapsed.as_secs().to_string()))
}

#[post("12/ulids")]
async fn parse_ulids(ulids: web::Json<Vec<Ulid>>) -> Result<HttpResponse, ServerError> {
    let res = ulids
        .into_inner()
        .iter()
        .rev()
        .map(|x| {
            // Replace with uuid feature on ulid crate
            let bytes = x.to_bytes();
            let uuid = Uuid::from_bytes(bytes);

            uuid.to_string()
        })
        .collect::<Vec<String>>();

    Ok(HttpResponse::Ok().json(res))
}

#[post("12/ulids/{weekday}")]
async fn count_ulids(
    path: web::Path<u32>,
    ulids: web::Json<Vec<Ulid>>,
) -> Result<HttpResponse, ServerError> /* replace this with a type */ {
    let weekday = path.into_inner();
    let mut christmas_c: usize = 0;
    let mut weekday_c: usize = 0;
    let mut in_future_c: usize = 0;
    let mut lsb_1_c: usize = 0;

    for ulid in ulids.into_inner() {
        let date: DateTime<Utc> = ulid.datetime().into();

        if date.month() == 12 && date.day() == 24 {
            christmas_c += 1;
        }

        if date.weekday().num_days_from_monday() == weekday {
            weekday_c += 1;
        }

        if Utc::now() < date {
            in_future_c += 1;
        }

        if ulid.to_bytes().last().unwrap() % 2 == 1 {
            lsb_1_c += 1;
        }
    }

    Ok(HttpResponse::Ok().json(json!({
        "christmas eve": christmas_c,
        "weekday": weekday_c,
        "in the future": in_future_c,
        "LSB is 1": lsb_1_c
    })))
}

#[get("13/sql")]
async fn test_sql(state: web::Data<AppState>) -> Result<HttpResponse, ServerError> {
    let test_run: i32 = sqlx::query_scalar("SELECT 20231213")
        .fetch_one(&state.pool)
        .await
        .map_err(|_ /*TOOD make the error enum accept text and pass internal error to the server error*/| ServerError::InternalError)?;

    Ok(HttpResponse::Ok().body(test_run.to_string()))
}

#[derive(Deserialize, Debug)]
struct Order {
    // Realistically, all i32 here shoul be usize or other unsigned types
    // but sqlx can bind unsigned
    id: i32,
    region_id: i32,
    gift_name: String,
    quantity: i32,
}

#[post("13/reset")]
async fn reset_orders(state: web::Data<AppState>) -> Result<HttpResponse, ServerError> {
    match sqlx::query("DELETE FROM orders;")
        .execute(&state.pool)
        .await
    {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(_) => Err(ServerError::InternalError),
    }
}

#[post("13/orders")]
async fn insert_orders(
    state: web::Data<AppState>,
    body: web::Json<Vec<Order>>,
) -> Result<HttpResponse, ServerError> {
    let orders = body.into_inner();

    let mut transaction = state.pool.begin().await.unwrap(); // handle error
    for order in orders {
        sqlx::query(
            "INSERT INTO orders (id, region_id, gift_name, quantity)
            VALUES ($1, $2, $3, $4);",
        )
        .bind(order.id)
        .bind(order.region_id)
        .bind(order.gift_name)
        .bind(order.quantity)
        .execute(transaction.as_mut())
        .await
        .unwrap();
    }
    transaction.commit().await.unwrap();

    Ok(HttpResponse::Ok().finish())
}

#[get("13/orders/total")]
async fn get_total(state: web::Data<AppState>) -> Result<HttpResponse, ServerError> {
    match sqlx::query_scalar::<sqlx::Postgres, i64>("SELECT SUM(quantity) FROM orders;")
        .fetch_one(&state.pool)
        .await
    {
        Ok(sum) => Ok(HttpResponse::Ok().json(json!({"total": sum}))),
        Err(_) => Err(ServerError::InternalError),
    }
}

#[get("13/orders/popular")]
async fn get_popular(state: web::Data<AppState>) -> Result<HttpResponse, ServerError> {
    match sqlx::query_scalar::<sqlx::Postgres, String>("SELECT gift_name FROM public.orders GROUP BY gift_name ORDER BY SUM(quantity) DESC LIMIT 1").fetch_optional(&state.pool).await {
        Ok(name) => Ok(HttpResponse::Ok().json(json!({"popular": name}))),
        Err(_) => Err(ServerError::InternalError)
    }
}

#[derive(Serialize, Deserialize)]
struct TemplateContext {
    content: String,
}

static TEMPLATE: &'static str = "\
<html>
  <head>
    <title>CCH23 Day 14</title>
  </head>
  <body>
    {content}
  </body>
</html>";

#[post("14/unsafe")]
async fn render_unsafe(body: web::Json<TemplateContext>) -> Result<HttpResponse, ServerError> {
    let context = body.into_inner();
    let mut tt = TinyTemplate::new();
    tt.set_default_formatter(&tinytemplate::format_unescaped);
    
    match tt.add_template("unsafe", TEMPLATE) {
        Err(_) => return Err(ServerError::InternalError),
        _ => (),
    }

    let rendered = match tt.render("unsafe", &context) {
        Ok(body) => body,
        Err(_) => return Err(ServerError::InternalError)
    };

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(rendered))
}

#[post("14/safe")]
async fn render_safe(body: web::Json<TemplateContext>) -> Result<HttpResponse, ServerError> {
    let context = body.into_inner();
    let mut tt = TinyTemplate::new();
    
    match tt.add_template("safe", TEMPLATE) {
        Err(_) => return Err(ServerError::InternalError),
        _ => (),
    }

    let rendered = match tt.render("safe", &context) {
        Ok(body) => body,
        Err(_) => return Err(ServerError::InternalError)
    };

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(rendered))
}
