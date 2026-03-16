use axum::Router;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let serve_dir = ServeDir::new("wasm/pkg")
        .append_index_html_on_directories(true);

    let app = Router::new().fallback_service(serve_dir);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Bengal Compiler Explorer running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
