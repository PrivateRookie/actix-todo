#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

use actix_files as fs;
use actix_web::{middleware, web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv;

mod api;
mod models;
mod schema;

fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<SqliteConnection>::new(connspec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let addr = std::env::var("SERVE_ADDR")
        .unwrap_or("127.0.0.1:8080".to_string());
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(
                web::resource("/events/")
                    .route(web::get().to(api::list_events))
                    .route(web::post().to(api::create_event))
                    .route(web::put().to(api::update_event))
                    .route(web::delete().to(api::delete_event)),
            )
            .service(fs::Files::new("/static", "./static").index_file("index.html"))
    })
    .bind(addr)?
    .run()
}
