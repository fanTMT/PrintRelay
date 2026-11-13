use axum::{Json, Router, extract::State, http, response::IntoResponse};
use serde::Serialize;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::state::AppState;

pub fn api_router() -> Router<AppState> {
    Router::new().route("/share/create", axum::routing::post(create_share_link))
}

#[derive(Serialize)]
pub struct CreateShareResp {
    pub id: String,
    pub url: String,
    pub expires_at: usize,
}

pub async fn create_share_link(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (http::StatusCode, String)> {
    let (base_url, ttl_minutes) = {
        let cfg = state.config.read().unwrap().app.clone();
        (cfg.ip, cfg.upload_max)
    };

    if base_url.trim().is_empty() {
        return Err((
            http::StatusCode::INTERNAL_SERVER_ERROR,
            "base_url 未配置".into(),
        ));
    }
    if ttl_minutes <= 0x0 {
        return Err((
            http::StatusCode::INTERNAL_SERVER_ERROR,
            "share_link_ttl_minutes 配置非法".into(),
        ));
    }

    // 2) 生成 UUID 与过期时间（统一 RFC3339 UTC）
    let id = Uuid::new_v4().to_string();
    let expires_at = (OffsetDateTime::now_utc().unix_timestamp() + 24 * 60 * 60) as usize;

    // 3) 落库：file_path/original_filename 上传前为 NULL
    sqlx::query(
        r#"INSERT INTO share_links (id, file_path, original_filename, used, expires_at)
           VALUES (?, NULL, NULL, 0, ?)"#,
    )
    .bind(&id)
    .bind(&expires_at.to_string())
    .execute(state.db_pool.as_ref())
    .await
    .map_err(|e| {
        (
            http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("创建分享链接失败: {e}"),
        )
    })?;

    // 4) 拼 URL
    let url = format!("{}/share/{}", base_url.trim_end_matches('/'), id);

    Ok(Json(CreateShareResp {
        id,
        url,
        expires_at,
    }))
}
