use serde::Serialize;
use sqlx::FromRow;

#[derive(Serialize, FromRow)]
pub struct Country {
    pub id: i32,
    pub name: String,
}
