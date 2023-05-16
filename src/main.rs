use axum::{
  http::{Method, Uri},
  middleware,
  response::{IntoResponse, Response},
  routing::get_service,
  Json, Router,
};
use ctx::Ctx;
use serde_json::json;
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use uuid::Uuid;

use crate::{log::log_request, model::ModelController};

pub use self::error::{Error, Result};

mod ctx;
mod error;
mod log;
mod model;
mod web;

#[tokio::main]
async fn main() -> Result<()> {
  let mc = ModelController::new().await?;

  let routes_apis = web::routes_ticket::routes(mc.clone())
    .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

  let routes_all = Router::new()
    .merge(web::routes_login::routes())
    .nest("/api", routes_apis)
    .layer(middleware::map_response(main_response_mapper))
    .layer(middleware::from_fn_with_state(
      mc.clone(),
      web::mw_auth::mw_ctx_resolver,
    ))
    .layer(CookieManagerLayer::new())
    .fallback_service(routes_static());

  // region:    ---  Start Server
  let addr = SocketAddr::from(([127, 0, 0, 1], 8081));
  println!("Listening on {}\n", addr);
  axum::Server::bind(&addr)
    .serve(routes_all.into_make_service())
    .await
    .unwrap();
  // enregion: ---  Start Server

  Ok(())
}

async fn main_response_mapper(
  ctx: Option<Ctx>,
  uri: Uri,
  req_method: Method,
  res: Response,
) -> Response {
  println!("->> {:<12} -- main_response_mapper", "RES_MAPPER");
  let uuid = Uuid::new_v4();

  // -- Get the eventual response error
  let service_error = res.extensions().get::<Error>();
  let client_status_error = service_error.map(|se| se.client_status_and_error());

  // -- If client error, build the new response
  let error_response = client_status_error.as_ref().map(|(sc, ce)| {
    let client_error_body = json!({
      "error": {
        "type": ce.as_ref(),
        "req_uuid": uuid.to_string(),
      }
    });
    println!("  ->> client_error_body: {client_error_body}");
    //Build new response
    (*sc, Json(client_error_body)).into_response()
  });

  //  Build and log the server log line
  let client_error = client_status_error.unzip().1;
  log_request(uuid, req_method, uri, ctx, service_error, client_error).await;
  println!();
  error_response.unwrap_or(res)
}

fn routes_static() -> Router {
  Router::new().nest_service("/", get_service(ServeDir::new("./")))
}
