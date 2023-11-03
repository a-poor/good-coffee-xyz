use std::path::PathBuf;
use axum::{routing::get, Router};
use tower_http::services::ServeDir;
use maud::{html, Markup, DOCTYPE};


async fn hello_world() -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                title { "Hello, world!" }
                link rel="stylesheet" href="/static/global.css";
            }
            body {
                h1 { "Hello, world!" }
                p { "Welcome to my website." }
            }
        }
    }
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .nest_service("/static", ServeDir::new(PathBuf::from("static")))
        ;

    Ok(router.into())
}
