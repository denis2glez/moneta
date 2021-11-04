use crypto_forex::config;
use crypto_forex::route::{self, CfxError};

/// Entry point of the `crypto_forex` application responsible for serving information about
/// cryptocurrencies through a Restful API consisting of the following endpoints:
/// - `/crypto/map`
/// - `/crypto/listing`

// A more correct return type would be `Result<!, CfxError>`, but the *never* type is still
// experimental.
#[actix_web::main]
async fn main() -> Result<(), CfxError> {
    let config = config::load_config()?;
    let server = route::CfxServer::build(config).await?;
    server.run().await?;

    Ok(())
}
