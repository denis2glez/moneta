use coin_market_cap::CoinMarketResponse;

#[test]
fn cryptocurrency_listings_latest_1() {
    let str_json = include_str!("data/cryptocurrency_listings_latest_1.json");
    // Read the JSON contents of the string as an instance of `CoinMarketResponse`.
    let _response: CoinMarketResponse =
        serde_json::from_str(str_json).expect("Failed to parse input!");
}

#[test]
fn cryptocurrency_listings_latest_2() {
    let str_json = include_str!("data/cryptocurrency_listings_latest_2.json");
    // Read the JSON contents of the string as an instance of `CoinMarketResponse`.
    let _response: CoinMarketResponse =
        serde_json::from_str(str_json).expect("Failed to parse input!");
}
