use crate::client::RustbloxClient;
use super::get_cookie;

#[tokio::test]
async fn bad_cookie_test() {
    let mut client = RustbloxClient {
        reqwest_client: Default::default(),
        roblox_cookie: Some("_|WARNING:bad-cookie".to_string()),
        csrf_token: Some("bad-token".to_string()),
        auto_reauth: true
    };

    let result = client.batch_get_requests(1).await;
    println!("{:#?}", result);
    assert!(result.is_err());
}

#[tokio::test]
async fn bad_token_refresh() {
    // Great, the group endpoints return a 401 if your x-csrf-token is bad...
    // guess I'll need to adjust code later
    let mut client = RustbloxClient {
        reqwest_client: Default::default(),
        roblox_cookie: Some(get_cookie()),
        csrf_token: Some("bad-token".to_string()),
        auto_reauth: true
    };

    let result = client.batch_get_requests(5681740).await;
    println!("{:#?}", result);
}
