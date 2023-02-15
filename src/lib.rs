pub mod routes;
pub mod runner;

use sqlx::{Pool, Postgres};

pub struct AppState {
    db: Pool<Postgres>,
}
