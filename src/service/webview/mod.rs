pub mod homeview;
pub mod partials;

use crate::state::Templates;
use axum::{
    http::{self, HeaderMap},
    response::{IntoResponse, Response},
};
use tracing::debug;

pub async fn favicon() -> impl IntoResponse {
    let icon = Templates::get("favicon.svg").expect("找不到模板文件");
    Response::builder()
        .status(200)
        .header(http::header::CONTENT_TYPE, "image/svg+xml")
        .body(axum::body::Body::from(icon.data))
        .unwrap()
}

pub async fn fallback(
    axum::extract::Path(filename): axum::extract::Path<String>,
) -> impl IntoResponse {
    debug!("请求的资源路径: {}", filename);
    // let possible_paths = [&filename, &format!("assets/{}", filename)];
    let possible_path = format!("assets/{}", filename);

    if let Some(context) = Templates::get(&possible_path) {
        let mine_type = mime_guess::from_path(&possible_path).first_or_octet_stream();
        let mut headers = HeaderMap::new();
        headers.insert(
            http::header::CONTENT_TYPE,
            mine_type.as_ref().parse().unwrap_or_else(|_| {
                // 处理MIME类型解析失败的情况
                mime_guess::mime::APPLICATION_OCTET_STREAM
                    .as_ref()
                    .parse()
                    .unwrap()
            }),
        );

        return (http::StatusCode::OK, headers, context.data).into_response();
    }
    debug!("资源未找到: {:?}", possible_path);
    (
        http::StatusCode::NOT_FOUND,
        [(http::header::CONTENT_TYPE, "text/plain; charset=utf-8")],
        "请求的资源不存在",
    )
        .into_response()
}
