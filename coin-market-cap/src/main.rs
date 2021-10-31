use coin_market_cap::coin_market::{App, CmcError};
use std::time::Duration;

// Use `jemalloc` only for musl-64 bits platforms
#[cfg(all(target_env = "musl", target_pointer_width = "64"))]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

/// Entry point of the `coin-market-cap` application, which is responsible for periodically fetching
/// information about cryptocurrencies from the CoinMarketCap API and then caching it locally using
/// a migrated PostgreSQL database.

// A more correct return type would be `Result<!, CmcError>`, but the *never* type is still
// experimental.
#[tokio::main]
async fn main() -> Result<(), CmcError> {
    let duration = Duration::from_secs(5*60);
    let app = App::new();

    loop {
        app.fetch_crypto_data(1, 100, "USD").await?;
        tokio::time::sleep(duration).await;
    }
}
