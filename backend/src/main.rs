use backend::TestApi;
use poem::{EndpointExt, Route, Server, listener::TcpListener, middleware::Cors};
use poem_openapi::OpenApiService;

#[tokio::main]
async fn main() {
    let api_service = OpenApiService::new(TestApi, "evelynn.place-backend", "1.0")
        .server("https://localhost:3000");
    let ui = api_service.swagger_ui();

    let app = Route::new()
        .nest("/api", api_service)
        .nest("/docs", ui)
        .with(Cors::new());

    let _ = Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await;
}
