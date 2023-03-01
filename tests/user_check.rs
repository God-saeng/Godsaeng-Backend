use reqwest::{Client, Response};
use serde_json::{Map, Value};

mod helper;
use helper::{login, test_run, TestApp};

async fn create_user(
    app: &TestApp,
    client: &Client,
    name: &str,
    password: &str,
) -> Result<Response, std::io::Error> {
    let mut map = Map::new();
    map.insert("name".to_string(), Value::String(name.to_string()));
    map.insert("password".to_string(), Value::String(password.to_string()));

    let response = client
        .post(&format!("{}/user", &app.address))
        .json(&map)
        .send()
        .await
        .expect("Failed to execute request.");
    Ok(response)
}

async fn patch_user(
    app: &TestApp,
    client: &Client,
    name: &str,
    password: &str,
    new_name: &str,
    new_password: &str,
) -> Result<Response, std::io::Error> {
    let mut map = Map::new();
    map.insert("name".to_string(), Value::String(name.to_string()));
    map.insert("password".to_string(), Value::String(password.to_string()));
    map.insert("new_name".to_string(), Value::String(new_name.to_string()));
    map.insert(
        "new_password".to_string(),
        Value::String(new_password.to_string()),
    );

    let response = client
        .patch(&format!("{}/user", &app.address))
        .json(&map)
        .send()
        .await
        .expect("Failed to execute request.");
    Ok(response)
}

async fn delete_user(
    app: &TestApp,
    client: &Client,
    name: &str,
    password: &str,
) -> Result<Response, std::io::Error> {
    let mut map = Map::new();
    map.insert("name".to_string(), Value::String(name.to_string()));
    map.insert("password".to_string(), Value::String(password.to_string()));

    let response = client
        .delete(&format!("{}/user", &app.address))
        .json(&map)
        .send()
        .await
        .expect("Failed to execute request.");
    Ok(response)
}

#[tokio::test]
async fn create_user_works() {
    let app = test_run().await;
    let client = reqwest::Client::new();
    let response: Response = create_user(&app, &client, "test_user", "test_password")
        .await
        .expect("Failed to create user");
    assert!(response.status().is_success());
}

#[tokio::test]
async fn create_user_returns_a_400_for_duplicated_id() {
    let app = test_run().await;
    let client = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .expect("Failed to create Client");
    let response: Response = create_user(&app, &client, "admin", "test_password")
        .await
        .expect("Failed to create user");

    assert_eq!(400, response.status().as_u16());
}

#[tokio::test]
async fn create_user_returns_a_500_for_long_id() {
    let app = test_run().await;
    let client = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .expect("Failed to create Client");
    let response: Response = create_user(
        &app,
        &client,
        "test_user_loooooooooooooooooooooooooooooong_id",
        "test_password",
    )
    .await
    .expect("Failed to create user");

    assert_eq!(500, response.status().as_u16());
}

#[tokio::test]
async fn patch_user_works() {
    let app = test_run().await;
    let client = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .expect("Failed to create Client");

    login(&app, &client, "admin", "secret")
        .await
        .expect("Failed to login as admin");

    let response: Response = patch_user(
        &app,
        &client,
        "admin",
        "secret",
        "test_user_new",
        "test_password_new",
    )
    .await
    .expect("Failed to create user");

    assert!(response.status().is_success());
}

#[tokio::test]
async fn patch_user_returns_a_400_for_duplicated_id() {
    let app = test_run().await;
    let client = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .expect("Failed to create Client");

    login(&app, &client, "admin", "secret")
        .await
        .expect("Failed to login as admin");

    let response: Response = patch_user(
        &app,
        &client,
        "admin",
        "secret",
        "admin",
        "test_password_new",
    )
    .await
    .expect("Failed to create user");

    assert_eq!(400, response.status().as_u16());
}

#[tokio::test]
async fn delete_user_works() {
    let app = test_run().await;
    let client = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .expect("Failed to create Client");

    login(&app, &client, "admin", "secret")
        .await
        .expect("Failed to login as admin");

    let response: Response = delete_user(&app, &client, "admin", "secret")
        .await
        .expect("Failed to create user");
    assert!(response.status().is_success());
}

#[tokio::test]
async fn delete_user_returns_a_401_for_invalid_password() {
    let app = test_run().await;
    let client = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .expect("Failed to create Client");

    login(&app, &client, "admin", "secret")
        .await
        .expect("Failed to login as admin");

    let response: Response = delete_user(&app, &client, "admin", "secret_ss")
        .await
        .expect("Failed to create user");
    assert_eq!(401, response.status().as_u16());
}
