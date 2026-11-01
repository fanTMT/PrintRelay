use axum::response::Html;

pub async fn system_partial() -> Html<String> {
    let cups = get_cups_status().await;
    Html(format!(
        r#"<div class="system-status">
            <h3>CUPS 状态</h3>
            <pre>{cups}</pre>
        </div>"#
    ))
}

pub async fn printer_partial() -> Html<String> {
    let cpu = get_cpu_percent();

    Html(format!(
        r#"<div class="system-status">
            <h3>CPU 占用</h3>
            <p>总体使用率：{cpu:.1}%</p>
        </div>"#
    ))
}


pub async fn history_partial() -> Html<String> {
    // 1. 异步执行 lpstat 命令获取所有打印历史
    let output = tokio::process::Command::new("lpstat")
        .args(["-W", "all", "-o"])
        .output()
        .await;

    let mut rows = String::new();

    match output {
        Ok(out) if out.status.success() => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            
            // 2. 解析每一行文本
            for line in stdout.lines() {
                if line.trim().is_empty() {
                    continue;
                }

                // lpstat 的输出通常是以空格分隔的，这里简单按照连续空白字符分割
                // 预期格式: PDF_Printer-1 fan 142336 2026年11月01日 星期日 12时14分49秒
                let parts: Vec<&str> = line.split_whitespace().collect();
                
                if parts.len() >= 4 {
                    // 提取关键信息
                    let job_id = parts[0];
                    let user = parts[1];
                    let size = parts[2];
                    // 时间和日期可能包含空格（如 "2026年11月01日 星期日 12时14分49秒"），所以把后面剩下的部分合并起来
                    let date_time = parts[3..].join(" ");

                    // 3. 拼装 HTML 行
                    rows.push_str(&format!(
                        r#"<tr>
                            <td class="job-id">{}</td>
                            <td class="user">{}</td>
                            <td class="size">{}</td>
                            <td class="time">{}</td>
                        </tr>"#,
                        job_id, user, size, date_time
                    ));
                }
            }
        }
        _ => {
            // 命令执行失败时的降级显示
            rows.push_str(r#"<tr><td colspan="4" class="error">无法获取打印历史记录</td></tr>"#);
        }
    }

    // 4. 返回包含表格的 HTML 片段
    Html(format!(
        r#"<div class="system-status">
            <h3>打印历史记录</h3>
            <table class="print-history-table">
                <thead>
                    <tr>
                        <th>任务 ID</th>
                        <th>用户</th>
                        <th>大小 (字节)</th>
                        <th>提交时间</th>
                    </tr>
                </thead>
                <tbody>
                    {}
                </tbody>
            </table>
        </div>"#,
        rows
    ))
}
async fn get_cups_status() -> String {
    let out = tokio::process::Command::new("lpstat")
        .args(["-p", "-d"])
        .output()
        .await;
    match out {
        Ok(o) if o.status.success() => {
            let s = String::from_utf8_lossy(&o.stdout).to_string();
            if s.trim().is_empty() {
                "无打印机信息".into()
            } else {
                s
            }
        }
        Ok(o) => format!("lpstat 错误：{}", String::from_utf8_lossy(&o.stderr)),
        Err(e) => format!("CUPS 不可用：{e}"),
    }
}

fn get_cpu_percent() -> f32 {
    use sysinfo::System;
    let mut sys = System::new();
    sys.refresh_cpu_usage();
    std::thread::sleep(std::time::Duration::from_millis(200));
    sys.refresh_cpu_usage();
    sys.global_cpu_usage().abs()
}
