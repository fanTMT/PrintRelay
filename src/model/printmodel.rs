use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use time::OffsetDateTime;

#[derive(Debug, FromRow, Clone, Serialize, Deserialize)]
pub struct PrintJobs {
    pub id: i64, // SQLx 通常使用 i64 而不是 uint
    pub job_name: String,
    pub printer_name: String,
    pub file_path: String,
    pub status: String,
    pub cups_job_id: String,
    // SQLx 没有内置的自动时间戳字段，需要手动处理
    pub created_at: String,
    pub updated_at: String,
}
impl Default for PrintJobs {
    fn default() -> Self {
        Self {
            id: 0,
            job_name: String::new(),
            printer_name: String::new(),
            file_path: String::new(),
            status: "等待".to_string(),
            cups_job_id: String::new(),
            created_at: get_now().unwrap().to_string(),
            updated_at: get_now().unwrap().to_string(),
        }
    }
}

pub fn get_now() -> anyhow::Result<OffsetDateTime> {
    Ok(time::OffsetDateTime::now_local()?)
}
