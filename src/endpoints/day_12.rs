use std::time::Instant;

use actix_web::{get, post, web, HttpResponse};
use chrono::{DateTime, Datelike, Utc};
use serde_json::json;
use ulid::Ulid;
use uuid::Uuid;

use crate::common::{AppState, EndpointRet};

#[post("12/save/{string}")]
async fn set_time(path: web::Path<String>, data: web::Data<AppState>) -> EndpointRet {
    let id = path.into_inner();
    let mut log = data.log.lock().unwrap(); // match and handle the error
    let start = Instant::now();

    log.insert(id, start);

    Ok(HttpResponse::Ok().finish())
}

#[get("12/load/{string}")]
async fn get_elapsed(path: web::Path<String>, data: web::Data<AppState>) -> EndpointRet {
    let id = path.into_inner();
    let log = data.log.lock().unwrap(); // match and handle

    let elapsed = log.get(&id).unwrap().elapsed();

    Ok(HttpResponse::Ok().body(elapsed.as_secs().to_string()))
}

#[post("12/ulids")]
async fn parse_ulids(ulids: web::Json<Vec<Ulid>>) -> EndpointRet {
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
async fn count_ulids(path: web::Path<u32>, ulids: web::Json<Vec<Ulid>>) -> EndpointRet /* replace this with a type */
{
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