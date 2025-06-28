use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode, errors::ErrorKind};
use poem_openapi::{ApiResponse, Object, SecurityScheme, auth::Bearer, payload::Json};

use crate::CONFIG;
use crate::models::claims::Claims;

#[derive(SecurityScheme)]
#[oai(
    ty = "bearer",
    key_name = "JWT",
    key_in = "header",
    checker = "jwt_checker"
)]
pub struct JwtAuth(pub Claims);

#[derive(ApiResponse)]
pub enum JwtErrorResponse {
    #[oai(status = 401)]
    Unauthorized(Json<ErrorResponse>),

    #[oai(status = 403)]
    Forbidden(Json<ErrorResponse>),

    #[oai(status = 400)]
    BadRequest(Json<ErrorResponse>),
}

#[derive(Object)]
pub struct ErrorResponse {
    pub code: String,
    pub message: String,
    #[oai(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

async fn jwt_checker(_req: &poem::Request, bearer: Bearer) -> Result<Claims, poem::Error> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = true;
    validation.set_audience(&["authenticated"]);

    let token_data = decode::<Claims>(
        &bearer.token,
        &DecodingKey::from_secret(CONFIG.jwt_secret.as_bytes()),
        &validation,
    )
    .map_err(|e| match e.kind() {
        ErrorKind::ExpiredSignature => JwtErrorResponse::Unauthorized(Json(ErrorResponse {
            code: "JWT_EXPIRED".to_string(),
            message: "Token has expired".to_string(),
            details: None,
        })),

        ErrorKind::InvalidSignature => JwtErrorResponse::Unauthorized(Json(ErrorResponse {
            code: "JWT_INVALID".to_string(),
            message: "Invalid token signature".to_string(),
            details: None,
        })),

        _ => JwtErrorResponse::BadRequest(Json(ErrorResponse {
            code: "JWT_MALFORMED".to_string(),
            message: "Invalid token format".to_string(),
            details: Some(e.to_string()),
        })),
    })?;

    Ok(token_data.claims)
}
