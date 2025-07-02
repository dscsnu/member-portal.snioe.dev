#[macro_export]
macro_rules! require_permissions {
    (all, $auth:expr, [$($perm:expr),+], $forbidden_variant:expr) => {{
        let mut missing = Vec::new();
        $(
            if !$auth.0.has_permission($perm) {
                missing.push($perm);
            }
        )+

        if !missing.is_empty() {
            return Ok($forbidden_variant(poem_openapi::payload::Json(crate::api::ErrorResponse {
                code: "UNAUTHORIZED".to_string(),
                message: "Insufficient permissions".to_string(),
                details: Some(format!("Missing Permission(s): {:?}", missing)),
            })))
        }
    }};

    (any, $auth:expr, [$($perm:expr),+], $forbidden_variant:expr) => {{
        let required = [$($perm),*];

        if !required.iter().any(|p| $auth.0.has_permission(p)) {
            return Ok($forbidden_expr(poem_openapi::payload::Json(crate::api::ErrorResponse {
                code: "UNAUTHORIZED".to_string(),
                message: "Insufficient permissions".to_string(),
                details: Some(format!("Requires at least one of: {:?}", required)),
            })));
        }
    }};
}
