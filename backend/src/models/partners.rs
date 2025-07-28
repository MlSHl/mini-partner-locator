use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Serialize, FromRow)]
pub struct Partner {
    pub id: i32,
    pub name: String,
    pub email: Option<String>,
    pub website_url: Option<String>,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Deserialize)]
pub struct NewPartner {
    pub name: String,
    pub email: Option<String>,
    pub website_url: Option<String>,
    pub country_ids: Vec<i32>,
}

#[derive(Deserialize)]
pub struct UpdatePartner {
    pub name: Option<String>,
    pub email: Option<String>,
    pub website_url: Option<String>,
}
