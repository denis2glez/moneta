mod crypto_map;
mod health_check;

use actix_web::{web, App, HttpServer};
use crypto_map::show_crypto_data;
use health_check::health_check;

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
