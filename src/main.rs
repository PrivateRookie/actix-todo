#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

use actix_files as fs;
use actix_web::{middleware, web, App, HttpServer, Result};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv;

mod api;
mod models;
mod schema;

fn index() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("./static/index.html")?)
}

fn icon() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("./static/favicon.ico")?)
}

fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<SqliteConnection>::new(connspec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let addr = std::env::var("SERVE_ADDR").unwrap_or("127.0.0.1:8080".to_string());
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(
                web::resource("/events/")
                    .route(web::get().to_async(api::list_events))
                    .route(web::post().to_async(api::create_event))
                    .route(web::put().to_async(api::update_event))
                    .route(web::delete().to_async(api::delete_event)),
            )
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/favicon.ico").route(web::get().to(icon)))
            .service(
                fs::Files::new("/static", "./static")
                    .redirect_to_slash_directory()
                    .show_files_listing(),
            )
    })
    .bind(addr)?
    .run()
}
