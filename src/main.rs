use dotenv::dotenv;
use godsaeng_backend::runner::run;
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;

use actix_session::storage::RedisSessionStore;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let store = {
        let redis_url = std::env::var("REDIS_URL").expect("REDI must be set");
        RedisSessionStore::new(redis_url).await.unwrap()
    };
    let secret_key = std::env::var("SECRET_KEY").expect("SECRET_KEY must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Error building a connection pool");

    let listener = TcpListener::bind("127.0.0.1:8000")?;
    run(listener, pool, store, secret_key)?.await?;
    Ok(())
}
