use axum::{extract::Query, routing::get, Router};
use serde::Deserialize;

#[tokio::main]
async fn main() {
    // Routing
    let app = Router::new().route("/", get(hello));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

// "Query" is axum:extract::Query
async fn hello(Query(params): Query<HelloParams>) -> String {
    format!("Hello {}", params.name)
}

#[derive(Deserialize)]
struct HelloParams {
    name: String,
}
