use poem_openapi::Object;
use serde::{Deserialize, Serialize};
use sqlx::{Executor, Postgres};
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize, Object)]
pub struct Tenure {
    pub id: Uuid,
    pub year: i32,
    pub is_active: bool,
}

impl Tenure {
    pub async fn get_all<'e, E>(executor: E) -> Result<Vec<Self>, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as!(
            Tenure,
            r#"
                SELECT id, year, is_active
                FROM tenure
                ORDER BY year DESC
            "#,
        )
        .fetch_all(executor)
        .await
    }

    pub async fn create<'e, E>(executor: E, year: i32, is_active: bool) -> Result<Self, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as!(
            Tenure,
            r#"
                INSERT INTO tenure (year, is_active)
                VALUES ($1, $2)
                RETURNING id, year, is_active
            "#,
            year,
            is_active
        )
        .fetch_one(executor)
        .await
    }

    pub async fn update<'e, E>(
        executor: E,
        id: Uuid,
        year: i32,
        is_active: bool,
    ) -> Result<Self, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        sqlx::query_as!(
            Tenure,
            r#"
                UPDATE tenure
                SET year = $2, is_active = $3
                WHERE id = $1
                RETURNING id, year, is_active
            "#,
            id,
            year,
            is_active
        )
        .fetch_one(executor)
        .await
    }

    pub async fn delete<'e, E>(executor: E, id: Uuid) -> Result<Uuid, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        let record = sqlx::query!(
            r#"
                DELETE FROM tenure
                WHERE id = $1
                RETURNING id
            "#,
            id
        )
        .fetch_one(executor)
        .await?;

        Ok(record.id)
    }

    pub async fn exists_by_year<'e, E>(executor: E, year: i32) -> Result<bool, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        let exists = sqlx::query_scalar!(
            r#"
                SELECT EXISTS(
                    SELECT 1 FROM tenure
                    WHERE year = $1
                )
            "#,
            year
        )
        .fetch_one(executor)
        .await?
        .unwrap_or(false);

        Ok(exists)
    }

    pub async fn exists_by_id<'e, E>(executor: E, id: Uuid) -> Result<bool, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        let exists = sqlx::query_scalar!(
            r#"
                SELECT EXISTS(
                    SELECT 1 FROM tenure
                    WHERE id = $1
                )
            "#,
            id
        )
        .fetch_one(executor)
        .await?
        .unwrap_or(false);

        Ok(exists)
    }

    pub async fn has_active_tenure<'e, E>(
        executor: E,
        exclude_id: Option<Uuid>,
    ) -> Result<bool, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        let exists = sqlx::query_scalar!(
            r#"
                SELECT EXISTS(
                    SELECT 1 FROM tenure
                    WHERE is_active = TRUE AND id != $1
                )
            "#,
            exclude_id
        )
        .fetch_one(executor)
        .await?
        .unwrap_or(false);

        Ok(exists)
    }

    pub async fn is_year_taken_by_other<'e, E>(
        executor: E,
        year: i32,
        exclude_id: Option<Uuid>,
    ) -> Result<bool, sqlx::Error>
    where
        E: Executor<'e, Database = Postgres>,
    {
        let exists: bool = sqlx::query_scalar!(
            r#"
                SELECT EXISTS(
                    SELECT 1 FROM tenure
                    WHERE year = $1 AND id != $2
                )
            "#,
            year,
            exclude_id
        )
        .fetch_one(executor)
        .await?
        .unwrap_or(false);

        Ok(exists)
    }
}
