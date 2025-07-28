use actix_web::{web, post, HttpResponse, Responder};
use sqlx::{query, query_as, PgPool};

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
        (name, email, website_url)
        values ($1, $2, $3)
        returning id, name, email, website_url, created_at
    "#;

    let partner_result = query_as::<_, Partner>(query)
        .bind(&new_partner.name)
        .bind(&new_partner.email)
        .bind(&new_partner.website_url)
        .fetch_one(db.get_ref())
        .await;

    match partner_result {
        Ok(partner) => {
            for &country_id in &new_partner.country_ids {
                let join_insert = query!("
                    insert into partner_countries
                    (partner_id, country_id)
                    values
                    ($1, $2)
                    ", partner.id, country_id)
                    .execute(db.get_ref())
                    .await;
                
                if let Err(e) = join_insert {
                    eprintln!("Failed to insert into partner_countries: {}", e);
                    return HttpResponse::InternalServerError().finish();
                }
            }
            HttpResponse::Created().json(partner)
        },
        Err(e) => {
            eprintln!("Failed to insert partner: {}" , e);
            HttpResponse::InternalServerError().finish()
        }
    }
    
}
