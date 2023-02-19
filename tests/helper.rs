use actix_session::storage::RedisSessionStore;
use dotenv::dotenv;
use godsaeng_backend::runner::run;
use reqwest::{Client, Response};
use serde_json::{Map, Value};
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;

use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

pub async fn test_run() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{port}");

    dotenv().ok();
    let db_url = std::env::var("TEST_DATABASE_URL").expect("DATABASE_URL must be set");
    let test_database = format!("{db_url}/postgres");

    let mut connection = PgConnection::connect(&test_database)
        .await
        .expect("Failed to connect to Postgres");

    let database_name = Uuid::new_v4().to_string();

    connection
        .execute(&*format!(r#"CREATE DATABASE "{database_name}";"#))
        .await
        .expect("Failed to create database.");

    let test_database_url = format!("{db_url}/{database_name}");
    println!("{test_database_url}");
    let connection_pool = PgPool::connect(&test_database_url)
        .await
        .expect("Failed to connect to Postgres");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");
    let store = {
        let redis_url = std::env::var("REDIS_URL").expect("REDI must be set");
        RedisSessionStore::new(redis_url).await.unwrap()
    };
    let secret_key = std::env::var("SECRET_KEY").expect("SECRET_KEY must be set");
    let server =
        run(listener, connection_pool.clone(), store, secret_key).expect("Failed to bind address");
    tokio::spawn(server);

    TestApp {
        address,
        db_pool: connection_pool,
    }
}

pub async fn login(
    app: &TestApp,
    client: &Client,
    name: &str,
    password: &str,
) -> Result<Response, std::io::Error> {
    let mut map = Map::new();
    map.insert("name".to_string(), Value::String(name.to_string()));
    map.insert("password".to_string(), Value::String(password.to_string()));
    let response = client
        .post(&format!("{}/login", &app.address))
        .json(&map)
        .send()
        .await
        .expect("Failed to execute request.");
    Ok(response)
}
