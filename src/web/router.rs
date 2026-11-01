use axum::{Router, routing::get};

use crate::{
    service::{self, webview::fallback, ws},
    state::AppState,
};

pub async fn creat_router(state: AppState) -> anyhow::Result<Router> {
    let view = Router::new()
        .route("/", get(service::webview::homeview::home_view))
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
    // .route("/{*path}", get(service::webview::handle_asset))
    // .fallback(get(service::webview::handle_asset));

    // --------------- 定义受保护路由组（需要中间件）---------------
    // let protected_routes = Router::new()
    //     .merge(index::index_router())
    //     .nest("/api", api::api_router()) // /api/xxx 除了 /api/auth 之外的路由
    //     .route("/api/upload", post(hander::upload_handler)) // 上传接口
    //     .layer(from_fn_with_state(
    //         state.clone(),
    //         auth_middleware::auth_middleware,
    //     ));
    // --------------- 合并路由 + 全局配置 ---------------
    let router = Router::new()
        .merge(view)
        // .merge(protected_routes) // 受保护路由（有中间件）
        // .route("/ws", axum::routing::any(ws::ws_handler))
        .layer(axum::extract::DefaultBodyLimit::max(20 * 1024 * 1024)) // 全局上传限制 20MB
        .with_state(state);
    Ok(router)
}
