use actix_web::{HttpRequest, Responder};

use crate::{config, database};

pub async fn show_crypto_data(req: HttpRequest) -> impl Responder {
    let rank: usize = req.match_info().get("rank").unwrap().parse().unwrap();
    log::info!("Crypto with rank {}!", rank);

    let config = config::load_config().unwrap();
    let pool = database::get_connection_pool(&config.database);
    let listing = database::get_crypto_map(pool).await.unwrap();
    format!("Crypto with rank {}:\n\n{:#?}", rank, listing[rank - 1])
}
