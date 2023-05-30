use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::env;

mod handlers;
mod models;
mod middleware;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let bind_address = env::var("BIND_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8080".to_string());

    HttpServer::new(move || {
        App::new()
            .data(models::Pool::new(&database_url))
            .configure(handlers::init_routes)
    })
    .bind(bind_address)?
    .run()
    .await
}
