//! Module that fetches information from the [CoinMarketCap API](https://coinmarketcap.com/api/documentation/v1/).
//! It contains the following submodules:
//! - `map` that consumes the endpoint `/v1/cryptocurrency/map`  
//! - `listings/latest` that consumes the endpoint `/v1/cryptocurrency/listings/latest`
//!
//! **Remark:** Many cryptocurrencies have the same symbol, for example, there are currently three
//! cryptocurrencies that commonly refer to themselves by the symbol `HOT`. Moreover, cryptocurrency
//! symbols also often change with cryptocurrency rebrands.

use chrono::prelude::*;
use rust_decimal::prelude::*;
use serde::Deserialize;
use sqlx::PgPool;
use thiserror::Error;

use crate::{configuration, database};

#[derive(Debug, Deserialize)]
pub struct Platform {
    pub id: u32,
    pub name: String,
    pub symbol: String,
    pub slug: String,
    pub token_address: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct Usd {
    pub usd: Changes,
}

#[derive(Debug, Deserialize)]
pub struct Changes {
    /// Latest average trade price across markets.
    pub price: Decimal,
    /// A measure of how much of a cryptocurrency was traded in the last 24 hours.
    pub volume_24h: Decimal,
    pub volume_change_24h: Decimal,
    /// 1 hour trading price percentage change for each currency.
    pub percent_change_1h: Decimal,
    /// 24 hour trading price percentage change for each currency.
    pub percent_change_24h: Decimal,
    /// 7 day trading price percentage change for each currency.
    pub percent_change_7d: Decimal,
    pub percent_change_30d: Decimal,
    pub percent_change_60d: Decimal,
    pub percent_change_90d: Decimal,
    /// The total market value of a cryptocurrency's circulating supply. It is analogous to the
    /// free-float capitalization in the stock market.
    ///
    /// `Market Cap = Current Price x Circulating Supply`
    ///
    /// (see [details](https://coinmarketcap.com/methodology/))
    pub market_cap: Decimal,
    pub market_cap_dominance: Decimal,
    /// The market cap if the max supply was in circulation.
    ///
    /// Fully-diluted market cap `(FDMC) = price x max supply`. If max supply is null, `FDMC =
    /// price x total supply`. If max supply and total supply are infinite or not available,
    /// fully-diluted market cap shows `- -`.
    pub fully_diluted_market_cap: Decimal,
    pub last_updated: DateTime<Utc>,
}

#[derive(Error, Debug)]
pub enum CmcError {
    #[error("Issues loading configuration")]
    LoadConfig(#[from] config::ConfigError),
    #[error("Issues during the request to the server")]
    Request(#[from] reqwest::Error),
    #[error("Issues querying the database")]
    DbQuery(#[from] sqlx::Error),
}

/// Module that consumes the endpoint `/v1/cryptocurrency/map`. The latter returns a mapping of all
/// cryptocurrencies to unique CoinMarketCap `id`s. Each cryptocurrency returned includes typical
/// identifiers such as `name`, `symbol`, and `token_address` for flexible mapping to `id`.
/// By default this endpoint returns cryptocurrencies that have actively tracked markets on
/// supported exchanges. You may receive a map of all inactive cryptocurrencies by passing
/// `listing_status=inactive`.
pub mod map {
    use super::{CmcError, Platform};
    use crate::configuration;
    use chrono::prelude::*;
    use serde::{
        self,
        de::{self, Deserializer, Unexpected},
        Deserialize,
    };

    #[derive(Debug, Deserialize)]
    pub struct Response {
        pub data: Vec<Data>,
        pub status: Status,
    }

    #[allow(unused)]
    #[derive(Debug, Deserialize)]
    pub struct Status {
        timestamp: DateTime<Utc>,
        error_code: u32,
        error_message: Option<String>,
        elapsed: u32,
        credit_count: u32,
        notice: Option<u32>,
    }

    #[derive(Debug, Deserialize)]
    pub struct Data {
        pub id: u32,
        pub name: String,
        pub symbol: String,
        pub slug: String,
        pub rank: u32,
        #[serde(deserialize_with = "bool_from_int")]
        pub is_active: bool,
        pub first_historical_data: DateTime<Utc>,
        pub last_historical_data: DateTime<Utc>,
        pub platform: Option<Platform>,
    }

    fn bool_from_int<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: Deserializer<'de>,
    {
        match u8::deserialize(deserializer)? {
            0 => Ok(false),
            1 => Ok(true),
            other => Err(de::Error::invalid_value(
                Unexpected::Unsigned(other as u64),
                &"zero or one",
            )),
        }
    }

    /// Makes a request to the endpoint `/v1/cryptocurrency/map` of the CoinMarketCap API.
    /// Returns a mapping of all cryptocurrencies to unique CoinMarketCap `id`s.
    /// By default this endpoint returns cryptocurrencies that have actively tracked markets on
    /// supported exchanges. You may receive a map of all inactive cryptocurrencies by passing
    /// `listing_status=inactive`.
    pub async fn request_crypto_map(
        start: u32,
        limit: u32,
        sort: &str,
    ) -> Result<Response, CmcError> {
        let config = configuration::load_config()?;

        // Pull new data from the server
        let client = reqwest::Client::new();
        let params = [
            ("start", start.to_string()),
            ("limit", limit.to_string()),
            ("sort", sort.to_string()),
        ];

        let response: Response = client
            .get(config.coin_market.base_url + "/v1/cryptocurrency/map")
            .header("X-CMC_PRO_API_KEY", config.coin_market.api_key)
            .query(&params)
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }
}

/// Module that consumes the endpoint `/v1/cryptocurrency/listing`. The latter returns a
/// paginated list of all active cryptocurrencies with latest market data. The default `market_cap`
/// sort returns cryptocurrency in order of CoinMarketCap's market cap rank but you may configure
/// this call to order by another market ranking field.
pub mod listing {
    use crate::configuration;
    use chrono::prelude::*;
    use rust_decimal::Decimal;
    use serde::Deserialize;

    use super::{CmcError, Platform, Usd};

    #[derive(Debug, Deserialize)]
    pub struct Response {
        pub data: Vec<Data>,
        pub status: Status,
    }

    #[allow(unused)]
    #[derive(Debug, Deserialize)]
    pub struct Status {
        timestamp: DateTime<Utc>,
        error_code: u32,
        error_message: Option<String>,
        elapsed: u32,
        credit_count: u32,
        notice: Option<u32>,
        total_count: u32,
    }

    #[derive(Debug, Deserialize)]
    pub struct Data {
        /// The CoinMarketCap's `id`.
        pub id: u32,
        /// The cryptocurrency name.
        pub name: String,
        /// The cryptocurrency symbol.
        ///
        /// **Remark:** `symbol` is not unique! Prefer CoinMarketCap's `id` as key.
        pub symbol: String,
        pub slug: String,
        /// Number of market pairs across all exchanges trading each currency.
        pub num_market_pairs: u32,
        pub date_added: DateTime<Utc>,
        pub tags: Vec<String>,
        /// Approximation of the maximum amount of coins that will ever exist in the lifetime
        /// of the currency.
        pub max_supply: Option<Decimal>,
        /// The amount of coins that are circulating in the market and are in public hands. It is
        /// analogous to the flowing shares in the stock market.
        pub circulating_supply: Decimal,
        /// Approximate total amount of coins in existence right now (minus any coins that have been
        /// verifiably burned).
        pub total_supply: Decimal,
        pub platform: Option<Platform>,
        /// CoinMarketCap's market cap rank as outlined in [their methodology](https://coinmarketcap.com/methodology/).
        /// Cryptocurrencies are listed by `cmc_rank` by default.
        pub cmc_rank: u32,
        pub last_updated: DateTime<Utc>,
        pub quote: Usd,
    }

    /// Makes a request to the endpoint `/v1/cryptocurrency/listings/latest` of the CoinMarketCap API.
    /// Returns a paginated list of all active cryptocurrencies with latest market data. The default
    /// `market_cap` sort returns cryptocurrency in order of CoinMarketCap's market cap rank.
    pub async fn request_crypto_listing(
        start: u32,
        limit: u32,
        convert: &str,
    ) -> Result<Response, CmcError> {
        let config = configuration::load_config()?;

        // Pull new data from the server
        let client = reqwest::Client::new();
        let params = [
            ("start", start.to_string()),
            ("limit", limit.to_string()),
            ("convert", convert.to_string()),
        ];
        let response: Response = client
            .get(config.coin_market.base_url + "/v1/cryptocurrency/listings/latest")
            .header("X-CMC_PRO_API_KEY", config.coin_market.api_key)
            .query(&params)
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }
}

pub struct App {
    pool: PgPool,
}

impl App {
    pub fn new() -> Self {
        let config = configuration::load_config().unwrap();
        let pool = database::get_connection_pool(&config.database);
        Self { pool }
    }

    pub async fn fetch_crypto_data(
        &self,
        start: u32,
        limit: u32,
        convert: &str,
    ) -> Result<(), CmcError> {
        let response_map = map::request_crypto_map(start, 2 * limit, "cmc_rank").await?;
        let response_listing = listing::request_crypto_listing(start, limit, convert).await?;

        // TODO: Currently we repopulate all tables in each update, this would change to keep the
        // previous data.
        database::clear_all_tables(self.pool.clone()).await?;
        database::update_crypto_map(response_map, self.pool.clone()).await?;
        database::update_crypto_listing(response_listing, self.pool.clone()).await?;

        Ok(())
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
