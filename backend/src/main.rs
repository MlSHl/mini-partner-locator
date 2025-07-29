mod models;
mod routes;
mod db;

use std::env;

use actix_web::{get, web, App, HttpServer, Responder};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;

use crate::routes::{admin_routes::{add_country_to_partner, create_partner, delete_partner, get_partner_details_by_id, remove_country_from_partner, update_partner}, country_routes::{get_countries, get_countries_by_region, get_regions}, 
    partner_routes::{get_partner_by_id, get_partners}};

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
            .service(delete_partner)
            .service(get_partner_by_id)
            .service(add_country_to_partner)
            .service(update_partner)
            .service(remove_country_from_partner)
            .service(get_countries_by_region)
            .service(get_regions)
            .service(get_partner_details_by_id)
            .route("/countries", web::get().to(get_countries))
            .route("/partners", web::get().to(get_partners))
            .app_data(web::Data::new(db_pool.clone()))
        })
        .bind(("127.0.0.1", port))?
        .workers(2)
        .run()
        .await
}
