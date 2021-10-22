use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::coin_market::{listings_latest, map, CmcError};
use crate::configuration::{self, DbConfig};

/// Update the databases `crypto_map` and `platforms` with data from `map::Response`.
// TODO Keep an eye on the development around `sqlx::FromRow`.
pub async fn update_crypto_map(response: map::Response, pool: PgPool) -> Result<(), sqlx::Error> {
    for data in &response.data {
        let mut platform_id = None;

        if let Some(platform) = &data.platform {
            platform_id = Some(data.id as i32);
            sqlx::query!(
                "INSERT INTO platforms VALUES ($1, $2, $3);",
                data.id as i32,     // crypto_map's derived blockchain id
                platform.id as i32, // crypto_map's base blockchain id
                platform.token_address,
            )
            .execute(&pool)
            .await?;
        }

        sqlx::query!(
            "INSERT INTO crypto_map VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9);",
            data.id as i32,
            data.name,
            data.symbol,
            data.slug,
            data.rank as i32,
            data.is_active,
            data.first_historical_data,
            data.last_historical_data,
            platform_id,
        )
        .execute(&pool)
        .await?;
    }
    Ok(())
}

/// Update the database `crypto_listings_latest` with data from `listings_latest::Response`.
pub async fn update_crypto_listings_latest(
    response: listings_latest::Response,
    pool: PgPool,
) -> Result<(), sqlx::Error> {
    for data in &response.data {
        let mut platform_id = None;
        if data.platform.is_some() {
            platform_id = Some(data.id as i32);
        }

        sqlx::query!(
        r#"INSERT INTO crypto_listings_latest VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11,
                                $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22);"#,
        data.id as i32,
        data.num_market_pairs as i32,
        &data.tags,
        data.max_supply,
        data.circulating_supply,
        data.total_supply,
        platform_id,
        data.cmc_rank as i32,
        "USD",  // data.quote,
        data.quote.usd.price,
        data.quote.usd.volume_24h,
        data.quote.usd.volume_change_24h,
        data.quote.usd.percent_change_1h,
        data.quote.usd.percent_change_24h,
        data.quote.usd.percent_change_7d,
        data.quote.usd.percent_change_30d,
        data.quote.usd.percent_change_60d,
        data.quote.usd.percent_change_90d,
        data.quote.usd.market_cap,
        data.quote.usd.market_cap_dominance,
        data.quote.usd.fully_diluted_market_cap,
        data.quote.usd.last_updated
        )
        .execute(&pool)
        .await?;
    }
    Ok(())
}

pub fn get_connection_pool(config: &DbConfig) -> PgPool {
    PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(config.with_db())
}
