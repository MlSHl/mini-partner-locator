use actix_web::{web, post, HttpResponse, Responder};
use sqlx::{query_as, PgPool};

use crate::models::partners::{NewPartner, Partner};

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

#[post("/partners")]
pub async fn create_partner(
    db: web::Data<PgPool>,
    new_partner: web::Json<NewPartner>
) -> impl Responder {
    let query = r#"
        insert into partners 
        (name, country, city, email, website_url)
        values ($1, $2, $3, $4, $5)
        returning id, name, country, city, email, website_url, created_at

    "#;

    let result = query_as::<_, Partner>(query)
        .bind(&new_partner.name)
        .bind(&new_partner.country)
        .bind(&new_partner.city)
        .bind(&new_partner.email)
        .bind(&new_partner.website_url)
        .fetch_all(db.get_ref())
        .await;

    match result {
        Ok(partner) => HttpResponse::Created().json(partner),
        Err(e) => {
            eprintln!("Failed to insert partner: {}" , e);
            HttpResponse::InternalServerError().finish()
        }
    }
    
}
