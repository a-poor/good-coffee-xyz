use std::path::PathBuf;
use axum::{routing::get, Router};
use tower_http::services::ServeDir;

mod util;
mod pages;
mod components;

use pages::*;
// use components::*;


#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(index))
        .nest_service("/static", ServeDir::new(PathBuf::from("static")))
        ;

    Ok(router.into())
}
