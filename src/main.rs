use actix_web::web::{self, ServiceConfig};
use shuttle_actix_web::ShuttleActixWeb;

mod endpoints;

#[shuttle_runtime::main]
async fn actix_web() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(
            web::scope("")
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
        );
    };

    Ok(config.into())
}
