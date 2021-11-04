use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::config::DbConfig;
use crate::model::*;

/// Returns all rows in table `crypto_map` as a `Vec<CryptoMap>`.
pub async fn get_crypto_map(pool: &PgPool) -> Result<Vec<CryptoMap>, sqlx::Error> {
    Ok(sqlx::query_as("SELECT * FROM crypto_map;")
        .fetch_all(pool)
        .await?)
}

/// Returns all rows in table `crypto_listing` as a `Vec<CryptoListing>`.
pub async fn get_crypto_listing(pool: &PgPool) -> Result<Vec<CryptoListing>, sqlx::Error> {
    Ok(sqlx::query_as("SELECT * FROM crypto_listing;")
        .fetch_all(pool)
        .await?)
}

/// Returns all rows in table `crypto_platform` as a `Vec<CryptoPlatform>`.
pub async fn get_crypto_platform(pool: &PgPool) -> Result<Vec<CryptoPlatform>, sqlx::Error> {
    Ok(sqlx::query_as("SELECT * FROM crypto_platform;")
        .fetch_all(pool)
        .await?)
}

pub fn get_connection_pool(config: &DbConfig) -> PgPool {
    PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(config.with_db())
}
