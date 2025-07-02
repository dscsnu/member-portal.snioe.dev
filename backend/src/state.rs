use sqlx::{PgPool, postgres::PgPoolOptions};
use std::sync::Arc;

use crate::CONFIG;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<PgPool>,
}

pub async fn create_db_pool() -> PgPool {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&CONFIG.database_url)
        .await
        .expect("Failed to connect to the database")
}
