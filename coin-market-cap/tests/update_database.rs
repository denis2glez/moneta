use coin_market_cap::{
    coin_market::{listings_latest, map, CmcError},
    configuration,
    database::*,
};

#[tokio::test]
async fn cryptocurrency_map_update_db_1() -> Result<(), CmcError> {
    let str_json = include_str!("data/cryptocurrency_map.json");
    // Read the JSON contents of the string as an instance of `map::Response`.
    let response: map::Response = serde_json::from_str(str_json).expect("Failed to parse input!");

    let config = configuration::load_config()?;

    let pool = get_connection_pool(&config.database);

    update_crypto_map(response, pool).await?;

    Ok(())
}

#[tokio::test]
async fn cryptocurrency_listings_latest_update_db_1() -> Result<(), CmcError> {
    let str_json = include_str!("data/cryptocurrency_listings_latest_1.json");
    // Read the JSON contents of the string as an instance of `listings_latest::Response`.
    let response: listings_latest::Response =
        serde_json::from_str(str_json).expect("Failed to parse input!");

    let config = configuration::load_config()?;

    let pool = get_connection_pool(&config.database);

    update_crypto_listings_latest(response, pool).await?;

    Ok(())
}
