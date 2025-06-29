use poem_openapi::{Object, Tags};

mod tenure;
mod test;

pub use tenure::TenureApi;
pub use test::TestApi;

#[derive(Tags)]
enum ApiTags {
    Tenure,
    Test,
}

#[derive(Object)]
pub struct ErrorResponse {
    pub code: String,
    pub message: String,
    #[oai(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}
