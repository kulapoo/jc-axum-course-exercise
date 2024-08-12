use ctx::Ctx;
use log::log_request;
use model::ModelController;
use serde_json::json;
use tokio::net::TcpListener;
use axum::{extract::{Path, Query}, http::{Method, Uri}, middleware, response::{Html, IntoResponse, Response}, routing::{get, get_service}, Json, Router};
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use uuid::Uuid;
use web::{mw_auth, routes_login};


pub use crate::error::{Error, Result};

mod error;
mod web;
mod model;
mod ctx;
mod log;

#[tokio::main]
async fn main() -> Result<()>  {

    let mc = ModelController::new().await?;

    let route_apis = web::routes_tickets::routes(mc.clone())
        .route_layer(middleware::from_fn(mw_auth::mw_require_auth));

    let routes_all = Router::new()
        .merge(routes_hello())
        .merge(routes_login::routes())
        .nest("/api", route_apis)
        .layer(middleware::map_response(main_response_mapper))
        .layer(middleware::from_fn_with_state(
			mc.clone(),
			web::mw_auth::mw_ctx_resolver,
		))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    // #region start_server
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

	println!("->> LISTENING on {:?}\n", listener.local_addr());
	axum::serve(listener, routes_all.into_make_service())
		.await
		.unwrap();
    // #endregion start_server
    Ok(())
}


async fn main_response_mapper(
    ctx: Option<Ctx>,
	uri: Uri,
	req_method: Method,
	res: Response,
) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");

    let uuid = Uuid::new_v4();

    let service_error = res.extensions().get::<Error>();
    let client_status_error = service_error.map(|se| se.client_status_and_error());

    let error_response =
        client_status_error
            .as_ref()
            .map(|(status_code, client_error)| {
                let client_error_body = json!({
                    "error": {
                        "type": client_error.as_ref(),
                        "req_uuid": uuid.to_string(),
                    }
                });

                println!("   ->> - client_error_body {client_error_body}");

                (*status_code, Json(client_error_body)).into_response()
            });

    let client_error = client_status_error.unzip().1;

    log_request(uuid, req_method, uri, ctx, service_error, client_error).await.unwrap();

    println!();
    error_response.unwrap_or(res)
}

#[derive(Debug, serde::Deserialize)]
struct HelloParams {
    name: Option<String>,

}


fn routes_static() -> Router {
    Router::new()
        .nest_service("/", get_service(ServeDir::new("./")))
}

fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2))
}

async fn handler_hello(params: Query<HelloParams>) -> impl IntoResponse {
    let Query(params) = params;
    let name = params.name.as_deref().unwrap_or("world");

    println!("->> {:<12} - handler_hello", "HANDLER");
    Html(format!("Hello <strong>{name}</strong>"))
}

async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello2", "HANDLER");
    Html(format!("Hello <strong>{name}</strong>"))
}