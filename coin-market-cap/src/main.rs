use coin_market_cap::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    request_data().await
}
