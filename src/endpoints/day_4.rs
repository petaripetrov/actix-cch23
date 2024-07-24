use actix_web::{post, web, HttpResponse};
use serde::Deserialize;
use serde_json::json;

use crate::common::EndpointRet;

// TODO move Speed and Deer into types
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

#[post("/4/strength")]
async fn strength(deer: web::Json<Vec<Deer>>) -> EndpointRet {
    let strength: i64 = deer.iter().map(|d| d.strength).sum();

    Ok(HttpResponse::Ok().body(strength.to_string()))
}

#[post("/4/contest")]
async fn contest(deer: web::Json<Vec<Deer>>) -> EndpointRet {
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