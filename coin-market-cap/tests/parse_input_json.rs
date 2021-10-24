use coin_market_cap::coin_market::{listing, map};
use std::collections::BTreeSet;

#[test]
fn parse_crypto_listing_1() {
    let str_json = include_str!("data/cryptocurrency_listings_latest_1.json");
    // Read the JSON contents of the string as an instance of `listing::Response`.
    let response: listing::Response =
        serde_json::from_str(str_json).expect("Failed to parse input!");

    assert!(
        response.data.len() == 100,
        "Error parsing `cryptocurrency_listing_1.json`"
    );
}

#[test]
fn parse_crypto_listing_2() {
    let str_json = include_str!("data/cryptocurrency_listings_latest_2.json");
    // Read the JSON contents of the string as an instance of `listing::Response`.
    let response: listing::Response =
        serde_json::from_str(str_json).expect("Failed to parse input!");
    assert!(
        response.data.len() == 5000,
        "Error parsing `cryptocurrency_listing_1.json`"
    );
}

#[test]
fn parse_crypto_listing_ignored_fields() {
    let str_json = include_str!("data/cryptocurrency_listings_latest_ignored_fields.json");

    let j_des = &mut serde_json::Deserializer::from_str(str_json);

    // We will build a set of paths to the unused elements.
    let mut unused = BTreeSet::new();

    let response: listing::Response = serde_ignored::deserialize(j_des, |path| {
        unused.insert(path.to_string());
    })
    .expect("Failed to parse input!");

    assert!(
        response.data.len() == 2,
        "Error parsing `cryptocurrency_listing_ignored_fields.json`"
    );

    // These are the ignored keys.
    let mut expected = BTreeSet::new();
    expected.insert("status.ignored_field_1".to_string());
    expected.insert("data.0.ignored_field_2".to_string());
    expected.insert("data.1.quote.USD.ignored_field_3".to_string());
    assert_eq!(unused, expected);
}

#[test]
fn parse_crypto_listing_ignored_fields_fails() {
    let str_json = include_str!("data/cryptocurrency_listings_latest_3.json");

    let j_des = &mut serde_json::Deserializer::from_str(str_json);

    // We will build a set of paths to the unused elements.
    let mut unused = BTreeSet::new();

    let response: listing::Response = serde_ignored::deserialize(j_des, |path| {
        unused.insert(path.to_string());
    })
    .expect("Failed to parse input!");

    assert!(
        response.data.len() == 5000,
        "Error parsing `cryptocurrency_listing_3.json`"
    );
    assert!(unused.is_empty(), "The json file contains ignored fields!");
}

#[test]
fn parse_crypto_map_1() {
    let str_json = include_str!("data/cryptocurrency_map.json");
    // Read the JSON contents of the string as an instance of `map::Response`.
    let response: map::Response = serde_json::from_str(str_json).expect("Failed to parse input!");

    assert!(
        response.data.len() == 6517,
        "Error parsing `cryptocurrency_map.json`"
    );
}
