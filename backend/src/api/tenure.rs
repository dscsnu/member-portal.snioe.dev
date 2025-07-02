use poem::{Result, error::InternalServerError, web::Data};
use poem_openapi::{ApiResponse, Object, OpenApi, payload::Json};
use serde::Deserialize;
use uuid::Uuid;

use super::{ApiTags, ErrorResponse};
use crate::{AppState, JwtAuth, models::Tenure, require_permissions};

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
    async fn get_all_tenures(&self, state: Data<&AppState>) -> Result<Json<Vec<Tenure>>> {
        let tenures = Tenure::get_all(&*state.db)
            .await
            .map_err(InternalServerError)?;

        Ok(Json(tenures))
    }

    #[oai(path = "/tenure", method = "post")]
    async fn create_tenure(
        &self,
        auth: JwtAuth,
        state: Data<&AppState>,
        data: Json<CreateTenureRequest>,
    ) -> Result<CreateTenureResponse> {
        require_permissions!(
            all,
            auth,
            ["member-portal.tenure.create"],
            CreateTenureResponse::Forbidden
        );

        let mut tx = state.db.begin().await.map_err(InternalServerError)?;

        let year_exists = Tenure::is_year_taken_by_other(&mut *tx, data.year.clone(), None)
            .await
            .map_err(InternalServerError)?;

        if year_exists {
            tx.rollback().await.map_err(InternalServerError)?;
            return Ok(CreateTenureResponse::Conflict(Json(ErrorResponse {
                code: "DUPLICATE_YEAR".to_string(),
                message: format!("A tenure for year {} already exists", data.year),
                details: None,
            })));
        }

        if data.is_active {
            let has_active_tenure = Tenure::has_active_tenure(&mut *tx, None)
                .await
                .map_err(InternalServerError)?;

            if has_active_tenure {
                tx.rollback().await.map_err(InternalServerError)?;
                return Ok(CreateTenureResponse::Conflict(Json(ErrorResponse {
                    code: "INVALID ACTION".to_string(),
                    message: "There can not be multiple active tenures at once".to_string(),
                    details: Some("Set the current active tenure to inactive first".to_string()),
                })));
            }
        }

        let tenure = Tenure::create(&mut *tx, data.year.clone(), data.is_active.clone())
            .await
            .map_err(InternalServerError)?;

        tx.commit().await.map_err(InternalServerError)?;

        Ok(CreateTenureResponse::Created(Json(tenure)))
    }

    #[oai(path = "/tenure", method = "put")]
    async fn put_tenure(
        &self,
        auth: JwtAuth,
        state: Data<&AppState>,
        data: Json<PutTenureRequest>,
    ) -> Result<PutTenureResponse> {
        require_permissions!(
            all,
            auth,
            ["member-portal.tenure.update"],
            PutTenureResponse::Forbidden
        );

        let mut tx = state.db.begin().await.map_err(InternalServerError)?;

        let tenure_exists = Tenure::exists_by_id(&mut *tx, data.id)
            .await
            .map_err(InternalServerError)?;

        if !tenure_exists {
            tx.rollback().await.map_err(InternalServerError)?;
            return Ok(PutTenureResponse::NotFound(Json(ErrorResponse {
                code: "TENURE_NOT_FOUND".to_string(),
                message: "Tenure not found".to_string(),
                details: None,
            })));
        }

        let year_taken_by_other =
            Tenure::is_year_taken_by_other(&mut *tx, data.year, Some(data.id))
                .await
                .map_err(InternalServerError)?;

        if year_taken_by_other {
            tx.rollback().await.map_err(InternalServerError)?;
            return Ok(PutTenureResponse::Conflict(Json(ErrorResponse {
                code: "DUPLICATE_YEAR".to_string(),
                message: format!("Another tenure already exists for year {}", data.year),
                details: None,
            })));
        }

        if data.is_active {
            let has_other_active_tenure = Tenure::has_active_tenure(&mut *tx, Some(data.id))
                .await
                .map_err(InternalServerError)?;

            if has_other_active_tenure {
                tx.rollback().await.map_err(InternalServerError)?;
                return Ok(PutTenureResponse::Conflict(Json(ErrorResponse {
                    code: "INVALID ACTION".to_string(),
                    message: "There can not be multiple active tenures at once".to_string(),
                    details: None,
                })));
            }
        }

        let tenure = Tenure::update(&mut *tx, data.id, data.year, data.is_active)
            .await
            .map_err(InternalServerError)?;

        tx.commit().await.map_err(InternalServerError)?;

        Ok(PutTenureResponse::Put(Json(tenure)))
    }

    #[oai(path = "/tenure", method = "delete")]
    async fn delete_tenure(
        &self,
        auth: JwtAuth,
        state: Data<&AppState>,
        data: Json<DeleteTenureRequest>,
    ) -> Result<DeleteTenureResponse> {
        require_permissions!(
            all,
            auth,
            ["member-portal.tenure.delete"],
            DeleteTenureResponse::Forbidden
        );

        let mut tx = state.db.begin().await.map_err(InternalServerError)?;

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
            let other_active_exists = Tenure::has_active_tenure(&mut *tx, Some(data.id))
                .await
                .map_err(InternalServerError)?;

            if !other_active_exists {
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

        Tenure::delete(&mut *tx, data.id)
            .await
            .map_err(InternalServerError)?;

        tx.commit().await.map_err(InternalServerError)?;

        Ok(DeleteTenureResponse::Deleted(Json(data.id)))
    }
}
