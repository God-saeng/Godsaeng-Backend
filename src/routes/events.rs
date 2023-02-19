use crate::AppState;
use actix_web::{
    delete, patch, post,
    web::{Data, Json},
    HttpResponse, Responder,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::{self, FromRow};

use actix_session::Session;

#[derive(Deserialize)]
pub struct CreateEventBody {
    pub note: String,
    pub event_date: String,
}

#[derive(Deserialize)]
pub struct PatchEventBody {
    pub id: i32,
    pub new_note: String,
    pub new_event_date: String,
}

#[derive(Deserialize)]
pub struct DeleteEventBody {
    pub id: i32,
}

#[derive(Serialize, FromRow)]
struct IdRow {
    pub id: i32,
}

#[derive(Serialize, FromRow)]
struct UserIdRow {
    pub user_id: i32,
}
// -> rust macro
pub fn is_valid_user(session: &Session) -> i32 {
    let user_id: Option<i32> = match session.get("user_id") {
        Ok(x) => x,
        Err(_) => Some(-1),
    };
    user_id.unwrap_or(-1)
}

#[post("/event")]
pub async fn create_event(
    state: Data<AppState>,
    body: Json<CreateEventBody>,
    session: Session,
) -> impl Responder {
    let user_id = is_valid_user(&session);
    if user_id == -1 {
        return HttpResponse::Unauthorized().json("Login first");
    }

    let ymd: Vec<&str> = body.event_date.split('-').collect();
    let date = NaiveDate::from_ymd_opt(
        ymd[0].parse().unwrap(),
        ymd[1].parse().unwrap(),
        ymd[2].parse().unwrap(),
    );

    match sqlx::query_as::<_, IdRow>(
        "INSERT INTO event (user_id, note, event_date) VALUES ($1, $2, $3) RETURNING id",
    )
    .bind(user_id)
    .bind(body.note.to_string())
    .bind(date)
    .fetch_one(&state.db)
    .await
    {
        Ok(id_row) => HttpResponse::Ok().json(id_row),
        Err(_) => HttpResponse::InternalServerError().json("Failed to create event"),
    }
}

#[patch("/event")]
pub async fn patch_event(
    state: Data<AppState>,
    body: Json<PatchEventBody>,
    session: Session,
) -> impl Responder {
    let ymd: Vec<&str> = body.new_event_date.split('-').collect();
    let new_date = NaiveDate::from_ymd_opt(
        ymd[0].parse().unwrap(),
        ymd[1].parse().unwrap(),
        ymd[2].parse().unwrap(),
    );
    let user_id = is_valid_user(&session);
    if user_id == -1 {
        return HttpResponse::Unauthorized().json("Login first");
    }
    let user_id_check: i32 =
        match sqlx::query_as::<_, UserIdRow>("SELECT user_id FROM event WHERE id = $1")
            .bind(body.id)
            .fetch_one(&state.db)
            .await
        {
            Ok(id_row) => id_row.user_id,
            Err(_) => return HttpResponse::BadRequest().json("Failed to find the event"),
        };

    if user_id != user_id_check {
        return HttpResponse::Unauthorized().json("Unauthorized event");
    }

    match sqlx::query_as::<_, IdRow>(
        "UPDATE event SET note = $1, event_date = $2 WHERE id = $3 RETURNING id",
    )
    .bind(body.new_note.to_string())
    .bind(new_date)
    .bind(body.id)
    .fetch_one(&state.db)
    .await
    {
        Ok(id_row) => HttpResponse::Ok().json(id_row),
        Err(_) => HttpResponse::InternalServerError().json("Failed to patch event"),
    }
}

#[delete("/event")]
pub async fn delete_event(
    state: Data<AppState>,
    body: Json<DeleteEventBody>,
    session: Session,
) -> impl Responder {
    let user_id = is_valid_user(&session);
    if user_id == -1 {
        return HttpResponse::Unauthorized().json("Login first");
    }
    let user_id_check =
        match sqlx::query_as::<_, UserIdRow>("SELECT user_id from event WHERE id = $1")
            .bind(body.id)
            .fetch_one(&state.db)
            .await
        {
            Ok(id_row) => id_row.user_id,
            Err(_) => return HttpResponse::BadRequest().json("Failed to find the event"),
        };
    if user_id != user_id_check {
        return HttpResponse::Unauthorized().json("Unauthorized event");
    }

    match sqlx::query("DELETE FROM event WHERE id = $1")
        .bind(body.id)
        .execute(&state.db)
        .await
    {
        Ok(_) => HttpResponse::Ok().json("Successfully deleted"),
        Err(_) => HttpResponse::InternalServerError().json("Failed to delete event"),
    }
}
