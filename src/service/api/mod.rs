// 处理 /api 下的路由 后期可以拆分成多个模块，方便维护
mod apiv1;
pub use apiv1::*;
use axum::{
    Router,
    routing::{get, post},
};

use crate::state::AppState;

pub fn api_router() -> Router<AppState> {
    Router::new()
        .route("/upload", post(upload_file))
        .route("/print-records", get(print_records))
        .route("/print-status", get(print_status))
        .route("/share/create", axum::routing::post(create_share_link))
}
