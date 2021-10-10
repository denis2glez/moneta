#[actix_web::main]
async fn main() -> std::io::Result<()> {
    cryto_forex::run().await
}
