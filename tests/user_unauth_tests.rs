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
