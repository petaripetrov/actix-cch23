mod endpoints;

use std::{collections::HashMap, sync::Mutex};

use actix_files::Files;
use actix_web::web::{self, ServiceConfig};
use shuttle_actix_web::ShuttleActixWeb;
use endpoints::AppState;

#[shuttle_runtime::main]
async fn actix_web() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    
    // Prevents double arc
    let state = web::Data::new(AppState {
        log: Mutex::new(HashMap::new()),
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
        );
    };

    Ok(config.into())
}
