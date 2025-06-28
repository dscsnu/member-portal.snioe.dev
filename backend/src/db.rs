use sqlx::{PgPool, postgres::PgPoolOptions};

use crate::CONFIG;

pub async fn create_db_pool() -> PgPool {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&CONFIG.database_url)
        .await
        .expect("Failed to connect to the database")
}
