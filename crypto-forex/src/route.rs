use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

use crate::{config, database};

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn show_crypto_data(req: HttpRequest) -> impl Responder {
    let rank: usize = req.match_info().get("rank").unwrap().parse().unwrap();
    log::info!("Crypto with rank {}!", rank);

    let config = config::load_config().unwrap();
    let pool = database::get_connection_pool(&config.database);
    let listing = database::get_crypto_map(pool).await.unwrap();
    format!("Crypto with rank {}:\n\n{:#?}", rank, listing[rank - 1])
}

pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/{rank}", web::get().to(show_crypto_data))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
