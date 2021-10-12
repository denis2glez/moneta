use crate::config::load_config;

pub mod config;
pub mod coin_market;

pub async fn request_data() -> Result<(), coin_market::Error> {
    // Pull new data from the server
    let config = load_config().unwrap();
    println!("{:?}", config.application);
    Ok(())
}
