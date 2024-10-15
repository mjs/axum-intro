use axum::{extract::State, routing::get, Router};

#[derive(Clone)]
struct AppState {
    prefix: String,
}

#[tokio::main]
async fn main() {
    let state = AppState {
        prefix: ">> ".into(),
    };

    // Routing
    let app = Router::new()
        .route("/hello", get(hello))
        .route("/bye", get(bye))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

// "State" is axum:extract::State
async fn hello(State(state): State<AppState>) -> String {
    format!("{}Hello", state.prefix)
}
async fn bye(State(state): State<AppState>) -> String {
    format!("{}Bye", state.prefix)
}
