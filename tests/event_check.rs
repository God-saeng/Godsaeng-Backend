use reqwest::{Client, Response};
use serde::Deserialize;
use serde_json::{Map, Number, Value};

mod helper;
use helper::{login, test_run, TestApp};

#[derive(Deserialize)]
struct IdRow {
    pub id: i32,
}

async fn create_event(
    app: &TestApp,
    client: &Client,
    note: &str,
    event_date: &str,
) -> Result<Response, std::io::Error> {
    let mut map = Map::new();
    map.insert("note".to_string(), Value::String(note.to_string()));
    map.insert(
        "event_date".to_string(),
        Value::String(event_date.to_string()),
    );
    let response = client
        .post(&format!("{}/event", &app.address))
        .json(&map)
        .send()
        .await
        .expect("Failed to execute request.");
    Ok(response)
}

async fn patch_event(
    app: &TestApp,
    client: &Client,
    id: i32,
    new_note: &str,
    new_event_date: &str,
) -> Result<Response, std::io::Error> {
    let mut map = Map::new();
    map.insert("id".to_string(), Value::Number(Number::from(id)));
    map.insert("new_note".to_string(), Value::String(new_note.to_string()));
    map.insert(
        "new_event_date".to_string(),
        Value::String(new_event_date.to_string()),
    );
    let response = client
        .patch(&format!("{}/event", &app.address))
        .json(&map)
        .send()
        .await
        .expect("Failed to execute request.");
    Ok(response)
}

async fn delete_event(app: &TestApp, client: &Client, id: i32) -> Result<Response, std::io::Error> {
    let mut map = Map::new();
    map.insert("id".to_string(), Value::Number(Number::from(id)));
    let response = client
        .delete(&format!("{}/event", &app.address))
        .json(&map)
        .send()
        .await
        .expect("Failed to execute request.");
    Ok(response)
}

#[tokio::test]
async fn create_event_works() {
    let app = test_run().await;
    let client = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .expect("Failed to create Client");

    login(&app, &client, "admin", "secret")
        .await
        .expect("Failed to login as admin");

    let response: Response = create_event(&app, &client, "test_note", "2022-01-01")
        .await
        .expect("Failed to create event.");
    assert!(response.status().is_success());

    let ret: IdRow = response.json().await.expect("Failed to get the json");
    assert_eq!(ret.id, 1)
}

#[tokio::test]
async fn patch_event_works() {
    let app = test_run().await;
    let client = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .expect("Failed to create Client");

    login(&app, &client, "admin", "secret")
        .await
        .expect("Failed to login as admin");

    let event_id = {
        let response: Response = create_event(&app, &client, "test_note", "2022-01-01")
            .await
            .expect("Failed to create event.");
        let ret: IdRow = response.json().await.expect("Failed to get the json");
        ret.id
    };
    assert_eq!(event_id, 1);

    let response: Response = patch_event(&app, &client, event_id, "new_note", "2023-01-01")
        .await
        .expect("Failed to patch event");
    assert!(response.status().is_success());
}

#[tokio::test]
async fn patch_event_returns_a_400_for_invalid_id() {
    let app = test_run().await;
    let client = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .expect("Failed to create Client");

    login(&app, &client, "admin", "secret")
        .await
        .expect("Failed to login as admin");
    let event_id = {
        let response: Response = create_event(&app, &client, "test_note", "2022-01-01")
            .await
            .expect("Failed to create event.");
        let ret: IdRow = response.json().await.expect("Failed to get the json");
        ret.id
    };
    assert_eq!(event_id, 1);

    let response: Response = patch_event(&app, &client, event_id + 1, "new_note", "2023-01-01")
        .await
        .expect("Failed to patch event");
    assert_eq!(400, response.status().as_u16());
}

#[tokio::test]
async fn delete_event_works() {
    let app = test_run().await;
    let client = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .expect("Failed to create Client");

    login(&app, &client, "admin", "secret")
        .await
        .expect("Failed to login as admin");
    let event_id = {
        let response: Response = create_event(&app, &client, "test_note", "2022-01-01")
            .await
            .expect("Failed to create event.");
        let ret: IdRow = response.json().await.expect("Failed to get the json");
        ret.id
    };
    let response: Response = delete_event(&app, &client, event_id)
        .await
        .expect("Failed to patch event");

    assert!(response.status().is_success());
}
