pub mod claims;

use poem_openapi::Object;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize, Object)]
pub struct Tenure {
    pub id: Uuid,
    pub year: i32,
    pub is_active: bool,
}
