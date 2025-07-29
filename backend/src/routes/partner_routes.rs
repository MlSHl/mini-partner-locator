use actix_web::{get, web, HttpResponse, Responder};
use sqlx::{PgPool, query_as};

use crate::models::partners::Partner;
use crate::db::partner_queries::fetch_partner;

pub async fn get_partners(db: web::Data<PgPool>) -> impl Responder {
    let result = query_as::<_, Partner>("select * from partners")
        .fetch_all(db.get_ref())
        .await;

    match result {
        Ok(partners) => web::Json(partners),
        Err(e) => {
            eprintln!("Fetching users failed: {}", e);
            web::Json(Vec::<Partner>::new())
        }
    }
}

#[get("/partners/{id}")]
pub async fn get_partner_by_id(
    db: web::Data<PgPool>,
    id: web::Path<i32>
) -> impl Responder{
    let id = id.into_inner();

    let result = fetch_partner(db.get_ref(), id).await;

    match result {
        Ok(Some(partner)) => {
            println!("Partner fetched successfully with id: {id}");
            HttpResponse::Ok().json(partner)
        }
        Ok(None) => {
            eprintln!("No partner found for id: {id}");
            HttpResponse::NotFound().body(format!("No partner found for id: {id}"))
        }
        Err(e) => {
            eprintln!("Error fetching partner: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
