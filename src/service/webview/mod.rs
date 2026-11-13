pub mod homeview;
pub mod partials;
pub mod shareview;

use crate::state::Templates;
use axum::{
    extract::Path,
    http,
    response::{IntoResponse, Response},
};

// pub async fn favicon() -> impl IntoResponse {
//     let icon = Templates::get("favicon.ico").expect("找不到模板文件");
//     Response::builder()
//         .status(200)
//         .header(http::header::CONTENT_TYPE, "image/x-icon")
//         .body(axum::body::Body::from(icon.data))
//         .unwrap()
// }

// pub async fn handle_asset(
//     axum::extract::Path(filename): axum::extract::Path<String>,
// ) -> impl IntoResponse {
//     // 打印调试信息，帮助确认路径问题
//     info!("请求的资源路径: {}", filename);
//     println!("请求的资源路径: {}", filename);
//
//     // let possible_paths = [&filename, &format!("static/{}", filename)];
//     let possible_paths = [&filename, &filename];
//     // 尝试所有可能的路径
//     for &path in &possible_paths {
//         // println!("尝试查找资源: {}", path);
//         debug!("尝试查找资源: {}", path);
//         println!("尝试查找资源: {}", path);
//
//         if let Some(content) = Templates::get(&format!("icons/{}", path)) {
//             // 确定MIME类型
//             let mime_type = mime_guess::from_path(path).first_or_octet_stream();
//             // 构建响应头
//             let mut headers = HeaderMap::new();
//             headers.insert(
//                 http::header::CONTENT_TYPE,
//                 mime_type.as_ref().parse().unwrap_or_else(|_| {
//                     // 处理MIME类型解析失败的情况
//                     mime_guess::mime::APPLICATION_OCTET_STREAM
//                         .as_ref()
//                         .parse()
//                         .unwrap()
//                 }),
//             );
//             // 添加缓存头（可选，根据需要调整）
//             headers.insert(
//                 http::header::CACHE_CONTROL,
//                 "public, max-age=86400".parse().unwrap(),
//             );
//             return (http::StatusCode::OK, headers, content.data).into_response();
//         }
//     }
//     // 如果所有路径都找不到资源
//     // println!("资源未找到: {:?}", possible_paths);
//     debug!("资源未找到: {:?}", possible_paths);
//     (
//         http::StatusCode::NOT_FOUND,
//         [(http::header::CONTENT_TYPE, "text/plain; charset=utf-8")],
//         "请求的资源不存在",
//     )
//         .into_response()
// }

/// 判断路径是否是「静态资源」（带扩展名）
fn has_extension(path: &str) -> bool {
    std::path::Path::new(path).extension().is_some()
}

pub fn asset_response(path: &str) -> Option<Response> {
    let key = path.trim_start_matches('/');
    if key.is_empty() {
        return None;
    }
    let file = Templates::get(key)?;
    let mime = mime_guess::from_path(key).first_or_octet_stream();

    let cache = match key {
        "index.html" => "no-cache",
        _ => "public, max-age=31536000, immutable",
    };

    Some(
        Response::builder()
            .status(http::StatusCode::OK)
            .header(http::header::CONTENT_TYPE, mime.as_ref())
            .header(http::header::CACHE_CONTROL, cache)
            .body(axum::body::Body::from(file.data.into_owned()))
            .unwrap(),
    )
}
pub async fn fallback(Path(path): Path<String>) -> Response {
    eprintln!("[fallback] path = {:?}", path);

    if let Some(resp) = asset_response(&path) {
        eprintln!("[fallback] HIT: {}", path);
        return resp;
    }
    eprintln!("[fallback] MISS: {}", path);

    if has_extension(&path) {
        return http::StatusCode::NOT_FOUND.into_response();
    }
    asset_response("index.html").unwrap_or_else(|| http::StatusCode::NOT_FOUND.into_response())
}
