mod endpoints;

use std::{collections::HashMap, sync::Mutex};

use actix_files::Files;
use actix_web::web::{self, ServiceConfig};
use shuttle_actix_web::ShuttleActixWeb;
use endpoints::AppState;
use sqlx::PgPool;

#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_shared_db::Postgres(
        local_uri = "postgres://postgres:{secrets.PASSWORD}@localhost:5432/postgres"
    )] pool: PgPool,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations!");

    // Prevents double arc
    let state = web::Data::new(AppState {
        log: Mutex::new(HashMap::new()),
        pool
    });

    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(
            web::scope("")
                .app_data(state)
                .service(endpoints::index_page) // maybe replace this with a page with links to the various tasks
                .service(endpoints::error_page)
                .service(endpoints::cube_bits)
                .service(endpoints::strength)
                .service(endpoints::contest)
                .service(endpoints::names_list)
                .service(endpoints::elf_on_shelf)
                .service(endpoints::decode)
                .service(endpoints::bake)
                .service(endpoints::poke_weigth)
                .service(endpoints::poke_drop)
                .service(Files::new("/11/assets", "assets"))
                .service(endpoints::red_pixels)
                .service(endpoints::set_time)
                .service(endpoints::get_elapsed)
                .service(endpoints::parse_ulids)
                .service(endpoints::count_ulids)
                .service(endpoints::test_sql)
                .service(endpoints::reset_orders)
                .service(endpoints::insert_orders)
                .service(endpoints::get_total)
                .service(endpoints::get_popular)
                .service(endpoints::render_unsafe)
                .service(endpoints::render_safe)
        );
    };

    Ok(config.into())
}
