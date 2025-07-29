use sqlx::PgPool;

use crate::models::partners::Partner;

pub async fn fetch_partner(db: &PgPool, id: i32) -> sqlx::Result<Option<Partner>> {
    sqlx::query_as!(Partner, 
        "select id, name, email, website_url, created_at from partners where id = $1",
        id).fetch_optional(db).await
}
