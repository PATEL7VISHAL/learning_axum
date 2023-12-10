use axum::{
    extract::{Path, Query},
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use serde::Deserialize;

pub fn routes() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

// regoin:: -- Handler Hello
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello", "HANDLER");
    let name = params.name.as_deref().unwrap_or("all");
    return Html(format!("Hello <strong> {name}!!! </strong>"));
}
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello", "HANDLER");
    return Html(format!("Hello <strong> {name}!!! </strong>"));
}
