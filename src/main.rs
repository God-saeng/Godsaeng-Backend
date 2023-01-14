mod events;
mod services;

use actix_web::{web::Data, App, HttpServer};
use dotenv::dotenv;
use events::create_event;
use services::{create_user, delete_user, patch_user};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

mod services;
use services::{create_user, patch_user, delete_user};

mod events;
use events::{create_event, patch_event};
pub struct AppState {
    db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Error building a connection pool");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState { db: pool.clone() }))
            .service(create_user)
            .service(patch_user)
            .service(delete_user)
            .service(create_event)
            .service(patch_event)
    })
    .bind(("0.0.0.0", 18421))?
    .run()
    .await
}
