use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::coin_market::{map, CmcError};
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
                platform.id as i32, // cryto_map's base blockchain id
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

pub async fn startup() -> Result<(), CmcError> {
    let config = configuration::load_config()?;
    let pool = get_connection_pool(&config.database);

    let response = map::request_crypto_map().await?;
    update_crypto_map(response, pool).await?;
    Ok(())
}

pub fn get_connection_pool(config: &DbConfig) -> PgPool {
    PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(config.with_db())
}
