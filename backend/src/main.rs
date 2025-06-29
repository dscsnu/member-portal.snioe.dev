use poem::{EndpointExt, Route, Server, listener::TcpListener, middleware::Cors};
use poem_openapi::OpenApiService;
use std::sync::Arc;

use backend::{AppState, TenureApi, create_db_pool};

#[tokio::main]
async fn main() {
    let db_pool = create_db_pool().await;
    let state = AppState {
        db: Arc::new(db_pool),
    };

    let api_service = OpenApiService::new(TenureApi, "evelynn.place-backend", "1.0")
        .server("http://localhost:3000");
    let ui = api_service.swagger_ui();

    let app = Route::new()
        .nest("/", api_service)
        .nest("/docs", ui)
        .data(state)
        .with(Cors::new());

    let _ = Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await;
}
