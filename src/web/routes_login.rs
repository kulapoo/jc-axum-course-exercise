use axum::{routing::post, Json, Router};
use serde_json::{json, Value};
use serde::Deserialize;
use tower_cookies::{Cookie, Cookies};
use crate::{web::AUTH_TOKEN, Error, Result};

pub fn routes() -> Router {
    Router::new()
        .route("/api/login", post(api_login))
}

pub async fn api_login(cookies: Cookies, payload: Json<LoginPaylod>) -> Result<Json<Value>> {
    println!("->> {:<12} - handler_api_login", "HANDLER");
    if payload.username != "demo1" || payload.pwd != "welcome" {
        return Err(Error::LoginFail);
    }

    cookies.add(Cookie::new(AUTH_TOKEN, "user-1.exp.sign"));

    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
pub struct LoginPaylod {
    pub username: String,
    pub pwd: String,
}

