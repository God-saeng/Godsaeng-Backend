use dotenv::dotenv;
use godsaeng_backend::runner::run;
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Error building a connection pool");

    let listener = TcpListener::bind("127.0.0.1:8000")?;
    run(listener, pool)?.await?;
    Ok(())
}
