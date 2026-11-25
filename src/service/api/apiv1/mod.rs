mod upload_file;
pub use upload_file::upload_file;

use std::collections::HashSet;

use axum::{Json, extract::State, http, response::IntoResponse};
use serde::Serialize;
use time::OffsetDateTime;
use tokio::process::Command;
use uuid::Uuid;

use crate::{models::print_jobs::PrintJob, state::AppState};

#[derive(Debug, Serialize)]
pub struct PrinterInfo {
    pub name: String,
    pub state: String,     // 原始状态文本，如 "idle" / "now printing"
    pub is_printing: bool, // 是否有任务在队列中
    pub is_def: bool,      // 是不是默认打印机
}
/// 获取打印机状态
pub async fn print_status(State(_state): State<AppState>) -> Json<serde_json::Value> {
    let output = Command::new("lpstat")
        .arg("-p")
        .arg("-o")
        .env("LANG", "C")
        .env("LC_ALL", "C")
        .output()
        .await
        .expect("msg");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut printers_with_jobs: HashSet<String> = HashSet::new();
    let mut result = Vec::new();

    for line in stdout.lines() {
        let line = line.trim();
        if let Some(job_printer) = line.split('-').next() {
            // 简单判断：行首部分是否是已知打印机名（稍后校验），先收集候选
            // 更稳妥的做法是等 -p 解析出名字后，再过滤 -o 行
        }
    }

    let mut printer_states: Vec<(String, String)> = Vec::new(); // (name, state_text)
    let mut job_printers: HashSet<String> = HashSet::new();

    for line in stdout.lines() {
        let line = line.trim();
        if line.starts_with("printer ") {
            // -p 输出：printer PDF_Printer is idle.
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                let name = parts[1].to_string();
                let state_text = parts[2..].join(" ");
                printer_states.push((name, state_text));
            }
        } else if !line.is_empty() {
            if let Some(printer_name) = line.split('-').next() {
                // 确保不是误匹配（比如某些行可能不是任务）
                if !printer_name.is_empty() && !printer_name.contains(' ') {
                    job_printers.insert(printer_name.to_string());
                }
            }
        }
    }

    // 合并结果
    for (name, state_text) in printer_states {
        let enabled = !state_text.to_lowercase().contains("disabled");
        let is_printing = enabled && job_printers.contains(&name);
        let is_def = false;

        result.push(PrinterInfo {
            name,
            state: state_text,
            is_printing,
            is_def,
        });
    }

    Json(serde_json::json!(result))
}

#[derive(Serialize)]
pub struct CreateShareResp {
    pub id: String,
    pub url: String,
    pub expires_at: usize,
}

/// 创建分享链接
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
    if ttl_minutes == 0 {
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

/// 获取打印记录
pub async fn print_records() -> impl axum::response::IntoResponse {
    Json(vec![
        PrintJob {
            id: Some(1),
            job_name: "document1.pdf".to_string(),
            printer_name: "Printer A".to_string(),
            file_path: "/path/to/document1.pdf".to_string(),
            status: "completed".to_string(),
            cups_job_id: Some(101),
            error_reason: None,
            created_at: "2025-06-01 10:00:00".to_string(),
            updated_at: "2025-06-01 10:05:00".to_string(),
            finished_at: Some("2025-06-01 10:05:00".to_string()),
        },
        PrintJob {
            id: Some(2),
            job_name: "image1.png".to_string(),
            printer_name: "Printer B".to_string(),
            file_path: "/path/to/image1.png".to_string(),
            status: "completed".to_string(),
            cups_job_id: Some(102),
            error_reason: None,
            created_at: "2025-06-02 14:30:00".to_string(),
            updated_at: "2025-06-02 14:35:00".to_string(),
            finished_at: Some("2025-06-02 14:35:00".to_string()),
        },
        PrintJob {
            id: Some(3),
            job_name: "report.docx".to_string(),
            printer_name: "Printer C".to_string(),
            file_path: "/path/to/report.docx".to_string(),
            status: "completed".to_string(),
            cups_job_id: Some(103),
            error_reason: None,
            created_at: "2025-06-03 09:15:00".to_string(),
            updated_at: "2025-06-03 09:20:00".to_string(),
            finished_at: Some("2025-06-03 09:20:00".to_string()),
        },
        PrintJob {
            id: Some(4),
            job_name: "presentation.pptx".to_string(),
            printer_name: "Printer D".to_string(),
            file_path: "/path/to/presentation.pptx".to_string(),
            status: "completed".to_string(),
            cups_job_id: Some(104),
            error_reason: None,
            created_at: "2025-06-04 11:45:00".to_string(),
            updated_at: "2025-06-04 11:50:00".to_string(),
            finished_at: Some("2025-06-04 11:50:00".to_string()),
        },
    ])
}
