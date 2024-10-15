use axum::{response::IntoResponse, routing::get, Json, Router};
use serde::Serialize;

#[tokio::main]
async fn main() {
    // Routing
    let app = Router::new().route("/", get(hello));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn hello() -> impl IntoResponse {
    let greeting = Message {
        typ: "greeting".into(),
        text: "Hi".into(),
    };
    Json(greeting)
}

#[derive(Serialize)]
struct Message {
    typ: String,
    text: String,
}
