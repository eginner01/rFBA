/// 测试配置读取是否正确
/// 运行方式: cargo run --bin test_config
use std::env;

fn main() {
    println!("=== 配置读取测试 ===\n");

    // 加载 .env 文件
    dotenvy::dotenv().ok();

    // 读取数据库配置
    let db_type = env::var("DATABASE_TYPE").unwrap_or_default();
    let db_host = env::var("DATABASE_HOST").unwrap_or_default();
    let db_port = env::var("DATABASE_PORT").unwrap_or_default();
    let db_user = env::var("DATABASE_USER").unwrap_or_default();
    let db_password = env::var("DATABASE_PASSWORD").unwrap_or_default();
    let db_name = env::var("DATABASE_NAME").unwrap_or_default();

    println!("从 .env 文件读取的配置:");
    println!("  DATABASE_TYPE: {}", db_type);
    println!("  DATABASE_HOST: {}", db_host);
    println!("  DATABASE_PORT: {}", db_port);
    println!("  DATABASE_USER: {}", db_user);
    println!("  DATABASE_PASSWORD: {}", if db_password.is_empty() { "(空)" } else { "***" });
    println!("  DATABASE_NAME: {}", db_name);
    println!();

    // 尝试解析为 DatabaseType
    let db_type_parsed = match db_type.to_lowercase().as_str() {
        "mysql" => "MySQL ✅".to_string(),
        "postgresql" | "postgres" => "PostgreSQL ✅".to_string(),
        "sqlite" | "sqlite3" => "SQLite ✅".to_string(),
        "" => "未配置 ❌".to_string(),
        other => format!("未知类型: {} ❌", other),
    };

    println!("数据库类型解析结果: {}", db_type_parsed);
    println!();

    // 检查MySQL连接（如果配置是MySQL）
    if db_type.to_lowercase() == "mysql" {
        println!("正在测试MySQL连接...");

        // 使用 mysql 客户端测试连接（暂时未使用）
        let _connection_str = format!(
            "mysql://{}:{}@{}:{}",
            db_user,
            if db_password.is_empty() { "" } else { &db_password },
            db_host,
            db_port
        );

        println!("连接字符串: mysql://{}:{}@{}:{}", db_user, if db_password.is_empty() { "(空)" } else { "***" }, db_host, db_port);

        // 简单测试: 检查MySQL进程
        #[cfg(target_os = "windows")]
        let mysql_running = check_process_windows("mysqld.exe");

        #[cfg(target_os = "linux")]
        let mysql_running = check_process_linux("mysqld");

        #[cfg(target_os = "macos")]
        let mysql_running = check_process_macos("mysqld");

        if mysql_running {
            println!("MySQL进程: 运行中 ✅");
        } else {
            println!("MySQL进程: 未运行 ❌");
            println!("请启动MySQL服务:");
            println!("  Linux:   sudo systemctl start mysql");
            println!("  macOS:   brew services start mysql");
            println!("  Windows: 启动MySQL服务");
        }
    }

    println!("\n=== 测试完成 ===");
}

#[cfg(target_os = "windows")]
fn check_process_windows(process_name: &str) -> bool {
    use std::process::Command;
    let output = Command::new("tasklist")
        .args(["/FI", &format!("IMAGENAME eq {}", process_name)])
        .output()
        .ok();
    match output {
        Some(output) => {
            let text = String::from_utf8_lossy(&output.stdout);
            text.contains(process_name)
        }
        None => false,
    }
}

#[cfg(target_os = "linux")]
fn check_process_linux(process_name: &str) -> bool {
    use std::process::Command;
    let output = Command::new("pgrep")
        .args(&[process_name])
        .output()
        .ok();
    match output {
        Some(output) => output.status.success(),
        None => false,
    }
}

#[cfg(target_os = "macos")]
fn check_process_macos(process_name: &str) -> bool {
    use std::process::Command;
    let output = Command::new("pgrep")
        .args(&[process_name])
        .output()
        .ok();
    match output {
        Some(output) => output.status.success(),
        None => false,
    }
}
