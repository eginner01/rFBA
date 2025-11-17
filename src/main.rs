/// 主应用入口
/// 启动 FastAPI Best Architecture - Rust 服务器
use fastapi_best_architecture_rust::core::registrar::AppRegistrar;
use fastapi_best_architecture_rust::core::SETTINGS;
use tracing::{error, info};

#[tokio::main]
async fn main() {
    // 初始化日志
    init_logging();

    info!("启动 FastAPI Best Architecture - Rust");
    info!("版本: 0.1.0");
    info!("运行环境: {}", SETTINGS.environment);

    // 初始化数据库
    info!("正在初始化数据库...");
    if let Err(err) = fastapi_best_architecture_rust::database::DatabaseManager::init().await {
        error!("数据库初始化失败: {}", err);
        std::process::exit(1);
    }

    // 初始化Redis
    info!("正在初始化 Redis...");
    let redis_url = SETTINGS.redis_url();
    if let Err(err) = fastapi_best_architecture_rust::database::redis::RedisManager::init(&redis_url) {
        error!("Redis 初始化失败: {}", err);
        std::process::exit(1);
    }

    // 初始化插件系统
    info!("正在初始化插件系统...");
    if let Err(err) = fastapi_best_architecture_rust::app::plugin::init_plugins().await {
        error!("插件系统初始化失败: {}", err);
        error!("   这不是致命错误，将继续运行（不加载插件）...");
        // 插件初始化失败不退出，因为可能只是没有插件目录
    }

    // 启动应用
    let app = AppRegistrar::new();
    app.start().await;
}

/// 初始化日志系统
fn init_logging() {
    use tracing_subscriber::fmt::time::ChronoLocal;
    
    // 根据 debug_mode 动态调整日志级别
    let log_level = if SETTINGS.debug_mode {
        "debug"  // Debug 模式：显示所有详细信息
    } else {
        SETTINGS.log_level.as_str()  // 普通模式：使用配置的级别
    };
    
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| {
            // 构建详细的过滤器
            if SETTINGS.debug_mode {
                // Debug 模式：显示所有模块的 debug 日志，包括 sea_orm 的 SQL
                tracing_subscriber::EnvFilter::new(
                    format!(
                        "{}=debug,sea_orm=debug,sqlx=debug,tower_http=debug,axum=debug",
                        env!("CARGO_PKG_NAME").replace('-', "_")
                    )
                )
            } else {
                // 普通模式：精简日志，隐藏 SQL 和 HTTP 详情
                tracing_subscriber::EnvFilter::new(
                    format!(
                        "{}={},sea_orm=warn,sqlx=warn,tower_http=warn,axum=info",
                        env!("CARGO_PKG_NAME").replace('-', "_"),
                        log_level
                    )
                )
            }
        });

    // 根据环境和 debug_mode 选择不同的格式
    if SETTINGS.is_dev() {
        if SETTINGS.debug_mode {
            // Debug 模式：显示详细信息（文件、行号、模块）
            tracing_subscriber::fmt()
                .with_env_filter(env_filter)
                .with_timer(ChronoLocal::new("%Y-%m-%d %H:%M:%S".to_string()))
                .with_target(true)      // 显示模块路径
                .with_thread_ids(false)
                .with_file(true)        // 显示文件名
                .with_line_number(true) // 显示行号
                .with_level(true)
                .with_ansi(true)
                .compact()
                .init();
            info!("调试模式已启用 - 显示详细日志（路由、SQL、响应时间）");
        } else {
            // 普通模式：精简清晰
            tracing_subscriber::fmt()
                .with_env_filter(env_filter)
                .with_timer(ChronoLocal::new("%Y-%m-%d %H:%M:%S".to_string()))
                .with_target(false)
                .with_thread_ids(false)
                .with_file(false)
                .with_line_number(false)
                .with_level(true)
                .with_ansi(true)
                .compact()
                .init();
        }
    } else {
        // 生产环境：JSON 格式，便于日志聚合
        tracing_subscriber::fmt()
            .with_env_filter(env_filter)
            .with_timer(ChronoLocal::new("%Y-%m-%d %H:%M:%S".to_string()))
            .json()
            .init();
    }

    info!("日志系统已初始化 (级别: {}, 调试模式: {})", log_level, SETTINGS.debug_mode);
}
