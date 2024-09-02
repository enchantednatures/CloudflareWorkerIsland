mod app;
mod components;

use app::shell;
use axum::{routing::get, Router};
use leptos::prelude::RenderHtml;
use tower_service::Service;
use worker::*;

fn router() -> Router {
    Router::new().route("/", get(root))
}

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    _env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    console_error_panic_hook::set_once();
    let mut response = router().call(req).await?;
    // s.set_header("content-type", "text/html")?;
    let headers = response.headers_mut();
    headers.insert("content-type", "text/html".parse().unwrap());
    Ok(response)
}

pub async fn root() -> String {
    shell().to_html()
}
