use actix_web::{web, Responder};
use crate::models::countries::Country;
use sqlx::PgPool;

pub async fn get_countries(db: web::Data<PgPool>) -> impl Responder {
    let result = sqlx::query_as::<_, Country>("select id, name, region from countries")
        .fetch_all(db.get_ref())
        .await;

    match result {
        Ok(countries) => web::Json(countries),
        Err(e) => {
            eprintln!("Database query failed: {}", e);
            web::Json(Vec::<Country>::new())
        }
    }
}
