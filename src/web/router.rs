use axum::http::{HeaderName, HeaderValue};
use axum::{Router, routing::get};
use tower_http::compression::CompressionLayer;
use tower_http::set_header::SetResponseHeaderLayer;

use crate::{
    service::{self, webview::fallback},
    state::AppState,
};

pub async fn creat_router(state: AppState) -> anyhow::Result<Router> {
    let router = Router::new()
        .merge(view_router())
        .nest("/api", service::api::api_router())
        .layer(axum::extract::DefaultBodyLimit::max(20 * 1024 * 1024)) // 全局上传限制 20MB
        .with_state(state);
    Ok(router)
}

/// 处理 / 下的路由 html界面相关
fn view_router() -> Router<AppState> {
    Router::new()
        .route("/", get(service::webview::index::home_view))
        .route("/favicon.svg", get(service::webview::favicon))
        .route("/{*path}", get(fallback))
        .layer(
            CompressionLayer::new()
                .br(true)
                .gzip(true)
                .quality(tower_http::compression::CompressionLevel::Precise(4)),
        )
        .layer(SetResponseHeaderLayer::overriding(
            HeaderName::from_static("cross-origin-opener-policy"),
            HeaderValue::from_static("same-origin"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            HeaderName::from_static("cross-origin-embedder-policy"),
            HeaderValue::from_static("credentialless"),
        ))
}
