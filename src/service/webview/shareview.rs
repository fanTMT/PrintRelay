use axum::extract::path;

pub async fn share_view(path: path::Path<String>) -> impl axum::response::IntoResponse {
    format!("share view: {}", path.to_string())
}
