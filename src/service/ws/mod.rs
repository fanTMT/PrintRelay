use crate::state::AppState;
use axum::{
    extract::{State, WebSocketUpgrade},
    response::Response,
};
use axum_signal::{JsonCodec, MessageContext, WsHub, WsHubConfig, serve_hub};
use serde::{Deserialize, Serialize};

pub async fn ws_handler(ws: WebSocketUpgrade, State(_state): State<AppState>) -> Response {
    ws.on_upgrade(move |socket| async move {
        let config = WsHubConfig::default();
        // 如果需要把 state 传入 hub，可以在 PrintHub::new(state) 中处理
        serve_hub(socket, PrintHub, &config).await
    })
}

#[derive(Deserialize)]
pub struct ClientMessage {
    pub action: String,
    pub content: String,
}

#[derive(Serialize)]
pub enum ServerResponse {
    Ok(String),
    Error(String),
}

pub struct PrintHub;

impl WsHub for PrintHub {
    type Codec = JsonCodec; // 使用 JSON 编码
    type InMessage = ClientMessage;
    type OutMessage = ServerResponse;

    async fn on_message(
        &self,
        msg: Self::InMessage,
        ctx: MessageContext<Self::OutMessage, Self::Codec>,
    ) {
        match msg.action.as_str() {
            "print" => {
                ctx.broadcast(ServerResponse::Ok(format!("任务已下发: {}", msg.content)))
                    .await
            }
            "status" => ctx.unicast(ServerResponse::Ok("在线".into())).await,
            _ => ctx.unicast(ServerResponse::Error("未知操作".into())).await,
        }
    }
}
