use rustblox::client::builder::RustbloxClientBuilder;
use rustblox::client::RustbloxClient;

async fn create_unauthed_client() -> RustbloxClient {
    let mut client = RustbloxClientBuilder::new()
        .build()
        .expect("Had an error building the client");
    client.login().await.unwrap();
    client
}

#[tokio::test]
async fn get_user_info() {
    let client = create_unauthed_client().await;
    let user_info = client.get_user_info(100534123).await;

    if let Err(why) = user_info {
        panic!("Had error getting user info:\n{}", why);
    }
    let info = user_info.unwrap();
    println!("{:#?}", info);
}

#[tokio::test]
async fn get_multiple_ids() {
    let client = create_unauthed_client().await;
    let user_ids = vec![68429027, 665352667, 165383308, 203539400];

    let user_info =
        client.get_users_from_ids(user_ids, true).await;
    assert!(user_info.is_ok())
}
