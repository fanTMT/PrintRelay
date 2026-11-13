use axum::{Router, routing::get};

use crate::{
    service::{self, webview::fallback},
    state::AppState,
};

pub async fn creat_router(state: AppState) -> anyhow::Result<Router> {
    let router = Router::new()
        .merge(view_router())
        .nest("/api", api_router())
        .layer(axum::extract::DefaultBodyLimit::max(20 * 1024 * 1024)) // 全局上传限制 20MB
        .with_state(state);
    Ok(router)
}

fn view_router() -> Router<AppState> {
    Router::new()
        .route("/", get(service::webview::homeview::home_view))
        .route("/favicon.ico", get(service::webview::favicon))
        .route("/assets/{*path}", get(fallback))
}

fn api_router() -> Router<AppState> {
    Router::new().route(
        "/print-records",
        get(service::webview::apiv1::print_records),
    )
}
