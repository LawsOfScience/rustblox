use crate::client::builder::RustbloxClientBuilder;
use crate::client::RustbloxClient;
use crate::structs::SortOrder;

async fn create_unauthed_client() -> RustbloxClient {
    let client = RustbloxClientBuilder::new()
        .build()
        .expect("Error building the client");

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

    let user_info = client.get_users_from_ids(user_ids, true).await;
    if let Err(why) = user_info {
        panic!("Had error getting user info:\n{}", why);
    }

    let info = user_info.unwrap();
    println!("{:#?}", info);
}

#[tokio::test]
async fn get_multiple_usernames() {
    let client = create_unauthed_client().await;
    let usernames = vec!["Aerasto", "TheWildDeveloper", "tannibus"];

    let user_info = client.get_users_from_usernames(usernames, true).await;
    if let Err(why) = user_info {
        panic!("Had error getting user info:\n{}", why);
    }

    let info = user_info.unwrap();
    println!("{:#?}", info);
}

#[tokio::test]
async fn get_previous_usernames() {
    let client = create_unauthed_client().await;

    let previous_usernames = client
        .get_previous_usernames(68429027, None, None, Some(SortOrder::Descending))
        .await;
    if let Err(why) = previous_usernames {
        panic!("Had error getting previous usernames:\n{}", why);
    }

    let info = previous_usernames.unwrap();
    println!("{:#?}", info);
}

#[tokio::test]
async fn search_user() {
    let client = create_unauthed_client().await;

    let user_info = client
        .search_user("TheWildDeveloper".to_string(), None, None)
        .await;
    if let Err(why) = user_info {
        panic!("Had error getting user info:\n{}", why);
    }

    let info = user_info.unwrap();
    println!("{:#?}", info);
}
