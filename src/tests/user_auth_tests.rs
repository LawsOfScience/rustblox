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
async fn change_display_name() {
    let mut client = create_authed_client().await;
    let request = client.change_display_name(1103782610, "TestingName".to_string()).await;
    println!("{:#?}", request);
    assert!(request.is_ok());
}

#[tokio::test]
async fn get_authed_user() {
    let mut client = create_authed_client().await;
    let request = client.get_authenticated_user().await;
    println!("{:#?}", request);
    assert!(request.is_ok());
}


#[tokio::test]
async fn get_authed_user_age_bracket() {
    let mut client = create_authed_client().await;
    let request = client.get_authenticated_user_age_bracket().await;
    println!("{:#?}", request);
    assert!(request.is_ok());
}

#[tokio::test]
async fn get_authed_usercountry_code() {
    let mut client = create_authed_client().await;
    let request = client.get_authenticated_user_country_code().await;
    println!("{:#?}", request);
    assert!(request.is_ok());
}

#[tokio::test]
async fn get_authed_user_roles() {
    let mut client = create_authed_client().await;
    let request = client.get_authenticated_user_roles().await;
    println!("{:#?}", request);
    assert!(request.is_ok());
}

#[tokio::test]
async fn validate_display_name() {
    let mut client = create_authed_client().await;
    let request = client.validate_user_display_name(1103782610, "TestingName".to_string()).await;
    println!("{:#?}", request);
    assert!(request.is_ok());
}
