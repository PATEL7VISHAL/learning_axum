#![allow(unused)]

use axum::{
    extract::{Path, Query},
    middleware,
    response::{Html, IntoResponse, Response},
    routing::{get, get_service},
    Router,
};
use serde::Deserialize;
use std::net::SocketAddr;
use tower_cookies::{CookieManagerLayer, Cookies};
use tower_http::services::ServeDir;

mod ctx;
mod error;
mod model;
mod web;

#[tokio::main]
async fn main() {
    // Initialise ModelController
    let mc = model::ModelController::new().await.unwrap();
    let routes_tickets = web::routes_tickets::routes(mc.clone())
        .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

    let routes = Router::new().merge(
        web::routes_hello::routes()
            .merge(web::routes_login::routers())
            .nest("/api", routes_tickets)
            .layer(middleware::map_response(main_response_mapper))
            .layer(middleware::from_fn_with_state(
                mc.clone(),
                web::mw_auth::mw_ctx_resolver,
            ))
            .layer(CookieManagerLayer::new())
            .fallback_service(routes_static()),
    );

    // region: --Start server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    println!("LISTENING on : {:?}", listener.local_addr());
    axum::serve(listener, routes.into_make_service())
        .await
        .unwrap();
}

async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<25} - main_response_mapper", "RES_MAPPER");
    println!();
    res
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}
