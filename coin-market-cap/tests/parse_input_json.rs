use std::collections::BTreeSet;
use coin_market_cap::coin_market::CoinMarketResponse;

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

#[test]
fn cryptocurrency_listings_latest_ignored_fields() {
    let str_json = include_str!("data/cryptocurrency_listings_latest_ignored_fields.json");

    let j_des = &mut serde_json::Deserializer::from_str(str_json);

    // We will build a set of paths to the unused elements.
    let mut unused = BTreeSet::new();

    let _response: CoinMarketResponse = serde_ignored::deserialize(j_des, |path| {
        unused.insert(path.to_string());
    })
    .expect("Failed to parse input!");

    // These are the ignored keys.
    let mut expected = BTreeSet::new();
    expected.insert("status.ignored_field_1".to_string());
    expected.insert("data.0.ignored_field_2".to_string());
    expected.insert("data.1.quote.USD.ignored_field_3".to_string());
    assert_eq!(unused, expected);
}

#[test]
fn cryptocurrency_listings_latest_ignored_fields_fails() {
    let str_json = include_str!("data/cryptocurrency_listings_latest_3.json");

    let j_des = &mut serde_json::Deserializer::from_str(str_json);

    // We will build a set of paths to the unused elements.
    let mut unused = BTreeSet::new();

    let _response: CoinMarketResponse = serde_ignored::deserialize(j_des, |path| {
        unused.insert(path.to_string());
    })
    .expect("Failed to parse input!");

    assert!(unused.is_empty(), "The json file contains ignored fields!");
}
