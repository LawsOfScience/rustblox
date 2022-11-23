use crate::client::builder::RustbloxClientBuilder;
use crate::client::RustbloxClient;
use super::get_cookie;

async fn create_authed_client() -> RustbloxClient {
    let mut client = RustbloxClientBuilder::new()
        .insert_cookie(&get_cookie())
        .expect("Error inserting cookie")
        .build()
        .expect("Error building the client");

    client.login().await.unwrap();
    client
}

#[tokio::test]
async fn get_user_join_request() {
    let mut client = create_authed_client().await;
    let request = client.get_user_join_request(5681740, 1115834788).await;
    println!("{:#?}", request);
}

#[tokio::test]
async fn accept_join_request() {
    let mut client = create_authed_client().await;
    let result = client.accept_user_join_request(5681740, 1115834788).await;
    println!("{:#?}", result);
    assert!(result.is_ok());
}

#[tokio::test]
async fn deny_join_request() {
    let mut client = create_authed_client().await;
    let result = client.deny_user_join_request(5681740, 1167483749).await;
    println!("{:#?}", result);
    assert!(result.is_ok());
}

#[tokio::test]
async fn batch_get_requests() {
    let mut client = create_authed_client().await;
    let result = client.batch_get_requests(5681740).await;
    println!("{:#?}", result);
    assert!(result.is_ok());
}

#[tokio::test]
async fn kick_user() {
    let mut client = create_authed_client().await;
    let result = client.kick_user(5681740, 1115834788).await;
    println!("{:#?}", result);
    assert!(result.is_ok());
}

#[tokio::test]
async fn get_group_roles() {
    let mut client = create_authed_client().await;
    let result = client.get_group_roles(5681740).await;
    println!("{:#?}", result);
    assert!(result.is_ok());
}

#[tokio::test]
async fn set_user_rank() {
    let mut client = create_authed_client().await;
    let result = client.set_user_role_in_group(5681740, 1115834788, 6).await;
    println!("{:#?}", result);
    assert!(result.is_ok());
}
