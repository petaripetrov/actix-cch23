use actix_web::{get, web, HttpResponse};

use crate::common::EndpointRet;

#[get("/1/{ids:.*}")]
async fn cube_bits(path: web::Path<String>) -> EndpointRet {
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