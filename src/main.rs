#![allow(unused)]

use crate::log::log_request;
use axum::{
    extract::{Path, Query},
    http::{Method, Uri},
    middleware,
    response::{Html, IntoResponse, Response},
    routing::{get, get_service},
    Json, Router,
};
use ctx::Ctx;
use serde::Deserialize;
use serde_json::json;
use std::net::SocketAddr;
use tower_cookies::{CookieManagerLayer, Cookies};
use tower_http::services::ServeDir;
use uuid::Uuid;

mod ctx;
mod error;
mod log;
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

async fn main_response_mapper(
    ctx: Option<Ctx>,
    uri: Uri,
    req_method: Method,
    res: Response,
) -> Response {
    println!("->> {:<25} - main_response_mapper", "RES_MAPPER");
    let uuid = Uuid::new_v4();
    // -- Get the eventual response error
    let service_error = res.extensions().get::<error::Error>();
    let client_state_error = service_error.map(|se| se.client_status_and_error());
    // -- if client error, build the new response
    let error_response = client_state_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let client_error_body = json!({
                "error":{
                    "type": client_error.as_ref(),
                    "req_uuid": uuid.to_string()
                }
            });
            println!("  ->> client_error_body: {client_error_body}");
            // Build the new response from the client_error_body
            (*status_code, Json(client_error_body)).into_response()
        });
    // TODO: Build and log the server log line.
    let client_error = client_state_error.unzip().1;
    log_request(uuid, req_method, uri, ctx, service_error, client_error).await;

    println!();
    error_response.unwrap_or(res)
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}
