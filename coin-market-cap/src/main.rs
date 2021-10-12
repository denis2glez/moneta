use coin_market_cap::*;

#[tokio::main]
async fn main() -> Result<(), coin_market::Error> {
    let config = configuration::load_config().unwrap();
    println!("{:?}", config);
    
    Ok(())
}
