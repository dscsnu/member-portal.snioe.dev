use poem::{Result, error::InternalServerError, web::Data};
use poem_openapi::{ApiResponse, Object, OpenApi, payload::Json};
use serde::Deserialize;
use uuid::Uuid;

use super::ApiTags;
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
    Forbidden(Json<String>),
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
    Forbidden(Json<String>),
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
        .map_err(|e| InternalServerError(e))?;

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
            return Ok(CreateTenureResponse::Forbidden(Json(
                "Insuffificent Permissions".to_string(),
            )));
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
        .fetch_one(&*pool.db)
        .await
        .map_err(InternalServerError)?;

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
            return Ok(PutTenureResponse::Forbidden(Json(
                "Insufficient Permissions".to_string(),
            )));
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
        .fetch_one(&*pool.db)
        .await
        .map_err(InternalServerError)?;

        Ok(PutTenureResponse::Put(Json(tenure)))
    }
}
