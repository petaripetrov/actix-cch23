use actix_web::web::ServiceConfig;
use shuttle_actix_web::ShuttleActixWeb;

mod endpoints;

#[shuttle_runtime::main]
async fn actix_web() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(endpoints::index_page); // maybe replace this with a page with links to the various tasks
        cfg.service(endpoints::error_page);
        cfg.service(endpoints::cube_bits);
        cfg.service(endpoints::strength);
        cfg.service(endpoints::contest);
        cfg.service(endpoints::names_list);
        cfg.service(endpoints::elf_on_shelf);
    };

    Ok(config.into())
}
