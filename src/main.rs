use axum::{
  extract::{Path, Query},
  response::{Html, IntoResponse},
  routing::{get, get_service},
  Router,
};
use serde::Deserialize;
use std::net::SocketAddr;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
  let routes_all = Router::new()
    .merge(routes_hello())
    .fallback_service(routes_static());

  // region --- Start Server
  let addr = SocketAddr::from(([127, 0, 0, 1], 8081));
  println!("Listening on {}\n", addr);
  axum::Server::bind(&addr)
    .serve(routes_all.into_make_service())
    .await
    .unwrap();
  // end region
}
// region: ---Route Hello
fn routes_hello() -> Router {
  Router::new()
    .route("/hello", get(handler_hello))
    .route("/hello2/:name", get(handler_hello2))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
  name: Option<String>,
}

// region: --- Handler Hello
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
  println!("->> {:<12} --handler_hello - {params:?}", "HANDLER");

  let name = params.name.as_deref().unwrap_or("World");
  Html(format!("Hello <strong>{name}!!!</strong>"))
}

async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
  println!("->> {:<12} --handler_hello2 - {name:?}", "HANDLER");

  Html(format!("Hello2 <strong>{name}!!!</strong>"))
}
// end region

fn routes_static() -> Router {
  Router::new().nest_service("/", get_service(ServeDir::new("./")))
}
