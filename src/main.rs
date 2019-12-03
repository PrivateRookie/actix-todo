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
struct MyEvent {
    content: String,
    #[serde(default = "naivedate_now")]
    created_at: NaiveDateTime,
    #[serde(default = "naivedate_now")]
    updated_at: NaiveDateTime,
}

fn create_event(item: web::Json<MyEvent>, pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    use self::schema::events::dsl::*;
    let uid = uuid::Uuid::new_v4().to_string();
    let new_event = models::NewEvent {
        id: uid.clone(),
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
        .filter(id.eq(&uid))
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
                    .route(web::post().to(create_event)),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
}
