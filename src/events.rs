use crate::AppState;
use actix_web::{
    post, patch, delete,
    web::{Data, Json},
    HttpResponse, Responder,
};
use chrono::NaiveDate;
use serde::Deserialize;
use sqlx::{self, FromRow};

#[derive(Deserialize)]
pub struct CreateEventBody {
    pub user_id: i32,
    pub note: String,
    pub event_date: String,
}

#[derive(Deserialize)]
pub struct PatchEventBody {
    pub id: i32,
    pub new_note: String,
}

#[derive(Deserialize)]
pub struct DeleteEventBody {
    pub id: i32
}

#[derive(FromRow)]
struct IdRow {
    pub id: i32,
}

#[post("/event")]
pub async fn create_event(state: Data<AppState>, body: Json<CreateEventBody>) -> impl Responder {
    let ymd: Vec<&str> = body.event_date.split('-').collect();
    let date = NaiveDate::from_ymd_opt(
        ymd[0].parse().unwrap(),
        ymd[1].parse().unwrap(),
        ymd[2].parse().unwrap(),
    );

    match sqlx::query_as::<_, IdRow>(
        "INSERT INTO event (user_id, note, event_date) VALUES ($1, $2, $3) RETURNING id",
    )
    .bind(body.user_id)
    .bind(body.note.to_string())
    .bind(date)
    .fetch_one(&state.db)
    .await
    {
        Ok(id_row) => HttpResponse::Ok().json(id_row.id),
        Err(_) => HttpResponse::InternalServerError().json("Failed to create event"),
    }
}

#[patch("/event")]
pub async fn patch_event(state: Data<AppState>, body: Json<PatchEventBody>) -> impl Responder {
    match sqlx::query_as::<_, IdRow>("UPDATE event SET note = $1 WHERE id = $2 RETURNING id")
        .bind(body.new_note.to_string())
        .bind(body.id)
        .fetch_one(&state.db)
        .await
    {
        Ok(id_row) => HttpResponse::Ok().json(id_row.id),
        Err(_) => HttpResponse::InternalServerError().json("Failed to patch event"),
    }
}

#[delete("/event")]
pub async fn delete_event(state: Data<AppState>, body: Json<DeleteEventBody>) -> impl Responder{

    match sqlx::query(
        "DELETE FROM event WHERE id = $1"
    )
        .bind(body.id)
        .execute(&state.db)
        .await
    {
        Ok(_) => HttpResponse::Ok().json("Successfully deleted"),
        Err(_) => HttpResponse::InternalServerError().json("Failed to delete event")
    }
}