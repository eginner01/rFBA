/// 使用SQL文件初始化数据库数据
/// 严格遵循Python后端格式，不创建任何自定义数据
use sqlx::mysql::MySqlPoolOptions;
use dotenvy::dotenv;
use std::fs;
use tracing::{info, error, warn};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载环境变量
    dotenv().ok();

    // 数据库连接配置
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "mysql://root:Xuxin@6455141@127.0.0.1:3306/fba?charset=utf8mb4".to_string());

    info!("🔌 连接数据库: {}", database_url.replace("Xuxin@6455141", "****"));

    // 创建连接池
    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await?;

    info!("✅ 数据库连接成功");

    // 读取SQL文件并执行
    let sql_files = vec![
        "sql/fix_user_table.sql",
        "sql/insert_test_users.sql",
    ];

    for sql_file in sql_files {
        if fs::metadata(sql_file).is_ok() {
            info!("📄 执行 SQL 文件: {}", sql_file);

            let sql_content = fs::read_to_string(sql_file)?;

            // 分割SQL语句（处理分号分隔的多个语句）
            let statements: Vec<&str> = sql_content
                .split(';')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty() && !s.starts_with("--"))
                .collect();

            for statement in statements {
                if !statement.is_empty() && statement.len() > 10 {  // 过滤掉太短的语句
                    match sqlx::query(statement).execute(&pool).await {
                        Ok(result) => {
                            info!("✅ 影响行数: {}", result.rows_affected());
                        },
                        Err(e) => {
                            // 忽略表已存在等常见错误
                            if !e.to_string().contains("Duplicate") &&
                               !e.to_string().contains("already exists") &&
                               !e.to_string().contains("doesn't exist") {
                                error!("❌ SQL执行失败: {}", e);
                                error!("❌ 语句: {}", statement);
                            }
                        }
                    }
                }
            }

            info!("✅ SQL文件执行完成: {}", sql_file);
        } else {
            warn!("⚠️  SQL文件不存在: {}", sql_file);
        }
    }

    // 验证用户数据
    let users = sqlx::query!("SELECT id, username, nickname, status, is_superuser, del_flag FROM sys_user")
        .fetch_all(&pool)
        .await?;

    info!("\n📋 数据库用户列表:");
    info!("========================================");
    for user in users {
        info!("用户名: {}", user.username);
        info!("昵称: {}", user.nickname);
        info!("状态: {}", if user.status == 1 { "启用" } else { "禁用" });
        info!("超级用户: {}", if user.is_superuser == 1 { "是" } else { "否" });
        info!("删除标志: {}", user.del_flag);
        info!("----------------------------------------");
    }

    info!("\n✅ 数据库初始化完成!");

    Ok(())
}
