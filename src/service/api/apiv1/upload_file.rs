use std::{collections::HashMap, path::Path, sync::LazyLock};

use axum::{Json, extract::State, response::IntoResponse};
use axum_extra::extract::Multipart;
use serde_json::json;
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};
use uuid::Uuid;

use crate::{psrerror::PsrError, state::AppState};

static SUPPORTED_EXT: LazyLock<HashMap<&'static str, bool>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert(".jpg", true);
    m.insert(".png", true);
    m.insert(".pdf", true);
    m.insert(".txt", true);
    m
});

pub async fn upload_file(
    State(state): State<AppState>,
    mut m: Multipart,
) -> Result<impl IntoResponse, PsrError> {
    // 上传临时文件夹
    let upload_path = "./uploads";
    // 创建文件夹
    fs::create_dir_all(upload_path)
        .await
        .map_err(|e| PsrError::NotFound(e.to_string()))?;

    while let Some(field) = m
        .next_field()
        .await
        .map_err(|e| PsrError::NotFound(e.to_string()))?
    {
        // 1. 立即提取文件名并转换为 owned String
        let name = if let Some(o_name) = field.file_name() {
            o_name.to_string()
        } else {
            continue;
        };
        // 验证文件名
        if !is_valid_filename(&name) {
            return Err(PsrError::NotFound("错误文件名".to_string()));
        }
        // 验证文件类型
        if !is_allowed_file_type(&name) {
            return Err(PsrError::NotFound("错误文件名".to_string()));
        }
        // // 限制文件大小 (10MB)
        // let max_size = state.config.read().unwrap().app.upload_max * 1024 * 1024;
        // if f.size_hint().0 > max_size {
        //     return Err(PsrError::NotFound("错误文件名".to_string()));
        // }
        let max_size = state.config.read().unwrap().app.upload_max * 1024 * 1024;
        let data = field
            .bytes()
            .await
            .map_err(|e| PsrError::NotFound(format!("读取文件数据失败: {}", e)))?;
        // 检查实际文件大小
        if data.len() > max_size {
            return Err(PsrError::NotFound("".to_string()));
        }

        // 生成安全的文件名
        let file_extension = &name.split('.').next_back().unwrap_or("bin");
        let safe_filename = format!("{}.{}", Uuid::new_v4(), file_extension); // 文件名
        let save_path = format!("{}/{}", upload_path, safe_filename); // 带文件名的路径
        // // 限制文件大小 (10MB)
        // let max_size = state.config.read().unwrap().app.upload_max * 1024 * 1024;
        // if f.size_hint().0 > max_size {
        //     return Err(PsrError::NotFound("错误文件名".to_string()));
        // }
        let mut file = File::create(save_path)
            .await
            .map_err(|e| PsrError::NotFound("".to_string()))?;

        file.write_all(&data)
            .await
            .map_err(|e| PsrError::NotFound("".to_string()))?;
    }

    println!("----- {:#?}", m);
    let value = json!({
        "code": 200,
        "success": true,
        "payload": {
            "features": [
                "serde",
                "json"
            ],
            "homepage": null
        }
    });
    Ok(Json(value))
}

// 检查文件名是否安全
fn is_valid_filename(filename: &str) -> bool {
    if filename.is_empty() || filename.len() > 255 {
        return false;
    }

    let path = Path::new(filename);
    let components: Vec<_> = path.components().collect();

    if components.len() != 1 {
        return false;
    }

    !filename.contains(|c: char| {
        c == '/'
            || c == '\\'
            || c == ':'
            || c == '*'
            || c == '?'
            || c == '"'
            || c == '<'
            || c == '>'
            || c == '|'
    })
}

// 检查文件类型是否允许 - 修复扩展名匹配逻辑
fn is_allowed_file_type(filename: &str) -> bool {
    filename
        .rsplit_once('.') // 获取最后一个点后的部分（如 "file.pdf" -> ("file", "pdf")）
        .map(|(_, ext)| format!(".{}", ext.to_lowercase())) // 转换为 ".pdf" 格式
        .and_then(|ext| SUPPORTED_EXT.get(ext.as_str())) // 查找带点的小写扩展名
        .copied()
        .unwrap_or(false)
}
