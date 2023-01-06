use actix_web::{
    post,
    web::{Data, Json},
    Responder, HttpResponse
};
use serde::{Deserialize, Serialize};
use sqlx::{self, FromRow};
use crate::AppState;

#[derive(Serialize, FromRow)]
struct User {
    id: i32,
    name: String
}

#[derive(Deserialize)]
pub struct CreateUserBody {
    pub name: String,
    pub password: String,
}

#[post("/user")]
pub async fn create_user(state: Data<AppState>, body: Json<CreateUserBody>) -> impl Responder {
    // check name duplication
    match sqlx::query_as::<_, User>(
        "SELECT id, name FROM userInfo WHERE name = $1" // will be changed to `SELECT COUNT`
    )
        .bind(body.name.to_string())
        .fetch_all(&state.db)
        .await
    {
        Ok(num) => {
            match num.len() {   
                0 => {},
                1 => return HttpResponse::BadRequest().json("Name is already exist"),
                _ => return HttpResponse::InternalServerError().json("Unintended Error")
            }
        },
        Err(_) => {
            return HttpResponse::InternalServerError().json("Unintended Error")}
    }
    // create user
    match sqlx::query_as::<_, User>(
        "INSERT INTO userInfo (name, password) VALUES ($1, $2) RETURNING id, name"
    )
        .bind(body.name.to_string())
        .bind(body.password.to_string())
        .fetch_one(&state.db)
        .await
    {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().json("Failed to create user"),
    }
}