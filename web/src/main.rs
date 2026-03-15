use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(serve_frontend))
        .nest_service("/wasm", ServeDir::new("wasm/pkg"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Bengal Godbolt running on http://localhost:3000");
    println!("Using WASM-based compilation (client-side)");
    axum::serve(listener, app).await.unwrap();
}

async fn serve_frontend() -> impl IntoResponse {
    Html(include_str!("../frontend.html"))
}
