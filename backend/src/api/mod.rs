use poem_openapi::{Object, Tags};

mod tenure;

pub use tenure::TenureApi;

#[derive(Tags)]
enum ApiTags {
    Tenure,
}

#[derive(Object)]
pub struct ErrorResponse {
    pub code: String,
    pub message: String,
    #[oai(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}
