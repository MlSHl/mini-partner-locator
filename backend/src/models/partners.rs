use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::models::countries::Country;

#[derive(Serialize, FromRow)]
pub struct Partner {
    pub id: i32,
    pub name: String,
    pub email: Option<String>,
    pub website_url: String,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Serialize)]
pub struct PartnerDetails {
    pub id: i32,
    pub name: String,
    pub email: Option<String>,
    pub website_url: String,
    pub created_at: Option<NaiveDateTime>,
    pub countries: Vec<Country>,
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

#[derive(Serialize)]
pub struct PartnerPerCountry {
    pub name: String,
    pub website_url: String,
    pub email: Option<String>,
}

