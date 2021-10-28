use coin_market_cap::configuration;

/// Loads the configuration and ensures that your API key is set.
#[tokio::test]
#[ignore]
async fn config_check_api_key() {
    let config = configuration::load_config().expect("Error loading the configuration!");

    assert_ne!(
        config.coin_market.api_key, "<secret-token>",
        "You must specify your API key in order to run any test in `fetch_crypto_data`"
    );
}

/// Loads the configuration and checks that all database settings are defined.
#[tokio::test]
async fn config_check_database() {
    let config = configuration::load_config().expect("Error loading the configuration!");
    assert!(
        !config.database.username.is_empty(),
        "The database username is empty!"
    );

    assert!(
        !config.database.password.is_empty(),
        "The database password is empty!"
    );

    assert!(config.database.port > 0, "The database port is zero!");

    assert!(
        !config.database.host.is_empty(),
        "The database host is empty!"
    );

    assert!(
        !config.database.database_name.is_empty(),
        "The database database_name is empty!"
    );
}

/// Loads the configuration and checks that all CoinMarketCap settings are defined.
#[tokio::test]
async fn config_check_coin_market() {
    let config = configuration::load_config().expect("Error loading the configuration!");

    assert!(
        !config.coin_market.base_url.is_empty(),
        "The CoinMarketCap `base_url` name is empty!"
    );
    assert_eq!(
        config.coin_market.base_url,
        "https://pro-api.coinmarketcap.com"
    );

    assert!(
        !config.coin_market.api_key.is_empty(),
        "The CoinMarketCap `api_key` is empty!"
    );
}
