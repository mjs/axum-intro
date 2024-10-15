use axum::{http::StatusCode, routing::get, Router};

#[tokio::main]
async fn main() {
    // Routing
    let app = Router::new()
        .route("/", get(hello))
        .route("/bye", get(goodbye));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn hello() -> (StatusCode, &'static str) {
    (StatusCode::IM_A_TEAPOT, "Hello, World!")
}

async fn goodbye() -> &'static str {
    "Goodbye, World!"
}
