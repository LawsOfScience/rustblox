use std::sync::{Arc, RwLock};

use crate::client::RustbloxClient;
use super::get_cookie;

#[tokio::test]
async fn bad_cookie_test() {
    let client = RustbloxClient {
        reqwest_client: Default::default(),
        roblox_cookie: Some("_|WARNING:bad-cookie".to_string()),
        csrf_token: Arc::new(RwLock::new(Some("bad-token".to_string()))),
        auto_reauth: true
    };

    let result = client.batch_get_requests(1).await;
    println!("{:#?}", result);
    assert!(result.is_err());
}

#[tokio::test]
async fn bad_token_refresh() {
    let client = RustbloxClient {
        reqwest_client: Default::default(),
        roblox_cookie: Some(format!(".ROBLOSECURITY={}", get_cookie())),
        csrf_token: Arc::new(RwLock::new(Some("bad-token".to_string()))),
        auto_reauth: true
    };

    let result = client.batch_get_requests(5681740).await;
    println!("{:#?}", result);
    assert!(result.is_ok());
}
