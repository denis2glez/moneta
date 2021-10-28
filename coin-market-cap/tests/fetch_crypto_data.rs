use coin_market_cap::coin_market;

/// Be aware that this test **makes a real request** to the endpoint of the CoinMarketCap API.
#[tokio::test]
#[ignore]
async fn fetch_crypto_listing() {
    // Pull new data from the server
    match coin_market::listing::request_crypto_listing(1, 100, "USD").await {
        Ok(response) => {
            assert!(
                response.data.len() == 100,
                "Error parsing the response from the server"
            );
        }
        Err(error) => panic!("Error calling `request_crypto_listing`: {}", error),
    }
}

/// Be aware that this test **makes a real request** to the endpoint of the CoinMarketCap API.
#[tokio::test]
#[ignore]
async fn fetch_crypto_map() {
    // Pull new data from the server
    match coin_market::map::request_crypto_map(1, 100, "cmc_rank").await {
        Ok(response) => {
            assert!(
                response.data.len() == 100,
                "Error parsing the response from the server"
            );
        }
        Err(error) => panic!("Error calling `request_crypto_map`: {}", error),
    }
}
