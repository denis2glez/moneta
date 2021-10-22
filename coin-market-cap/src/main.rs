use std::time::Duration;
use coin_market_cap::coin_market::*;

/// Entry point of the `coin-market-cap` application, which is responsible for periodically fetching
/// information about cryptocurrencies from the CoinMarketCap API and then caching it locally using
/// a migrated PostgreSQL database.

// A more correct return type would be `Result<!, CmcError>`, but the *never* type is still
// experimental.
#[tokio::main]
async fn main() -> Result<(), CmcError> {

    // loop {   
    //     fetch_crypto_data().await?;
    //     tokio::time::sleep(Duration::from_secs(60)).await;
    // }

    Ok(())
}
