use crate::client::builder::RustbloxClientBuilder;

#[tokio::test]
async fn get_group_roles() {
    let mut client = RustbloxClientBuilder::new()
        .build()
        .unwrap();
    let result = client.get_group_roles(5681740).await;
    println!("{:#?}", result);
    assert!(result.is_ok());
}

#[tokio::test]
async fn get_user_group_roles() {
    let mut client = RustbloxClientBuilder::new()
        .build()
        .unwrap();
    let result = client.get_user_group_roles(68429027).await;
    println!("{:#?}", result);
    assert!(result.is_ok());
}

#[tokio::test]
async fn get_members_in_group_role() {
    let mut client = RustbloxClientBuilder::new()
        .build()
        .unwrap();
    let result = client.get_group_role_members(
        5681740,
        85311978,
        None,
        None,
        None
    ).await;
    println!("{:#?}", result);
    assert!(result.is_ok());
}
