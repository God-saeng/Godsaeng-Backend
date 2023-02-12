use crate::routes::events::{create_event, delete_event, patch_event};
use crate::routes::users::{create_user, delete_user, patch_user};
use crate::AppState;

use actix_web::{dev::Server, web::Data, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState {
                db: db_pool.clone(),
            }))
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
