use poem_openapi::Tags;

mod tenure;
mod test;

pub use tenure::TenureApi;
pub use test::TestApi;

#[derive(Tags)]
enum ApiTags {
    Tenure,
    Test,
}
