use super::models;
use super::schema;
use actix_web::{web, Error, HttpResponse};
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use futures::Future;

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
) -> impl Future<Item = HttpResponse, Error = Error> {
    use self::schema::events::dsl::*;
    let uu_id = uuid::Uuid::new_v4().to_string();
    let new_event = models::NewEvent {
        uid: Some(uu_id.clone()),
        content: item.content.clone(),
        finished: false,
        created_at: item.created_at,
        updated_at: item.updated_at,
    };
    let p_cloned = pool.clone();

    web::block(move || {
        diesel::insert_into(events)
            .values(&new_event)
            .execute(&pool.get().unwrap())
    })
    .then(move |res| match res {
        Ok(_) => events
            .filter(uid.eq(&uu_id))
            .load::<models::Event>(&p_cloned.get().unwrap()),
        Err(_) => Ok(vec![]),
    })
    .then(|res| match res {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
}

pub fn list_events(pool: web::Data<Pool>) -> impl Future<Item = HttpResponse, Error = Error> {
    use self::schema::events::dsl::*;
    web::block(move || events.load::<models::Event>(&pool.get().unwrap())).then(|res| match res {
        Ok(results) => Ok(HttpResponse::Ok().json(results)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
}

pub fn update_event(
    item: web::Json<UpdateEventContent>,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    use self::schema::events::dsl::*;

    let p_cloned = pool.clone();
    let item_uid = item.uid.clone();

    web::block(move || {
        events
            .filter(uid.eq(item_uid))
            .load::<models::Event>(&pool.get().unwrap())
    })
    .then(move |res| match res {
        Ok(records) => {
            diesel::update(records.last().unwrap())
                .set((
                    finished.eq(item.finished.unwrap()),
                    content.eq(item.content.clone().unwrap()),
                    updated_at.eq(item.updated_at),
                ))
                .execute(&p_cloned.get().unwrap())
                .unwrap();
            Ok(HttpResponse::Ok().json(records))
        }
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
}

pub fn delete_event(
    item: web::Json<UpdateEventContent>,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    use self::schema::events::dsl::*;
    web::block(move || {
        diesel::delete(events.filter(uid.eq(&item.uid))).execute(&pool.get().unwrap())
    })
    .then(|res| match res {
        Ok(_) => Ok(HttpResponse::Ok().json({})),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
}
