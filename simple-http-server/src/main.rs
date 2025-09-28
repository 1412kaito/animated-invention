use axum::{
    http::StatusCode,
    response::{Json, IntoResponse},
    routing::get,
    Router,
};
use serde::Serialize;
use std::net::SocketAddr;


#[derive(Serialize)]
struct HelloResponse {
    message: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

async fn hello() -> Json<HelloResponse> {
    Json(HelloResponse {
        message: "Hello world!".to_string(),
    })
}

async fn not_found() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Json(ErrorResponse {
            error: "Route not found".to_string(),
        }),
    )
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/v1", get(hello))
        .fallback(not_found);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
