use crate::routes::events::{create_event, delete_event, patch_event};
use crate::routes::users::{create_user, delete_user, login, patch_user};
use crate::AppState;

use actix_web::{dev::Server, web::Data, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;

use actix_web::cookie::Key;

use actix_session::storage::RedisSessionStore;
use actix_session::SessionMiddleware;

pub fn run(
    listener: TcpListener,
    db_pool: PgPool,
    store: RedisSessionStore,
    secret_key: String,
) -> Result<Server, std::io::Error> {
    let key = Key::from(secret_key.as_bytes()); // at least 64 byte
    let server = HttpServer::new(move || {
        App::new()
            .wrap(
                SessionMiddleware::builder(store.clone(), key.clone())
                    .cookie_name("session".to_string())
                    .build(),
            )
            .app_data(Data::new(AppState {
                db: db_pool.clone(),
            }))
            .service(login)
            .service(create_user)
            .service(patch_user)
            .service(delete_user)
            .service(create_event)
            .service(patch_event)
            .service(delete_event)
    })
    .listen(listener)?
    .run();

    Ok(server)
}
