use crate::{config::Config, service::jwt};
use rust_embed::RustEmbed;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct AppState {
    // 数据库连接池（Arc 确保线程安全共享）
    pub db_pool: Arc<sqlx::Pool<sqlx::Sqlite>>,
    // 全局配置（RwLock 允许多线程读写）
    pub config: Arc<RwLock<Config>>,
    // jsonwebtoken
    pub jwt_config: Arc<jwt::JwtConfig>,
}

// 资源文件嵌入（RustEmbed）
#[derive(RustEmbed)]
#[folder = "ui/dist"]
pub struct Templates;
