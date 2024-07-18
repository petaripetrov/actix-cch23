use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{
    error, get,
    http::{header::ContentType, StatusCode},
    post, web, HttpRequest, HttpResponse,
};
use base64::{engine::general_purpose, Engine};
use derive_more::{Display, Error};
use image::{io::Reader as ImageRader, Rgb};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

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

// #[derive(Deserialize, Serialize)]
// struct CookieData {
//     flour: usize,
//     sugar: usize,
//     butter: usize,
//     #[serde(rename(deserialize = "baking powder", serialize = "baking powder"))]
//     baking_powder: usize,
//     #[serde(rename(deserialize = "chocolate chips", serialize = "chocolate chips"))]
//     chocolate_chips: usize,
// }

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
