use coin_market_cap::*;

#[tokio::main]
async fn main() -> Result<(), coin_market::Error> {
    request_data().await
}
