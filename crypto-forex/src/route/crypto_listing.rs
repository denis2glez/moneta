use crate::database;
use actix_web::{web, Responder};
use sqlx::PgPool;

#[derive(serde::Deserialize)]
pub struct Params {
    cmc_rank: usize,
}

pub async fn listing(query: web::Query<Params>, pool: web::Data<PgPool>) -> impl Responder {
    let rank = query.cmc_rank;
    log::info!("Crypto with rank {}!", rank);

    let listing = database::get_crypto_listing(&pool).await.unwrap();
    format!("Crypto with rank {}:\n\n{:#?}", rank, listing[rank - 1])
}
