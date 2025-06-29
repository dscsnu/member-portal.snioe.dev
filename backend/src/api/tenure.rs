use poem::{Result, error::InternalServerError, web::Data};
use poem_openapi::{ApiResponse, Object, OpenApi, payload::Json};
use serde::Deserialize;
use uuid::Uuid;

use super::{ApiTags, ErrorResponse};
use crate::{AppState, middleware::JwtAuth, models::Tenure};

#[derive(Debug, Deserialize, Object)]
pub struct CreateTenureRequest {
    pub year: i32,
    pub is_active: bool,
}

#[derive(ApiResponse)]
enum CreateTenureResponse {
    #[oai(status = 201)]
    Created(Json<Tenure>),

    #[oai(status = 403)]
    Forbidden(Json<ErrorResponse>),

    #[oai(status = 409)]
    Conflict(Json<ErrorResponse>),
}

#[derive(Debug, Deserialize, Object)]
pub struct PutTenureRequest {
    pub id: Uuid,
    pub year: i32,
    pub is_active: bool,
}

#[derive(ApiResponse)]
enum PutTenureResponse {
    #[oai(status = 200)]
    Put(Json<Tenure>),

    #[oai(status = 403)]
    Forbidden(Json<ErrorResponse>),

    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),

    #[oai(status = 409)]
    Conflict(Json<ErrorResponse>),
}

#[derive(Debug, Deserialize, Object)]
pub struct DeleteTenureRequest {
    pub id: Uuid,
}

#[derive(ApiResponse)]
enum DeleteTenureResponse {
    #[oai(status = 200)]
    Deleted(Json<Uuid>),

    #[oai(status = 403)]
    Forbidden(Json<ErrorResponse>),

    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),

    #[oai(status = 409)]
    Conflict(Json<ErrorResponse>),
}

pub struct TenureApi;
#[OpenApi(tag = "ApiTags::Tenure")]
impl TenureApi {
    #[oai(path = "/tenure", method = "get")]
    async fn get_all_tenures(&self, pool: Data<&AppState>) -> Result<Json<Vec<Tenure>>> {
        let tenures: Vec<Tenure> = sqlx::query_as!(
            Tenure,
            r#"
                SELECT id, year, is_active
                FROM tenure
                ORDER BY year DESC
            "#
        )
        .fetch_all(&*pool.db)
        .await
        .map_err(InternalServerError)?;

        Ok(Json(tenures))
    }

