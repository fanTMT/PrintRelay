use psr::{create_pool, init_config, init_logger, run};
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化配置
    let config = init_config()?;
    // 初始化 log
    init_logger(&config);
    // 初始化数据库
    let pool = create_pool(&config).await;
    info!("启动 Sqlite 数据库...");
    // 启动服务
    run(pool, config).await?;
    Ok(())
    // http://192.168.31.1:3000
}
