use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Serialize, FromRow)]
pub struct Partner {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub website_url: String,
    pub created_at: NaiveDateTime,
}

#[derive(Deserialize)]
pub struct NewPartner {
    pub name: String,
    pub email: String,
    pub website_url: String,
    pub country_ids: Vec<i32>,
}
