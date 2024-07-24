use actix_web::{get, web, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::common::{EndpointRet, ServerError};

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
async fn poke_weigth(path: web::Path<usize>) -> EndpointRet {
    let id = path.into_inner();

    match get_poke_weight(id).await {
        Ok(weight) => Ok(HttpResponse::Ok()
            .body(((weight as f32) / 10.0/* convert hectograms to kg */).to_string())),
        Err(_) => Err(ServerError::InternalError),
    }
}

#[get("/8/drop/{id}")]
async fn poke_drop(path: web::Path<usize>) -> EndpointRet {
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