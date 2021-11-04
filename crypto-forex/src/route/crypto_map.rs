use crate::database;
use actix_web::{web, Responder};
use sqlx::PgPool;

#[derive(serde::Deserialize)]
pub struct Params {
    cmc_id: usize,
}

pub async fn map(query: web::Query<Params>, pool: web::Data<PgPool>) -> impl Responder {
    let id = query.cmc_id;
    log::info!("Crypto with id {}!", id);

    let map = database::get_crypto_map(&pool).await.unwrap();
    format!("Crypto with id {}:\n\n{:#?}", id, map[id - 1])
}
