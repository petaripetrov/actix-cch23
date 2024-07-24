use actix_files::Files;
use actix_web::{get, web, HttpResponse, Scope};

use crate::common::{EndpointRet, ServerError};

mod day_1;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;

#[get("/")]
async fn index_page() -> EndpointRet {
    Ok(HttpResponse::Ok().finish())
}

#[get("/-1/error")]
async fn error_page() -> EndpointRet {
    Err(ServerError::InternalError)
}

pub fn routes() -> Scope {
    web::scope("")
        .service(index_page) // maybe replace this with a page with links to the various tasks
        .service(error_page)
        .service(day_1::cube_bits)
        .service(day_4::strength)
        .service(day_4::contest)
        .service(day_5::names_list)
        .service(day_6::elf_on_shelf)
        .service(day_7::decode)
        .service(day_7::bake)
        .service(day_8::poke_weigth)
        .service(day_8::poke_drop)
        .service(Files::new("/11/assets", "assets"))
        .service(day_11::red_pixels)
        .service(day_12::set_time)
        .service(day_12::get_elapsed)
        .service(day_12::parse_ulids)
        .service(day_12::count_ulids)
        .service(day_13::test_sql)
        .service(day_13::reset_orders)
        .service(day_13::insert_orders)
        .service(day_13::get_total)
        .service(day_13::get_popular)
        .service(day_14::render_unsafe)
        .service(day_14::render_safe)
        .service(day_15::password_nice)
        .service(day_15::password_game)
}
