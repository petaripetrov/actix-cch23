use actix_web::{post, web, HttpResponse};

use crate::common::EndpointRet;

#[derive(serde::Deserialize)]
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
) -> EndpointRet {
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