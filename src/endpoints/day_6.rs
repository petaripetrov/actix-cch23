use actix_web::{post, HttpResponse};
use serde_json::json;
use crate::common::EndpointRet;

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
async fn elf_on_shelf(text: String) -> EndpointRet {
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