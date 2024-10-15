use axum::{extract::Path, routing::get, Router};

#[tokio::main]
async fn main() {
    // Routing
    let app = Router::new().route("/hello/:name", get(hello));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

// "Path" is axum:extract::Path
async fn hello(Path(name): Path<String>) -> String {
    format!("Hello {}", name)
}
