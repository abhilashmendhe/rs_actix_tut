use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct GameModel {
    pub id: Uuid,
    pub field_name: String, 
    pub address: String,

    pub date: chrono::DateTime<Utc>,
    pub created_at: Option<chrono::DateTime<Utc>>,
    pub updated_at: Option<chrono::DateTime<Utc>>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateGameSchema {
    pub field_name: String,
    pub address: String,
    pub date: chrono::DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateGameSchema {
    pub field_name: String,
    pub address: String,
    pub date: chrono::DateTime<Utc>,
}

