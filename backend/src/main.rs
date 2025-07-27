mod models;
mod routes;

use std::env;

use actix_web::{get, web, App, HttpServer, Responder};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;

use crate::routes::{country_routes::get_countries, partner_routes::{create_partner, get_partners}};

#[get("/health")]
async fn health() -> impl Responder {
    "Backend is running" 
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 8080;
    println!("Starting server on port: {port}");
    dotenv().ok();
    println!("Environment variables loaded from .env");

    // Database connection setup 
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to Postgres database");
    println!("Connected to database");
    

    HttpServer::new(move || {
        App::new()
            .service(health)
            .service(create_partner)
            .route("/countries", web::get().to(get_countries))
            .route("/partners", web::get().to(get_partners))
            .app_data(web::Data::new(db_pool.clone()))
        })
        .bind(("127.0.0.1", port))?
        .workers(2)
        .run()
        .await
}
