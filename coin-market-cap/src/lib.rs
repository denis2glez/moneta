use chrono::prelude::*;
use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::BufReader};

pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(Debug, Serialize, Deserialize)]
struct CoinMarketResponse {
    data: Vec<Data>,
    status: Status,
}

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    id: u32,
    name: String,
    symbol: String,
    slug: String,
    num_market_pairs: u32,
    date_added: DateTime<Utc>,
    tags: Vec<String>,

    max_supply: Option<Decimal>,
    circulating_supply: Decimal,
    total_supply: Decimal,
    platform: Option<Platform>,
    cmc_rank: u32,
    last_updated: DateTime<Utc>,
    quote: Usd,
}

#[derive(Debug, Serialize, Deserialize)]
struct Status {
    timestamp: DateTime<Utc>,
    error_code: u32,
    error_message: Option<String>,
    elapsed: u32,
    credit_count: u32,
    notice: Option<u32>,
    total_count: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Platform {
    id: u32,
    name: String,
    symbol: String,
    slug: String,
    token_address: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
struct Usd {
    usd: Changes,
}

#[derive(Debug, Serialize, Deserialize)]
struct Changes {
    price: Decimal,
    volume_24h: Decimal,
    percent_change_1h: Decimal,
    percent_change_24h: Decimal,
    percent_change_7d: Decimal,
    percent_change_30d: Decimal,
    percent_change_60d: Decimal,
    percent_change_90d: Decimal,
    /// CoinMarketCap's market cap rank as outlined in their methodology.
    market_cap: Decimal,
    market_cap_dominance: Decimal,
    fully_diluted_market_cap: Decimal,
    last_updated: DateTime<Utc>,
}

pub async fn run() -> Result<(), Error> {
    // Open the file in read-only mode with buffer.
    let file = File::open("data/cryptocurrency_listings_latest_1.json")?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `CoinMarketResponse`.
    let res: CoinMarketResponse = serde_json::from_reader(reader)?;

    println!("body = {:?}", res);

    Ok(())
}
