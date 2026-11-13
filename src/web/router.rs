use axum::{Router, routing::get};

use crate::{
    service::{self, webview::fallback},
    state::AppState,
};

pub async fn creat_router(state: AppState) -> anyhow::Result<Router> {
    let view = Router::new()
        .route("/", get(service::webview::homeview::home_view))
        .route(
            "/share/{uuid}",
            get(service::webview::shareview::share_view),
        )
        .nest("/api", service::api::api_router())
        .route(
            "/partials/system",
            get(service::webview::partials::system_partial),
        )
        .route(
            "/partials/printer",
            get(service::webview::partials::printer_partial),
        )
        .route(
            "/partials/history",
            get(service::webview::partials::history_partial),
        )
        .route("/{*path}", get(fallback));
    let router = Router::new()
        .merge(view)
        // .merge(protected_routes) // 受保护路由（有中间件）
        // .route("/ws", axum::routing::any(ws::ws_handler))
        .layer(axum::extract::DefaultBodyLimit::max(20 * 1024 * 1024)) // 全局上传限制 20MB
        .with_state(state);
    Ok(router)
}
