一、技术选型
层次 技术 说明
Web框架 axum Tokio官方出品，生态好
异步运行时 tokio
静态资源嵌入 rust-embed 编译期把Vue产物嵌入二进制
模板/回退 tower-http SPA fallback、gzip、CORS
前端 Vue3 + Vite + TS
序列化 serde / serde_json
日志 tracing + tracing-subscriber
配置 config 或 dotenvy
数据库(可选) sqlx


``` bash
lpstat -p -d 是用来查看打印机状态和默认打印机的
lpstat -W completed 列出所有已完成的打印任务
lpstat -W all -o 查看所有状态（含已取消/完成）

```