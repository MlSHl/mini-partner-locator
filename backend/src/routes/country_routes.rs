use crate::models::countries::{Country, CountryName, Region};
use actix_web::{HttpResponse, Responder, get, web};
use sqlx::PgPool;
use strum::IntoEnumIterator;

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

#[get("/countries/{region}")]
pub async fn get_countries_by_region(
    db: web::Data<PgPool>,
    path: web::Path<Region>,
) -> impl Responder {
    let region = path.into_inner().to_string();
    
    let result = sqlx::query_as!(
        CountryName,
        "select name from countries where region=$1",
        region
    )
    .fetch_all(db.get_ref())
    .await;

    match result {
        Ok(countries_response) => {
            println!("Returning countries of region: {region}");
            HttpResponse::Ok().json(countries_response)
        }
        Err(e) => {
            eprintln!("Unable to fetch countries from the region: {region}, error: {e}");
            HttpResponse::InternalServerError().finish()
        }
    }
}


#[get("/regions")]
pub async fn get_regions() -> impl Responder {
    let regions: Vec<Region> = Region::iter().collect();
    HttpResponse::Ok().json(regions)
}
