use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use sqlx::{query, query_as, PgPool};
use crate::{db::partner_queries::fetch_partner, models::{countries::{Country}, partners::{NewPartner, Partner, PartnerDetails, UpdatePartner}}};

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
                HttpResponse::NotFound().json(serde_json::json!({
                    "message":"Partner not found"
                }))
            } else {
                println!("Partner deleted with id: {id}");
                HttpResponse::Ok().json(serde_json::json!({
                    "message": "Partner successfully deleted"
                }))
            }
        }
        Err(e) => {
            eprintln!("Could not delete partner, {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/admin/partners/{id}")]
pub async fn get_partner_details_by_id(
    db: web::Data<PgPool>,
    id: web::Path<i32>
) -> impl Responder{
    let id = id.into_inner();
    let fetched_partner = fetch_partner(db.get_ref(), id).await;
    
    let partner = match fetched_partner {
        Ok(Some(p)) => {
            println!("User found with id: {id}");
            p
        },
        Ok(None) => {
            eprintln!("No partner found int he database for id: {id}");
            return HttpResponse::NotFound().json(serde_json::json!({
                    "message":"Partner not found"
            }))
        },
        Err(e) => {
            eprintln!("Error while fetching partner with id: {id}, error: {e}");
            return HttpResponse::InternalServerError().finish();
        }
    };

    let fetched_countries = sqlx::query_as!(Country,
        "select c.id, c.name, c.region
        from partner_countries pc
        left join countries c
        on pc.country_id= c.id
        where pc.partner_id = $1",
        id).fetch_all(db.get_ref()).await;

    let countries = match fetched_countries {
        Ok(vec) => {
            println!("Fetched countries for the partner with id: {id}");
            vec
        },
        Err(e) => {
            eprintln!("Error while trying to fetch countries for partner with id: {id}, error: {e}");
            return HttpResponse::InternalServerError().finish();
        }
    };

    let result = PartnerDetails {
        id: partner.id,
        name: partner.name,
        email: partner.email,
        website_url: partner.website_url,
        created_at: partner.created_at,
        countries: countries,
    };

    HttpResponse::Ok().json(result) 
}

#[get("/admin/partners")]
pub async fn get_all_partners_with_countries(
    db: web::Data<PgPool>,
) -> impl Responder {
    let fetched_partners = sqlx::query_as!(
        Partner,
        "SELECT id, name, email, website_url, created_at FROM partners"
    )
    .fetch_all(db.get_ref())
    .await;

    let partners = match fetched_partners {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Failed to fetch partners: {e}");
            return HttpResponse::InternalServerError().finish();
        }
    };

    let mut enriched_partners = Vec::new();
    for partner in partners {
        let countries = sqlx::query_as!(
            Country,
            "SELECT c.id, c.name, c.region
             FROM partner_countries pc
             LEFT JOIN countries c ON c.id = pc.country_id
             WHERE pc.partner_id = $1",
            partner.id
        )
        .fetch_all(db.get_ref())
        .await;

        let countries = match countries {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Error fetching countries for partner id {}: {e}", partner.id);
                return HttpResponse::InternalServerError().finish();
            }
        };

        let result = PartnerDetails {
            id: partner.id,
            name: partner.name,
            email: partner.email,
            website_url: partner.website_url,
            created_at: partner.created_at,
            countries,
        };

        enriched_partners.push(result);
    }

    HttpResponse::Ok().json(enriched_partners)
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
                HttpResponse::Ok().json(serde_json::json!({
                    "message": "Country was added successfully"
                }))
        }
        Err(e) => {
            if let Some(db_err) = e.as_database_error() {
                if db_err.code().as_deref() == Some("23505"){
                    return HttpResponse::Conflict().json(serde_json::json!({
                        "message":"This country is already added to this partner"
                    }))
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
                HttpResponse::NotFound().json(serde_json::json!({
                    "message":"Partner not found"
                }))
            } else {
                println!("Successfully updated the user with id: {partner_id}");
                HttpResponse::Ok().json(serde_json::json!({
                    "message": "Partner successfully updated"
                }))
            }
        }
        Err(e) => {
            eprintln!("Error updating the user with id: {partner_id}, {e}");
            HttpResponse::InternalServerError().finish() 
        }
    }
}

