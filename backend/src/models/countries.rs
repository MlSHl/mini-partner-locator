use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use strum_macros::{Display, EnumIter, EnumString};

#[derive(Serialize, FromRow)]
pub struct Country {
    pub id: i32,
    pub name: String,
    pub region: String,
}

#[derive(Serialize)]
pub struct CountryName {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Display, EnumString, EnumIter)]
#[strum(serialize_all = "UPPERCASE", ascii_case_insensitive)]
#[serde(rename_all = "UPPERCASE")]
pub enum Region {
    EMEA,
    NORAM,
    LATAM,
    APAC,
}

#[derive(Serialize)]
pub struct RegionResponse {
    pub region: Region,
}
