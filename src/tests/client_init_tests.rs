use super::get_cookie;
use crate::client::builder::RustbloxClientBuilder;

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
    let ping = client.login().await;
    assert!(ping.is_ok());
}

#[tokio::test]
async fn build_and_login() {
    let mut client = RustbloxClientBuilder::new()
        .insert_cookie(&get_cookie())
        .expect("Invalid cookie")
        .build()
        .expect("Had an error building the client");

    let login = client.login().await;

    assert!(login.is_ok());
}

#[tokio::test]
async fn build_and_login_new() {
    let mut client = RustbloxClientBuilder::new()
        .insert_cookie(&get_cookie())
        .expect("Invalid cookie")
        .build()
        .expect("Had an error building the client");

    let login = client.login().await;

    assert!(login.is_ok());
}
