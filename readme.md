一、技术选型

|     层次     |                技术                 | 说明                      |
| :----------: | :---------------------------------: | ------------------------- |
|   Web框架    |                axum                 | Tokio官方出品，生态好     |
|  异步运行时  |                tokio                |                           |
| 静态资源嵌入 |             rust-embed              | 编译期把Vue产物嵌入二进制 |
|     前端     |          Vue3 + Vite + TS           |                           |
|    序列化    |         serde / serde\_json         |                           |
|     日志     |    tracing + tracing-subscriber     |                           |
|     配置     |          config 或 dotenvy          |                           |
|    数据库    |                sqlx                 |                           |
|  模板/回退   | tower-http SPA fallback、gzip、CORS |                           |

> cups的相关cmd

```bash
lpstat -p -d 是用来查看打印机状态和默认打印机的
lpstat -W completed 列出所有已完成的打印任务
lpstat -W all -o 查看所有状态（含已取消/完成）

```

## 待实现功能

- [x] \[leechjia\]能不能自定义端口？

已经添加配置文件设置自定义端口

- [ ] \[Daniyu\]如果能显示一共打印 了多少页，一共打印了多少面就好了（以及显示一共多少张彩色，多少张黑白）

- [ ] \[又是馒头\]打电子发票是刚需，一般都是扫码开发票，都在手机上，现在要发送到电脑上打印，能直接打印就好了，参照 lvfapiao点com，能实现每张A4纸打3张发票的功能就好了

- [ ] \[byanly\]没有明白怎么运行，我WIN11运行print\_server.exe一闪而过，啥都又没，请指教
