use crate::{
    error::{Error, Result},
    web,
};
use axum::{routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};
// use crate::Cookies;

pub fn routers() -> Router {
    Router::new().route("/api/login", post(api_login))
}

async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:<25} - api_login", "HANDLER");

    //TODO: Impl read db/auth login
    if payload.username != "demo" || payload.pwd != "test" {
        return Err(Error::LoginFail);
    }

    //TODO:  //FIX: Implment read auth-token generation/signature
    cookies.add(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign"));

    // Create success body
    let body = Json(json!({
        "result":{
            "success": true
        }
    }));

    return Ok(body);
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String,
}
