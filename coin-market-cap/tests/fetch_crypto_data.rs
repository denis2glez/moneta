use coin_market_cap::{coin_market::CoinMarketResponse, configuration};

/// Be aware that this test **makes a real request** to the endpoint of the CoinMarketCap API.
#[tokio::test]
async fn fetch_crypto_listings_latest() {
    let config = configuration::load_config().expect("Error loading the configuration!");

    // Pull new data from the server
    let client = reqwest::Client::new();
    let params = [("start", "1"), ("limit", "5000"), ("convert", "USD")];
    let _response: CoinMarketResponse = client
        .get(config.coin_market.base_url + "/v1/cryptocurrency/listings/latest")
        .header("X-CMC_PRO_API_KEY", config.coin_market.api_key)
        .query(&params)
        .send()
        .await
        .expect("Error making the request to the server!")
        .json()
        .await
        .expect("Error receiving response from server!");
}
