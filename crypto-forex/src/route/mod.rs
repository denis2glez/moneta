mod crypto_listing;
mod crypto_map;
mod health_check;

use sqlx::PgPool;
use std::net::TcpListener;
use thiserror::Error;

use actix_web::{
    dev::Server,
    web::{self, Data},
    App, HttpServer,
};
use crypto_listing::listing;
use crypto_map::map;
use health_check::health_check;

use crate::{config, database};

pub struct CfxServer {
    port: u16,
    server: Server,
}

impl CfxServer {
    pub async fn build(config: config::Configuration) -> Result<Self, std::io::Error> {
        let db_pool = database::get_connection_pool(&config.database);

        let address = format!("{}:{}", config.application.host, config.application.port);
        let listener = TcpListener::bind(&address)?;
        let port = listener.local_addr().unwrap().port();
        let server = setup(listener, db_pool)?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

#[derive(Error, Debug)]
pub enum CfxError {
    #[error("Issues loading configuration")]
    CfgError(#[from] ::config::ConfigError),
    #[error("Issues with I/O operations")]
    IoError(#[from] std::io::Error),
}

fn setup(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let pool = Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/crypto/map/", web::get().to(map))
            .route("/crypto/listing/", web::get().to(listing))
            .app_data(pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
