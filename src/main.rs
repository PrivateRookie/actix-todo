#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv;

mod models;
mod schema;

type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

fn naivedate_now() -> NaiveDateTime {
    Utc::now().naive_utc()
}

#[derive(Debug, Serialize, Deserialize)]
struct UpdateEventContent {
    uid: String,
    content: Option<String>,
    finished: Option<bool>,
    #[serde(default = "naivedate_now")]
    updated_at: NaiveDateTime,
}

fn create_event(
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

fn list_events(pool: web::Data<Pool>) -> HttpResponse {
    use self::schema::events::dsl::*;
    let results = events
        .load::<models::Event>(&pool.get().unwrap())
        .expect("failed to list events");
    HttpResponse::Ok().json(results)
}

fn update_event(item: web::Json<UpdateEventContent>, pool: web::Data<Pool>) -> HttpResponse {
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

fn delete_event(item: web::Json<UpdateEventContent>, pool: web::Data<Pool>) -> HttpResponse {
    use self::schema::events::dsl::*;

    diesel::delete(events.filter(uid.eq(&item.uid)))
        .execute(&pool.get().unwrap())
        .expect("failed to delete event");
    HttpResponse::Ok().json({})
}

fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<SqliteConnection>::new(connspec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(
                web::resource("/events/")
                    .route(web::get().to(list_events))
                    .route(web::post().to(create_event))
                    .route(web::put().to(update_event))
                    .route(web::delete().to(delete_event)),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
}
