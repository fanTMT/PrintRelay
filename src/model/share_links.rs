use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use time::{Duration, OffsetDateTime};

#[derive(Debug, FromRow, Clone, Serialize, Deserialize)]
pub struct ShareLinks {
    pub id: String,        // UUID
    pub file_path: String, // 上传的临时文件
    pub original_filename: Option<String>,
    pub used: i32,          // 0=未用, 1=已用
    pub expires_at: String, // 过期时间，Unix 时间戳
    pub created_at: String, // 创建时间，Unix 时间戳
}

impl Default for ShareLinks {
    fn default() -> Self {
        Self {
            id: String::new(),
            file_path: String::new(),
            original_filename: None,
            used: 0,
            expires_at: (OffsetDateTime::now_utc().unix_timestamp() + 24 * 60 * 60).to_string(),
            created_at: time::OffsetDateTime::now_utc()
                .saturating_add(Duration::hours(8))
                .unix_timestamp()
                .to_string(),
        }
    }
}
