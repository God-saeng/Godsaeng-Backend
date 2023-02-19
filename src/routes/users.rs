use crate::AppState;
use actix_web::{
    delete, patch, post,
    web::{Data, Json},
    HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};
use sqlx::{self, FromRow};

#[derive(Serialize, FromRow)]
struct User {
    id: i32,
    name: String,
}
#[derive(Deserialize)]
pub struct CreateUserBody {
    pub name: String,
    pub password: String,
}
#[derive(Deserialize)]
pub struct PatchUserBody {
    pub name: String,
    pub password: String,
    pub new_name: String,
    pub new_password: String,
}
#[derive(Deserialize)]
pub struct DeleteUserBody {
    pub name: String,
    pub password: String,
}
#[derive(FromRow)]
struct IdRow {
    pub id: i32,
}

use actix_session::Session;

#[post("/user")]
pub async fn create_user(state: Data<AppState>, body: Json<CreateUserBody>) -> impl Responder {
    // check name duplication
    match check_duplication(&state, body.name.to_string()).await {
        true => {}
        false => return HttpResponse::BadRequest().json("Name is already exist"),
    }
    match sqlx::query_as::<_, User>(
        "INSERT INTO userInfo (name, password) VALUES ($1, $2) RETURNING id, name",
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

async fn check_duplication(state: &Data<AppState>, name: String) -> bool {
    match sqlx::query_as::<_, User>(
        "SELECT id, name FROM userInfo WHERE name = $1", // will be changed to `SELECT COUNT`
    )
    .bind(name)
    .fetch_all(&state.db)
    .await
    {
        Ok(num) => matches!(num.len(), 0),
        Err(_) => false,
    }
}

pub fn is_valid_user(session: &Session) -> i32 {
    let user_id: Option<i32> = match session.get("user_id") {
        Ok(x) => x,
        Err(_) => Some(-1),
    };
    user_id.unwrap_or(-1)
}

#[patch("/user")]
pub async fn patch_user(
    state: Data<AppState>,
    body: Json<PatchUserBody>,
    session: Session,
) -> impl Responder {
    let user_id = is_valid_user(&session);
    if user_id == -1 {
        return HttpResponse::Unauthorized().json("Login first");
    }

    match check_duplication(&state, body.new_name.to_string()).await {
        true => {}
        false => return HttpResponse::BadRequest().json("Name is already exist"),
    }

    match sqlx::query_as::<_, User>(
        "UPDATE userInfo SET name = $1, password = $2 WHERE id = $3 RETURNING id, name",
    )
    .bind(body.new_name.to_string())
    .bind(body.new_password.to_string())
    .bind(user_id)
    .fetch_one(&state.db)
    .await
    {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().json("Failed to patch user"),
    }
}

#[delete("/user")]
pub async fn delete_user(
    state: Data<AppState>,
    body: Json<DeleteUserBody>,
    session: Session,
) -> impl Responder {
    let user_id = is_valid_user(&session);
    if user_id == -1 {
        return HttpResponse::Unauthorized().json("Login first");
    }
    // check one more time
    let user_id_check: i32 =
        match user_auth(&state, body.name.to_string(), body.password.to_string()).await {
            -1 => return HttpResponse::Unauthorized().json("Authentication failed"),
            x => x,
        };
    if user_id != user_id_check {
        return HttpResponse::Unauthorized().json("Input your ID and Password");
    }

    match sqlx::query("DELETE FROM userInfo WHERE id = $1")
        .bind(user_id_check)
        .execute(&state.db)
        .await
    {
        Ok(_) => HttpResponse::Ok().json("Successfully deleted"),
        Err(_) => HttpResponse::InternalServerError().json("Failed to delete user"),
    }
}

#[derive(Deserialize)]
pub struct LoginUserBody {
    pub name: String,
    pub password: String,
}

async fn user_auth(state: &Data<AppState>, name: String, password: String) -> i32 {
    match sqlx::query_as::<_, IdRow>("SELECT id FROM userInfo WHERE name = $1 AND password = $2")
        .bind(name)
        .bind(password)
        .fetch_one(&state.db)
        .await
    {
        Ok(id_row) => id_row.id,
        Err(_) => -1,
    }
}

#[post("/login")]
pub async fn login(
    state: Data<AppState>,
    body: Json<LoginUserBody>,
    session: Session,
) -> impl Responder {
    let user_id: i32 =
        match user_auth(&state, body.name.to_string(), body.password.to_string()).await {
            -1 => return HttpResponse::Unauthorized().json("Authentication failed"),
            x => x,
        };

    session
        .insert("user_id", user_id)
        .expect("Error to insert session");
    session.renew();

    // ##
    // have to add session counter
    // ##

    HttpResponse::Ok().json(User {
        id: user_id,
        name: body.name.to_string(),
    })
}
