use coinbase_pro_rs::{ASync, Public, SANDBOX_URL};

#[allow(unused)]
async fn get_time() {
    let client: Public<ASync> = Public::new_with_keep_alive(SANDBOX_URL, false);
    // if keep_alive is not disables - tokio::run will hold the connection without exiting test
    let time = client.get_time().await.unwrap();

    assert!(!time.iso.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_works() {
        get_time().await;
    }
}
