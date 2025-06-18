use super::ApiTags;
use crate::{middleware::JwtAuth, models::claims::Claims};

use poem_openapi::{
    Object, OpenApi,
    payload::{Json, PlainText},
};

#[derive(Object)]
struct EchoClaimsResponse {
    claims: String,
}

pub struct TestApi;
#[OpenApi(tag = "ApiTags::Test")]
impl TestApi {
    #[oai(path = "/test/echo-claims", method = "get")]
    async fn echo_claims(&self, auth: JwtAuth) -> Json<Claims> {
        Json(auth.0)
    }

    #[oai(path = "/test/ping", method = "get")]
    async fn ping(&self) -> PlainText<String> {
        PlainText("pong".to_string())
    }
}
