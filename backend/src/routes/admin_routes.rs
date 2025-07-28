use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use sqlx::{query, query_as, PgPool};
use crate::models::partners::{NewPartner, Partner, UpdatePartner};

#[post("/admin/partners")]
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

#[delete("/admin/partners/{id}")]
pub async fn delete_partner(
    db: web::Data<PgPool>,
    id: web::Path<i32>
) -> impl Responder {
    // x.into_inner() moves id, gotta take ownership to log
    let id = id.into_inner();

    let result = sqlx::query!("delete from partners p where p.id = $1", id)
        .execute(db.get_ref())
        .await;

    match result {
        Ok(deleted_result) => {
            if deleted_result.rows_affected() == 0 {
                eprintln!("Could not delete, partner not found with id: {id}");
                HttpResponse::NotFound().body("Partner not found")
            } else {
                println!("Partner deleted with id: {id}");
                HttpResponse::Ok().body("Partner deleted successfully")
            }
        }
        Err(e) => {
            eprintln!("Could not delete partner, {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/admin/partners/{id}")]
pub async fn get_partner_by_id(
    db: web::Data<PgPool>,
    id: web::Path<i32>
) -> impl Responder{
    let id = id.into_inner();

    let result = sqlx::query_as!(
        Partner, 
        "select id, name, email, website_url, created_at from partners where id = $1",
        id)
        .fetch_optional(db.get_ref())
        .await;

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

#[delete("admin/partners/{id}/country/{country_id}")]
pub async fn remove_country_from_partner(
    db: web::Data<PgPool>,
    path: web::Path<(i32, i32)>
) -> impl Responder {
    let (partner_id, country_id) = path.into_inner();
    
    let result = sqlx::query!(
        "delete from partner_countries where partner_id = $1 and country_id = $2",
        partner_id, country_id
    ).execute(db.get_ref()).await;

    match result {
        Ok(deleted) => {
            if deleted.rows_affected() == 0 {
                println!("No such partner and country id pair found: {partner_id}, {country_id}");
                HttpResponse::NotFound().body("No such partner and country pair found")
            } else {
                println!("Deleted the partner and country id pair: {partner_id}, {country_id}");
                HttpResponse::Ok().body("Successfully removed country from the partner")
            }
        }
        Err(e) => {
            eprintln!("Error while trying to delete partner country id pair: {partner_id}, {country_id}, error: {e}");
            HttpResponse::InternalServerError().finish()
        }
    }
}


#[patch("/admin/partners/{id}/country/{country_id}")]
pub async fn add_country_to_partner(
    db: web::Data<PgPool>,
    path: web::Path<(i32, i32)>
) -> impl Responder {
    let (partner_id, country_id) = path.into_inner();

    let result = sqlx::query!(
        "insert into partner_countries 
        (partner_id, country_id)
        values
        ($1, $2)", 
        partner_id, country_id
    ).execute(db.get_ref()).await;

    match result {
        Ok(_) => {
            println!("Inseted the partner country id pair: {partner_id}, {country_id}");
            HttpResponse::Ok().body("Added country to the partner successfully")
        }
        Err(e) => {
            if let Some(db_err) = e.as_database_error() {
                if db_err.code().as_deref() == Some("23505"){
                    return HttpResponse::Conflict().body("This country is already added to this partner");
                }
            }
            eprintln!("Failed to insert the partner country id pair: {partner_id}, {country_id}, error: {e}");
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[patch("/admin/partners/{id}")]
pub async fn update_partner(
    db: web::Data<PgPool>,
    updated_partner: web::Json<UpdatePartner>,
    path: web::Path<i32>
) -> impl Responder {
    let partner_id = path.into_inner();

    let result = sqlx::query!(
        "update partners 
        set name = coalesce($1, name), 
            email = coalesce($2, email),
            website_url = coalesce($3, website_url)
        where id = $4",
        updated_partner.name, updated_partner.email, updated_partner.website_url, partner_id)
        .execute(db.get_ref())
        .await;
        
    match result {
        Ok(updated_result) => {
            if updated_result.rows_affected() == 0{
                println!("Unable to find partner with partner_id: {partner_id}");
                HttpResponse::NotFound().body("No such partner found")
            } else {
                println!("Successfully updated the user with id: {partner_id}");
                HttpResponse::Ok().body("Partner successfully updated")
            }
        }
        Err(e) => {
            eprintln!("Error updating the user with id: {partner_id}, {e}");
            HttpResponse::InternalServerError().finish() 
        }
    }
}

