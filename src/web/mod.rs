use crate::{config, service::jwt, state::AppState, web::router::creat_router};
use std::sync::{Arc, RwLock};
use tracing::info;

mod curd;
mod router;
mod view;
mod ws;

pub async fn run(pool: sqlx::Pool<sqlx::Sqlite>, config: config::Config) -> anyhow::Result<()> {
    let state = AppState {
        // 数据库
        db_pool: Arc::new(pool),
        // 配置
        config: Arc::new(RwLock::new(config.clone())),
        // jsonwebtoken
        jwt_config: Arc::new(jwt::JwtConfig::new("ykU/3UQweBqMw9Ldxxp20JfY8G1q0b3n8zNnixbDmUtg8tDrZMwLlj84ssWws6YCjQ8lwI96FtnzZ7jVeagE2A==".to_string())),
    };
    // 格式化输出
    let ip = format!("{}:{}", config.app.ip, config.app.port);
    info!(target: "psr","web页面运行： http://{}", ip.to_string());
    // 全地址监听「方案选单」
    let address = format!("[::]:{}", config.app.port);
    info!(target: "psr","实际运行服务器 http://{}", address);
    let app = creat_router(state).await?;

    let listener = tokio::net::TcpListener::bind(address).await?;
    // 启动axum服务
    axum::serve(listener, app).await?;
    Ok(())
}
   