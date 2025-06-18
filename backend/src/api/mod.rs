use poem_openapi::Tags;

mod test;
pub use test::TestApi;

#[derive(Tags)]
enum ApiTags {
    Test,
}
