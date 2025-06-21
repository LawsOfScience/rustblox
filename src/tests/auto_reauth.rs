use std::sync::{Arc, RwLock};

use reqwest::{header::{HeaderMap, HeaderValue}, Method};
use crate::client::{RequestComponents, RustbloxClient};
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

    // Some manual stuff because I don't plan on supporting this endpoint
    // aside from this test
    let body = serde_json::json!({ "description": "testing" }).to_string();
    let mut headers = HeaderMap::new();
    headers.append("Content-Type", HeaderValue::from_static("application/json"));
    headers.append("Content-Length", HeaderValue::from_str(&body.as_bytes().len().to_string()).unwrap());

    let components = RequestComponents {
        needs_auth: true,
        method: Method::POST,
        url: "https://users.roblox.com/v1/description".into(),
        headers: Some(headers),
        body: Some(body),
    };
    let result = client.make_request::<serde_json::Value>(components, false).await;
    println!("{:#?}", result);
    assert!(result.is_ok());
}
