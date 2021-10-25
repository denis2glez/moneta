//! Module that retrieves information from the cryptocurrencies database. Fundamentally, it contains
//! the `CryptoPlatform` struct that represents a cryptocurrency platform used by other, the
//! `CryptoMap` struct that describes all the characteristics of a specific cryptocurrency and the
//! `CryptoListing` struct provides accurate and timely data for the cryptoasset.

use chrono::prelude::*;
use rust_decimal::prelude::*;
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct CryptoPlatform {
    pub id: i32,
    pub platform: i32,
    pub token_address: String,
}

#[derive(Debug, FromRow)]
pub struct CryptoMap {
    pub id: i32,
    pub name: String,
    pub symbol: String,
    pub slug: String,
    pub rank: i32,
    pub is_active: bool,
    pub first_historical_data: DateTime<Utc>,
    pub last_historical_data: DateTime<Utc>,
    pub platform: Option<i32>,
}

#[derive(Debug, FromRow)]
pub struct CryptoListing {
    /// The CoinMarketCap's `id`.
    pub id: i32,
    /// Number of market pairs across all exchanges trading each currency.
    pub num_market_pairs: i32,
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
    pub platform: i32,
    /// CoinMarketCap's market cap rank as outlined in [their methodology](https://coinmarketcap.com/methodology/).
    /// Cryptocurrencies are listed by `cmc_rank` by default.
    pub cmc_rank: i32,
    pub quote: String,

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
