use super::models;
use super::schema;
use actix_web::{web, Error, HttpResponse};
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub fn naivedate_now() -> NaiveDateTime {
    Utc::now().naive_utc()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateEventContent {
    uid: String,
    content: Option<String>,
    finished: Option<bool>,
    #[serde(default = "naivedate_now")]
    updated_at: NaiveDateTime,
}

pub fn create_event(
    item: web::Json<models::NewEvent>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    use self::schema::events::dsl::*;
    let uu_id = uuid::Uuid::new_v4().to_string();
    let new_event = models::NewEvent {
        uid: uu_id.clone(),
        content: item.content.clone(),
        finished: false,
        created_at: item.created_at,
        updated_at: item.updated_at,
    };
    let conn = &pool.get().unwrap();

    diesel::insert_into(events)
        .values(&new_event)
        .execute(conn)
        .expect("failed to create record");
    let new_user = events
        .filter(uid.eq(&uu_id))
        .load::<models::Event>(conn)
        .expect("failed to load user");
    Ok(HttpResponse::Ok().json(new_user))
}

pub fn list_events(pool: web::Data<Pool>) -> HttpResponse {
    use self::schema::events::dsl::*;
    let results = events
        .load::<models::Event>(&pool.get().unwrap())
        .expect("failed to list events");
    HttpResponse::Ok().json(results)
}

pub fn update_event(item: web::Json<UpdateEventContent>, pool: web::Data<Pool>) -> HttpResponse {
    use self::schema::events::dsl::*;
    let record = events
        .filter(uid.eq(&item.uid))
        .load::<models::Event>(&pool.get().unwrap())
        .expect("failed to find record");

    diesel::update(record.last().unwrap())
        .set((
            finished.eq(item.finished.unwrap()),
            content.eq(item.content.clone().unwrap()),
            updated_at.eq(item.updated_at),
        ))
        .execute(&pool.get().unwrap())
        .expect("failed to update event");

    HttpResponse::Ok().json(record)
}

pub fn delete_event(item: web::Json<UpdateEventContent>, pool: web::Data<Pool>) -> HttpResponse {
    use self::schema::events::dsl::*;

    diesel::delete(events.filter(uid.eq(&item.uid)))
        .execute(&pool.get().unwrap())
        .expect("failed to delete event");
    HttpResponse::Ok().json({})
}
