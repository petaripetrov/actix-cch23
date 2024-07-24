mod endpoints;
mod common;

use std::{collections::HashMap, sync::Mutex};

use actix_web::web::{self, ServiceConfig};
use common::AppState;
use shuttle_actix_web::ShuttleActixWeb;
use sqlx::PgPool;

#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_shared_db::Postgres(
        local_uri = "postgres://postgres:{secrets.PASSWORD}@localhost:5432/postgres"
    )]
    pool: PgPool,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations!");

    // Prevents double arc
    let state = web::Data::new(AppState {
        log: Mutex::new(HashMap::new()),
        pool,
    });

    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(endpoints::routes().app_data(state));
    };

    Ok(config.into())
}
