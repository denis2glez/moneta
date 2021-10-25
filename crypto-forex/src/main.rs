#[actix_web::main]
async fn main() -> std::io::Result<()> {
    crypto_forex::route::run().await
}
