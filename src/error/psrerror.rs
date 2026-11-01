use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::error::autherror::AuthError;

#[derive(thiserror::Error, Debug)]
pub enum PsrError {
    #[error("not found: {0}")]
    NotFound(String),
    // 数据库问题
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),
    // 认证问题
    #[error(transparent)]
    Auth(#[from] AuthError),
}

impl IntoResponse for PsrError {
    fn into_response(self) -> Response {
        match self {
            PsrError::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()).into_response(),
            PsrError::Database(e) => {
                tracing::error!("database error: {e}");
                (StatusCode::INTERNAL_SERVER_ERROR, "服务内部错误").into_response()
            }
            PsrError::Auth(e) => e.into_response(), // 直接委托，返回 Response
        }
    }
}
