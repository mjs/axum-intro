# Intro to Axum
### Menno Finlay-Smits
#### `<hello@menno.io>`

---

## Axum?

- A leading web framework for Rust
- Built on Tokio, Hyper and Tower
- From the Tokio project
	- Only works with Tokio

`https://github.com/tokio-rs/axum`

---

## Features

- Routing
- Request param and body parsing
- Response generation
- Compatible with Tower middleware
	- Timeouts
	- Rate limiting
- Compatible with Tower HTTP middleware
	- Tracing
	- Compression
- Clever tricks to reduce boilerplate

---

## Not Features

- Templates
- Database access/ORM
- Background jobs
- Deployment

Similar scope to Actix Web</br>
See Rocket & Loco for all-in-one solutions

---
## Basic example

```rust
use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    // Routing
    let app = Router::new()
  	  .route("/", get(hello))
  	  .route("/bye", get(goodbye));
```

notes: Routing all done in one place

---

```rust
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
  	  .await
  	  .unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

---

```rust
async fn hello() -> &'static str {
    "Hello, World!" // Simple string response
}

async fn goodbye() -> &'static str {
    "Goodbye, World!"
}
```

---

## Response

- Anything that implements the `IntoResponse` trait can be used as a return value from a handler fn
- Various common types have implementations of `IntoResponse`
	- Raw strings (200 status)
	- (status, response) tuples

```rust
async fn hello() -> (StatusCode, &'static str) {
    (StatusCode::IM_A_TEAPOT, "Hello, World!")
}
```

---

### JSON Responses

```rust
#[derive(Serialize)]
struct Message {
    typ: String,
    text: String,
}
async fn hello() -> impl IntoResponse {
    let greeting = Message {
  	  typ: "greeting".into(),
  	  text: "Hi".into(),
    };
    axum::Json(greeting)
}
```

---

### More on Responses

- Can implement `IntoReponse` for your own types (useful for errors)
- All returns for a handler must use the same response type!
- Or return Response and call `into_response()` manually

---

## Query Params

```rust
#[derive(Deserialize)]
struct Params {
  name: String,
}

// "Query" is axum:extract::Query
async fn hello(Query(params): Query<Params>) -> String {
  format!("Hello {}", params.name)
}
```

e.g. `http://localhost:3000/?name=Foo`

notes: Query(params) on LHS is pattern matching / destructuring. See https://stackoverflow.com/questions/75419469/what-is-the-name-of-rust-function-signatures-with-type-on-left-of-parameter

---

## Path Variables

```rust
// Routing  - uses conventional path variable syntax
let app = Router::new()
  .route("/hello/:name", get(hello));

...

// "Path" is axum:extract::Path
async fn hello(Path(name): Path<String>) -> String {
  format!("Hello {}", name)
}
```

Can also serialise path variables into structs.

---

## Other Extractors

- Headers
- JSON request bodies
- Body as UTF-8 string
- Body as bytes
- Whole request

---

## Sharing State Between Requests

- E.g. for a DB connection pool
```rust
s#[derive(Clone)]
struct AppState {
  prefix: String,
}
```

---

```rust
s#[derive(Clone)]
#[tokio::main]
async fn main() {
  let state = AppState {
	  prefix: ">> ".into(),
  };

  let app = Router::new()
	  .route("/hello", get(hello))
	  .route("/bye", get(bye))
	  .with_state(state);  // pass state to Router
...
}
```

note:
- Can also pass different state to different handlers
- Can combine different states too
- Arc may be required

---

```rust
// "State" is axum:extract::State
async fn hello(State(state): State<AppState>) -> String {
  format!("{}Hello", state.prefix)
}
async fn bye(State(state): State<AppState>) -> String {
  format!("{}Bye", state.prefix)
}
```

---

## More Resources

- Official Axum docs and examples
- Feeling Axum: https://github.com/rameshovyas/feeling-axum/
	- Fantastic examples for common patterns
