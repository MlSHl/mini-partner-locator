use actix_web::{Responder, web};
use sqlx::{PgPool, query_as};

use crate::models::partners::Partner;

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
