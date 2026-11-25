use anyhow::Context;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

use crate::utils::ip::get_local_ip;

static NAME: &str = "printerAxum";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)] // 自动生成版本/帮助信息
pub struct Args {
    /// 端口（可选）
    #[arg(short = 'p', long = "port", default_value_t = 3000)]
    port: u32,
    /// 公网IP（可选）
    #[arg(short = 'i', long = "ip")]
    ip: Option<String>,
}

/// 配置
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    /// 软件相关
    pub app: AppConfig,
    /// 数据库
    pub database: DatabaseConfig,
    /// 日志
    pub log: LogConfig,
}

/// 软件配置
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AppConfig {
    pub ip: String,
    /// 启动端口
    pub port: u32,
    /// 公网iP（可空）
    pub public_ip: Option<String>,
    /// 最大单个上传文件大小
    pub upload_max: usize,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DatabaseConfig {
    /// 数据库地址（可MySQL）
    pub url: String,
    /// 数据库最大连接数
    pub max_connections: u32,
    /// 数据库超时时间
    pub timeout: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LogConfig {
    /// 日志等级
    pub level: String,
    /// 日志文件保存位置
    pub file_path: String,
}

/// 初始化配置 从配置文件读取
pub fn init_config() -> anyhow::Result<Config> {
    let config_path = default_config_path()?;
    // 检查配置文件是否存在，不存在则生成默认配置
    if !config_path.exists() {
        println!("配置文件不存在，生成默认配置...");
        generate_default_config(&config_path, Args::parse())?;
    }
    let config = load_config(config_path)?;
    // println!("加载配置------- {:#?}", config);
    // println!("{:<10}{:?}MB", "最大上传文件大小:", config.app.upload_max);
    println!("{:<10}{:?}", "日志文件目录:", config.log.file_path);
    Ok(config)
}

/// 将配置保存到文件（覆盖原文件）
pub fn save_config<P: AsRef<std::path::Path>>(config: &Config, path: P) -> anyhow::Result<()> {
    // 将 Config 结构体序列化为格式化的 TOML 字符串（带缩进和注释）
    let toml_str =
        toml::to_string_pretty(config).context("配置序列化失败（结构体可能包含不支持的类型）")?;
    // 写入文件（覆盖原有内容）
    fs::write(&path, toml_str)
        .with_context(|| format!("无法写入配置到文件: {}", path.as_ref().display()))?;
    Ok(())
}

/// 从文件路径读取并解析配置
pub fn load_config<P: AsRef<Path>>(path: P) -> anyhow::Result<Config> {
    // 读取文件内容
    let config_str = fs::read_to_string(&path)
        .with_context(|| format!("无法读取配置文件: {}", path.as_ref().display()))?;

    // 解析 TOML 为 Config 结构体
    let config = toml::from_str(&config_str)
        .with_context(|| format!("配置文件格式错误: {}", path.as_ref().display()))?;

    Ok(config)
}

/// 获取默认配置文件路径
pub fn default_config_path() -> anyhow::Result<std::path::PathBuf> {
    // 使用 dirs 库获取用户配置目录（跨平台：Linux 是 ~/.config，Windows 是 AppData/Roaming 等）
    let config_dir = dirs::config_dir()
        .context("无法获取系统配置目录")?
        .join(NAME); // 应用专属目录
    // 确保目录存在
    fs::create_dir_all(&config_dir).context("无法创建配置目录")?;
    // 配置文件路径
    Ok(config_dir.join("config.toml"))
}

/// 生成默认配置文件
pub fn generate_default_config<P: AsRef<Path>>(
    path: P,
    Args { port, ip }: Args,
) -> anyhow::Result<()> {
    let local_ipv4 = get_local_ip().expect("获取本地IP失败！！！");
    let logspath = dirs::config_dir()
        .context("无法获取系统配置目录")?
        .join("psr/logs")
        .to_string_lossy()
        .to_string(); // 应用专属目录
    fs::create_dir_all(&logspath).context("无法创建logs目录")?;

    let default_config = Config {
        app: AppConfig {
            ip: local_ipv4.unwrap_or("localhost".to_string()),
            port,
            public_ip: ip,
            upload_max: 10,
        },
        database: DatabaseConfig {
            url: "sqlite:apps.db?mode=rwc".to_string(),
            max_connections: 20,
            timeout: 2000,
        },
        log: LogConfig {
            level: "info".to_string(),
            file_path: logspath,
        },
    };
    // 序列化为 TOML 字符串
    let toml_str = toml::to_string_pretty(&default_config)?;
    // 写入文件
    std::fs::write(&path, toml_str)
        .with_context(|| format!("无法写入默认配置到: {}", path.as_ref().display()))?;
    Ok(())
}
