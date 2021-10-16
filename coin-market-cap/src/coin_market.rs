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
use serde::{Deserialize, Serialize};
use thiserror::Error;

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
    /// Latest average trade price across markets.
    price: Decimal,
    /// A measure of how much of a cryptocurrency was traded in the last 24 hours.
    volume_24h: Decimal,
    volume_change_24h: Decimal,
    /// 1 hour trading price percentage change for each currency.
    percent_change_1h: Decimal,
    /// 24 hour trading price percentage change for each currency.
    percent_change_24h: Decimal,
    /// 7 day trading price percentage change for each currency.
    percent_change_7d: Decimal,
    percent_change_30d: Decimal,
    percent_change_60d: Decimal,
    percent_change_90d: Decimal,
    /// The total market value of a cryptocurrency's circulating supply. It is analogous to the
    /// free-float capitalization in the stock market.
    ///
    /// `Market Cap = Current Price x Circulating Supply`
    ///
    /// (see [details](https://coinmarketcap.com/methodology/))
    market_cap: Decimal,
    market_cap_dominance: Decimal,
    /// The market cap if the max supply was in circulation.
    ///
    /// Fully-diluted market cap `(FDMC) = price x max supply`. If max supply is null, `FDMC =
    /// price x total supply`. If max supply and total supply are infinite or not available,
    /// fully-diluted market cap shows `- -`.
    fully_diluted_market_cap: Decimal,
    last_updated: DateTime<Utc>,
}

#[derive(Error, Debug)]
pub enum CmcError {
    #[error("Issues loading configuration")]
    LoadConfig(#[from] config::ConfigError),
    #[error("Issues during the request to the server")]
    Request(#[from] reqwest::Error),
}

/// Module that consumes the endpoint `/v1/cryptocurrency/map`. The latter returns a mapping of all
/// cryptocurrencies to unique CoinMarketCap `id`s. Each cryptocurrency returned includes typical
/// identifiers such as `name`, `symbol`, and `token_address` for flexible mapping to `id`.
/// By default this endpoint returns cryptocurrencies that have actively tracked markets on
/// supported exchanges. You may receive a map of all inactive cryptocurrencies by passing
/// `listing_status=inactive`.
pub mod map {
    use super::{CmcError, Platform, Status};
    use crate::configuration;
    use chrono::prelude::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Response {
        data: Vec<Data>,
        status: Status,
    }
    #[derive(Debug, Serialize, Deserialize)]
    struct Data {
        id: u32,
        name: String,
        symbol: String,
        slug: String,
        rank: u32,
        is_active: bool,
        first_historical_data: DateTime<Utc>,
        last_historical_data: DateTime<Utc>,
        platform: Option<Platform>,
    }

    /// Makes a request to the endpoint `/v1/cryptocurrency/map` of the CoinMarketCap API.
    /// Returns a mapping of all cryptocurrencies to unique CoinMarketCap `id`s.
    /// By default this endpoint returns cryptocurrencies that have actively tracked markets on
    /// supported exchanges. You may receive a map of all inactive cryptocurrencies by passing
    /// `listing_status=inactive`.
    pub async fn request_crypto_map() -> Result<Response, CmcError> {
        let config = configuration::load_config()?;

        // Pull new data from the server
        let client = reqwest::Client::new();
        let response: Response = client
            .get(config.coin_market.base_url + "/v1/cryptocurrency/map")
            .header("X-CMC_PRO_API_KEY", config.coin_market.api_key)
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }
}

/// Module that consumes the endpoint `/v1/cryptocurrency/listings_latest`. The latter returns a
/// paginated list of all active cryptocurrencies with latest market data. The default `market_cap`
/// sort returns cryptocurrency in order of CoinMarketCap's market cap rank but you may configure
/// this call to order by another market ranking field. 
pub mod listings_latest {
    use crate::configuration;
    use chrono::prelude::*;
    use rust_decimal::Decimal;
    use serde::{Deserialize, Serialize};

    use super::{CmcError, Platform, Status, Usd};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Response {
        data: Vec<Data>,
        status: Status,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Data {
        /// The CoinMarketCap's `id`.
        id: u32,
        /// The cryptocurrency name.
        name: String,
        /// The cryptocurrency symbol.
        ///
        /// **Remark:** `symbol` is not unique! Prefer CoinMarketCap's `id` as key.
        symbol: String,
        slug: String,
        /// Number of market pairs across all exchanges trading each currency.
        num_market_pairs: u32,
        date_added: DateTime<Utc>,
        tags: Vec<String>,
        /// Approximation of the maximum amount of coins that will ever exist in the lifetime
        /// of the currency.
        max_supply: Option<Decimal>,
        /// The amount of coins that are circulating in the market and are in public hands. It is
        /// analogous to the flowing shares in the stock market.
        circulating_supply: Decimal,
        /// Approximate total amount of coins in existence right now (minus any coins that have been
        /// verifiably burned).
        total_supply: Decimal,
        platform: Option<Platform>,
        /// CoinMarketCap's market cap rank as outlined in [their methodology](https://coinmarketcap.com/methodology/).
        /// Cryptocurrencies are listed by `cmc_rank` by default.
        cmc_rank: u32,
        last_updated: DateTime<Utc>,
        quote: Usd,
    }

    /// Makes a request to the endpoint `/v1/cryptocurrency/listings/latest` of the CoinMarketCap API.
    /// Returns a paginated list of all active cryptocurrencies with latest market data. The default
    /// `market_cap` sort returns cryptocurrency in order of CoinMarketCap's market cap rank.
    pub async fn request_cryto_listings_latest(
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
