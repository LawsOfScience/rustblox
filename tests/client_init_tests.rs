use rustblox::client::builder::RustbloxClientBuilder;

#[test]
fn build_client() {
    let client_builder = RustbloxClientBuilder::new();
    let client = client_builder.build();
    assert!(client.is_ok());
}

#[tokio::test]
async fn build_and_ping() {
    let mut client = RustbloxClientBuilder::new()
        .build()
        .expect("Had an error building the client");
    let ping = client
        .login()
        .await;
    assert!(ping.is_ok());
}

#[tokio::test]
async fn build_and_login() {
    let token_path = std::path::Path::new("tests/token.txt");
    let roblox_cookie = std::fs::read_to_string(token_path).expect("Couldn't read token.txt file");

    let mut client = RustbloxClientBuilder::new()
        .with_token(roblox_cookie.trim())
        .build()
        .expect("Had an error building the client");
    let login = client.login().await;
    assert!(login.is_ok());
}