    #[oai(path = "/tenure", method = "post")]
    async fn create_tenure(
        &self,
        auth: JwtAuth,
        pool: Data<&AppState>,
        data: Json<CreateTenureRequest>,
    ) -> Result<CreateTenureResponse> {
        if !auth.0.has_permission("member-portal.tenure.create") {
            return Ok(CreateTenureResponse::Forbidden(Json(ErrorResponse {
                code: "UNAUTHORIZED".to_string(),
                message: "Insufficient permissions".to_string(),
                details: Some("Missing Permission(s): [member-portal.tenure.create]".to_string()),
            })));
        }

        let mut tx = pool.db.begin().await.map_err(InternalServerError)?;

        let year_exists = sqlx::query_scalar!(
            r#"
                SELECT EXISTS (
                    SELECT 1 FROM tenure
                    WHERE year = $1
                )
            "#,
            data.year
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(InternalServerError)?;

        if year_exists.unwrap_or(false) {
            tx.rollback().await.map_err(InternalServerError)?;
            return Ok(CreateTenureResponse::Conflict(Json(ErrorResponse {
                code: "DUPLICATE_YEAR".to_string(),
                message: format!("A tenure for year {} already exists", data.year),
                details: None,
            })));
        }

        if data.is_active {
            let has_other_active_tenure = sqlx::query_scalar!(
                r#"
                    SELECT EXISTS (
                        SELECT 1 FROM tenure
                        WHERE is_active = TRUE
                    )
                "#,
            )
            .fetch_one(&mut *tx)
            .await
            .map_err(InternalServerError)?;

            if has_other_active_tenure.unwrap_or(false) {
                tx.rollback().await.map_err(InternalServerError)?;
                return Ok(CreateTenureResponse::Conflict(Json(ErrorResponse {
                    code: "INVALID ACTION".to_string(),
                    message: "There can not be multiple active tenures at once".to_string(),
                    details: Some("Set the current active tenure to inactive first".to_string()),
                })));
            }
        }

        let tenure = sqlx::query_as!(
            Tenure,
            r#"
               INSERT INTO tenure (year, is_active)
               VALUES ($1, $2)
               RETURNING id, year, is_active 
            "#,
            data.year,
            data.is_active,
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(InternalServerError)?;

        tx.commit().await.map_err(InternalServerError)?;

        Ok(CreateTenureResponse::Created(Json(tenure)))
    }

    #[oai(path = "/tenure", method = "put")]
    async fn put_tenure(
        &self,
        auth: JwtAuth,
        pool: Data<&AppState>,
        data: Json<PutTenureRequest>,
    ) -> Result<PutTenureResponse> {
        if !auth.0.has_permission("member-portal.tenure.update") {
            return Ok(PutTenureResponse::Forbidden(Json(ErrorResponse {
                code: "UNAUTHORIZED".to_string(),
                message: "Insufficient permissions".to_string(),
                details: Some("Missing Permission(s): [member-portal.tenure.update]".to_string()),
            })));
        }

        let mut tx = pool.db.begin().await.map_err(InternalServerError)?;

        let tenure_exists = sqlx::query_scalar!(
            r#"
                SELECT EXISTS (
                    SELECT 1 FROM tenure
                    WHERE id = $1
                )
            "#,
            data.id
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(InternalServerError)?;

        if !tenure_exists.unwrap_or(false) {
            tx.rollback().await.map_err(InternalServerError)?;
            return Ok(PutTenureResponse::NotFound(Json(ErrorResponse {
                code: "TENURE_NOT_FOUND".to_string(),
                message: "Tenure not found".to_string(),
                details: None,
            })));
        }

        let year_taken_by_other = sqlx::query_scalar!(
            r#"
                SELECT EXISTS (
                    SELECT 1 FROM tenure
                    WHERE year = $1 AND id != $2
                )
            "#,
            data.year,
            data.id
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(InternalServerError)?;

        if year_taken_by_other.unwrap_or(false) {
            tx.rollback().await.map_err(InternalServerError)?;
            return Ok(PutTenureResponse::Conflict(Json(ErrorResponse {
                code: "DUPLICATE_YEAR".to_string(),
                message: format!("Another tenure already exists for year {}", data.year),
                details: None,
            })));
        }

        if data.is_active {
            let has_other_active_tenure = sqlx::query_scalar!(
                r#"
                    SELECT EXISTS(
                        SELECT 1 FROM tenure
                        WHERE is_active = TRUE AND id != $1
                    )
                "#,
                data.id,
            )
            .fetch_one(&mut *tx)
            .await
            .map_err(InternalServerError)?;

            if has_other_active_tenure.unwrap_or(false) {
                tx.rollback().await.map_err(InternalServerError)?;
                return Ok(PutTenureResponse::Conflict(Json(ErrorResponse {
                    code: "INVALID ACTION".to_string(),
                    message: "There can not be multiple active tenures at once".to_string(),
                    details: None,
                })));
            }
        }

        let tenure = sqlx::query_as!(
            Tenure,
            r#"
                UPDATE tenure
                SET year = $2, is_active = $3
                WHERE id = $1
                RETURNING id, year, is_active
            "#,
            data.id,
            data.year,
            data.is_active
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(InternalServerError)?;

        tx.commit().await.map_err(InternalServerError)?;

        Ok(PutTenureResponse::Put(Json(tenure)))
    }

    #[oai(path = "/tenure", method = "delete")]
    async fn delete_tenure(
        &self,
        auth: JwtAuth,
        pool: Data<&AppState>,
        data: Json<DeleteTenureRequest>,
    ) -> Result<DeleteTenureResponse> {
        if !auth.0.has_permission("member-portal.tenure.delete") {
            return Ok(DeleteTenureResponse::Forbidden(Json(ErrorResponse {
                code: "UNAUTHORIZED".to_string(),
                message: "Insufficient permissions".to_string(),
                details: Some("Missing Permission(s): [member-portal.tenure.delete]".to_string()),
            })));
        }

        let mut tx = pool.db.begin().await.map_err(InternalServerError)?;

        let tenure = sqlx::query!(
            r#"
                SELECT id, is_active FROM tenure
                WHERE id = $1
            "#,
            data.id
        )
        .fetch_optional(&mut *tx)
        .await
        .map_err(InternalServerError)?;

        if tenure.is_none() {
            tx.rollback().await.map_err(InternalServerError)?;
            return Ok(DeleteTenureResponse::NotFound(Json(ErrorResponse {
                code: "TENURE_NOT_FOUND".to_string(),
                message: "Tenure not found".to_string(),
                details: None,
            })));
        }

        let tenure = tenure.unwrap();

        if tenure.is_active {
            let other_active_exists = sqlx::query_scalar!(
                r#"
                    SELECT EXISTS (
                        SELECT 1 FROM tenure
                        WHERE is_active = TRUE AND id != $1
                    )
                "#,
                tenure.id
            )
            .fetch_one(&mut *tx)
            .await
            .map_err(InternalServerError)?;

            if !other_active_exists.unwrap_or(false) {
                tx.rollback().await.map_err(InternalServerError)?;
                return Ok(DeleteTenureResponse::Conflict(Json(ErrorResponse {
                    code: "LAST_ACTIVE_TENURE".to_string(),
                    message: "Cannot delete the only active tenure".to_string(),
                    details: Some(
                        "Set another tenure to active before deleting this one".to_string(),
                    ),
                })));
            }
        }

        sqlx::query!(
            r#"
                DELETE FROM tenure
                WHERE id = $1
            "#,
            data.id
        )
        .execute(&mut *tx)
        .await
        .map_err(InternalServerError)?;

        tx.commit().await.map_err(InternalServerError)?;

        Ok(DeleteTenureResponse::Deleted(Json(data.id)))
    }
}
