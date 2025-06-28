use poem::{Result, error::InternalServerError, web::Data};
use poem_openapi::{OpenApi, payload::Json};

use super::ApiTags;
use crate::{AppState, middleware::JwtAuth, models::Tenure};

pub struct TenureApi;
#[OpenApi(tag = "ApiTags::Tenure")]
impl TenureApi {
    #[oai(path = "/tenure", method = "get")]
    async fn get_all_tenure(&self, pool: Data<&AppState>) -> Result<Json<Vec<Tenure>>> {
        let tenures = sqlx::query_as!(
            Tenure,
            r#"SELECT id, year, is_active FROM tenure ORDER BY year DESC"#
        )
        .fetch_all(&*pool.db)
        .await
        .map_err(|e| InternalServerError(e))?;

        Ok(Json(tenures))
    }
}
