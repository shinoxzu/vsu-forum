mod handlers;
mod models;
mod services;

use axum::{
    routing::get,
    Router
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/ok", get(ok_handler))
        .route("/hello", get(hello_handler))
        .route("/bye", get(bye_handler));

    let listener = tokio::net::TcpListener::bind("localhost:5050").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn ok_handler() -> &'static str {
    "OK"
}

async fn hello_handler() -> &'static str {
    "Hello World"
}

async fn bye_handler() -> &'static str {
    "bye"
}
