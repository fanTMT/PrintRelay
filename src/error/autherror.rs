use axum::Json;
use axum::http::{HeaderValue, StatusCode, header::WWW_AUTHENTICATE};
use axum::response::IntoResponse;
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("错误的凭证: {0}")]
    WrongCredentials(String),
    #[error("缺少凭据: {0}")]
    MissingCredentials(String),
    #[error("令牌创建错误: {0}")]
    TokenCreation(String),
    #[error("无效令牌")]
    InvalidToken,
    #[error("过期令牌")]
    ExpiredToken,
    #[error("服务内部错误: {0}")]
    AnyError(#[from] anyhow::Error),
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_msg, challenge) = match self {
            AuthError::WrongCredentials(e) => (
                StatusCode::UNAUTHORIZED,
                format!("错误的凭证,{}", e),
                Some("Bearer error=\"invalid_token\""),
            ),
            AuthError::MissingCredentials(e) => (
                StatusCode::UNAUTHORIZED,
                format!("缺少凭证,{}", e),
                Some("Bearer error=\"invalid_request\""),
            ),
            AuthError::TokenCreation(e) => {
                tracing::error!("token creation failed: {e}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "令牌创建失败".to_string(),
                    None,
                )
            }
            AuthError::InvalidToken => (
                StatusCode::UNAUTHORIZED,
                "无效令牌".to_string(),
                Some("Bearer error=\"invalid_token\""),
            ),
            AuthError::ExpiredToken => (
                StatusCode::UNAUTHORIZED,
                "令牌已过期".to_string(),
                Some(
                    "Bearer error=\"invalid_token\", error_description=\"The access token expired\"",
                ),
            ),
            AuthError::AnyError(e) => {
                tracing::error!("internal error: {e:?}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "服务内部错误".to_string(),
                    None,
                )
            }
        };

        let body = Json(json!({
            "error": error_msg,
            "status": status.as_u16(),
        }));

        let mut response = (status, body).into_response();

        if let Some(challenge) = challenge {
            if let Ok(header_value) = HeaderValue::from_str(challenge) {
                response
                    .headers_mut()
                    .insert(WWW_AUTHENTICATE, header_value);
            }
        }

        response
    }
}
