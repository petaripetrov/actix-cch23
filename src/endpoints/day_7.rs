use std::collections::HashMap;

use actix_web::{get, HttpRequest, HttpResponse};
use base64::{engine::general_purpose, Engine};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::common::{EndpointRet, ServerError};

#[get("/7/decode")]
async fn decode(req: HttpRequest) -> EndpointRet {
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
async fn bake(req: HttpRequest) -> EndpointRet {
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
