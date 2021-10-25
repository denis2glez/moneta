use coin_market_cap::{
    coin_market::{listing, map, CmcError},
    configuration,
    database::*,
};

#[tokio::test]
async fn update_crypto_map_db_1() -> Result<(), CmcError> {
    let str_json = include_str!("data/cryptocurrency_map_50.json");
    // Read the JSON contents of the string as an instance of `map::Response`.
    let response: map::Response = serde_json::from_str(str_json).expect("Failed to parse input!");

    assert!(
        response.data.len() == 6517,
        "Error parsing `cryptocurrency_map.json` (wrong data number)"
    );

    let config = configuration::load_config()?;
    let pool = get_connection_pool(&config.database);

    clear_all_tables(pool.clone()).await?;
    update_crypto_map(response, pool).await?;

    Ok(())
}

#[tokio::test]
async fn update_crypto_listing_db_1() -> Result<(), CmcError> {
    let str_json = include_str!("data/cryptocurrency_listings_latest_50.json");
    // Read the JSON contents of the string as an instance of `listing::Response`.
    let response: listing::Response =
        serde_json::from_str(str_json).expect("Failed to parse input!");

    assert!(
        response.data.len() == 100,
        "Error parsing `cryptocurrency_listing_1.json` (wrong data number)"
    );

    let config = configuration::load_config()?;
    let pool = get_connection_pool(&config.database);

    clear_all_tables(pool.clone()).await?;
    // First calls previous test function
    update_crypto_map_db_1()?;
    update_crypto_listing(response, pool).await?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn update_crypto_populate_db_100_items() -> Result<(), CmcError> {
    let map_json = include_str!("data/cryptocurrency_map_100_ranked.json");
    let listings_json = include_str!("data/cryptocurrency_listings_latest_100.json");
    // Read the JSON contents of the string as an instance of `listing::Response`.
    let response_map: map::Response =
        serde_json::from_str(map_json).expect("Failed to parse input!");
    assert!(
        response_map.data.len() == 100,
        "Error parsing `cryptocurrency_map_100.json` (wrong data number)"
    );

    let response_listings: listing::Response =
        serde_json::from_str(listings_json).expect("Failed to parse input!");
    assert!(
        response_listings.data.len() == 100,
        "Error parsing `cryptocurrency_listing_100.json` (wrong data number)"
    );

    let config = configuration::load_config()?;
    let pool = get_connection_pool(&config.database);

    clear_all_tables(pool.clone()).await?;
    update_crypto_map(response_map, pool.clone()).await?;
    update_crypto_listing(response_listings, pool.clone()).await?;

    Ok(())
}
