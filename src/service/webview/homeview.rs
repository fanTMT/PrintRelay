use axum::response::Html;

use crate::state::Templates;

pub async fn home_view() -> Html<String> {
    let index_html = Templates::get("index.html").expect("找不到模板文件");
    let html_content = std::str::from_utf8(index_html.data.as_ref())
        .expect("无效的 UTF-8")
        .to_string();
    Html(html_content)
}
