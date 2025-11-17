/// 应用注册器
/// 用于组装和注册整个应用的各种组件

use axum::{
    http::{HeaderValue, Method},
    routing::get,
    Router,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::{
    cors::CorsLayer,
    trace::TraceLayer,
};
use redis::Client as RedisClient;

use crate::websocket::create_socketio_server;

// 企业级日志颜色支持
use owo_colors::OwoColorize;

use crate::database::DatabaseConnection;

use code_generator_plugin::CodeGeneratorPlugin;
use config_plugin::ConfigPlugin;
use notice_plugin::NoticePlugin;

use crate::{
    app::admin::api::v1::router as admin_v1_router,
    app::auth::router as auth_router,
    app::complete_module::router as complete_router,
    app::data_scope::router as data_scope_router,
    app::dict_data::router as dict_data_router,
    app::dict_type::router as dict_type_router,
    app::file_info::router as file_info_router,
    app::log_level::router as log_level_router,
    app::login_log::router as login_log_router,
    app::opera_log::router as opera_log_router,
    app::menu::router as menu_router,
    app::monitor::router as monitor_router,
    app::permission::router as permission_router,
    app::plugin::router as plugin_router,
    app::role::router as role_router,
    app::role_permission::router as role_permission_router,
    app::task::router as task_router,
    app::user::router as user_router,
    app::user_role::router as user_role_router,
    app::dept::router as dept_router,
    common::response::api_response,
    core::SETTINGS,
};

/// 应用状态
#[derive(Clone)]
pub struct AppState {
    pub config: crate::core::Settings,
    pub redis_client: Arc<RedisClient>,
}

/// 数据库和Redis连接状态（用于需要数据库连接的路由）
pub type DatabaseState = (DatabaseConnection, Option<RedisClient>);

/// 应用注册器
pub struct AppRegistrar {
    state: AppState,
}

impl AppRegistrar {
    /// 创建新的应用注册器
    pub fn new() -> Self {
        // 初始化Redis连接
        let redis_client = Arc::new(
            redis::Client::open(SETTINGS.redis_url())
                .expect("Failed to connect to Redis")
        );

        Self {
            state: AppState {
                config: SETTINGS.clone(),
                redis_client,
            },
        }
    }

    /// 注册应用路由
    pub async fn build_router(self) -> Router {
        // 配置CORS - 允许前端域名访问
        let allowed_origins = [
            "http://127.0.0.1:8000".parse::<HeaderValue>().unwrap(),
            "http://localhost:5173".parse::<HeaderValue>().unwrap(),
        ];
        let cors = CorsLayer::new()
            .allow_origin(allowed_origins)
            .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::PATCH, Method::OPTIONS])
            .allow_headers([axum::http::header::CONTENT_TYPE, axum::http::header::AUTHORIZATION])
            .allow_credentials(true);

        // 根路径
        let root_router = Router::new()
            .route("/", get(|| async { api_response::success("FastAPI Best Architecture - Rust", "Welcome to the API") }))
            .route("/health", get(|| async { api_response::success("ok", "Service is healthy") }));

        // API v1 路由 - 创建无状态的路由器
        let mut api_v1_router = Router::new();

        // 认证相关路由 - /api/v1/auth/*
        api_v1_router = api_v1_router.nest("/api/v1/auth", auth_router::auth_routes());

        // 系统管理路由 - /api/v1/sys/*
        api_v1_router = api_v1_router.nest("/api/v1/sys/users", user_router::user_routes());
        api_v1_router = api_v1_router.nest("/api/v1/sys/roles", role_router::role_routes());
        api_v1_router = api_v1_router.nest("/api/v1/sys/menus", menu_router::menu_routes());
        api_v1_router = api_v1_router.nest("/api/v1/sys/depts", dept_router::dept_routes());
        api_v1_router = api_v1_router.nest("/api/v1/sys/permissions", permission_router::permission_routes());
        api_v1_router = api_v1_router.nest("/api/v1/sys/user-roles", user_role_router::user_role_routes());
        api_v1_router = api_v1_router.nest("/api/v1/sys/role-permissions", role_permission_router::role_permission_routes());
        // 数据权限和数据规则路由（分别注册，避免嵌套错误）
        api_v1_router = api_v1_router.nest("/api/v1/sys", data_scope_router::data_scope_routes());
        api_v1_router = api_v1_router.nest("/api/v1/sys/dict-types", dict_type_router::create_dict_type_router());
        api_v1_router = api_v1_router.nest("/api/v1/sys/dict-datas", dict_data_router::dict_data_routes());
        api_v1_router = api_v1_router.nest("/api/v1/sys/files", file_info_router::file_info_routes());
        api_v1_router = api_v1_router.nest("/api/v1/sys/log-levels", log_level_router::log_level_routes());
        // Notice插件路由 - 需要数据库连接
        let db_for_notice = crate::database::DatabaseManager::get_connection().await.clone();
        let notice_router = NoticePlugin::create_router(db_for_notice);
        api_v1_router = api_v1_router.nest("/api/v1/sys/notices", notice_router);
        api_v1_router = api_v1_router.nest("/api/v1/sys/plugins", plugin_router::plugin_routes());
        // Config插件路由 - 需要数据库和Redis连接
        let db_for_config = crate::database::DatabaseManager::get_connection().await.clone();
        let redis_for_config = crate::database::redis::RedisManager::get_connection().await
            .expect("Failed to get Redis connection for Config plugin");
        let config_router = ConfigPlugin::create_router(db_for_config, redis_for_config);
        api_v1_router = api_v1_router.nest("/api/v1/sys/configs", config_router);

        // 其他路由（无/sys前缀）
        api_v1_router = api_v1_router.nest("/api/v1", admin_v1_router());
        api_v1_router = api_v1_router.nest("/api/v1", complete_router::complete_routes());
        api_v1_router = api_v1_router.nest("/api/v1", task_router::task_routes());
        // 日志相关路由 - /api/v1/logs/login/* 和 /api/v1/logs/opera/*
        api_v1_router = api_v1_router.nest("/api/v1/logs/login", login_log_router::login_log_routes());
        api_v1_router = api_v1_router.nest("/api/v1/logs/opera", opera_log_router::opera_log_routes());

        // 监控路由（现在是无状态的，因为使用全局数据库连接），挂载到 /api/v1/monitors/*
        let redis_client = Some((*self.state.redis_client).clone());
        let monitor_router = monitor_router::monitor_routes(redis_client);
        api_v1_router = api_v1_router.nest("/api/v1", monitor_router);

        // 代码生成器插件路由 - 需要数据库连接
        let db = crate::database::DatabaseManager::get_connection().await.clone();
        let codegen_router = CodeGeneratorPlugin::create_router(db);
        api_v1_router = api_v1_router.nest("/api/v1/generates", codegen_router);

        // 初始化 Socket.IO 服务器（使用完整的 WebSocket 实现）
        let (socketio_layer, _io) = create_socketio_server();

        // 构建完整路由 - 应用CORS和追踪层
        let mut app = Router::new()
            .merge(root_router)
            .merge(api_v1_router)
            .layer(cors);

        // 应用JWT认证中间件（在 Socket.IO 之前，这样 Socket.IO 不会被拦截）
        app = app.layer(axum::middleware::from_fn_with_state(
                self.state.clone(),
                crate::middleware::jwt_auth_middleware::middleware
            ));

        // Debug 模式下添加请求日志中间件
        if self.state.config.debug_mode {
            app = app.layer(axum::middleware::from_fn(
                crate::middleware::request_log_middleware::middleware
            ));
        }

        // 挂载 Socket.IO 层（放在最后，这样它不会被 JWT 中间件拦截）
        app = app.layer(socketio_layer);

        app
            .layer({
                // 根据 debug_mode 配置 HTTP 请求追踪
                if self.state.config.debug_mode {
                    // Debug 模式：记录详细的请求和响应信息
                    TraceLayer::new_for_http()
                        .make_span_with(
                            tower_http::trace::DefaultMakeSpan::new()
                                .include_headers(true)   // 显示请求头
                                .level(tracing::Level::DEBUG),
                        )
                        .on_request(
                            tower_http::trace::DefaultOnRequest::new()
                                .level(tracing::Level::INFO)  // 显示请求开始
                        )
                        .on_response(
                            tower_http::trace::DefaultOnResponse::new()
                                .level(tracing::Level::INFO)   // 显示响应详情
                                .include_headers(false)        // 不显示响应头（太长）
                                .latency_unit(tower_http::LatencyUnit::Millis),
                        )
                        .on_failure(
                            tower_http::trace::DefaultOnFailure::new()
                                .level(tracing::Level::ERROR)
                        )
                } else {
                    // 普通模式：最简化日志，几乎不记录 HTTP 详情
                    TraceLayer::new_for_http()
                        .make_span_with(
                            tower_http::trace::DefaultMakeSpan::new()
                                .include_headers(false)
                                .level(tracing::Level::TRACE),  // 只在 TRACE 级别记录 span，实际上不显示
                        )
                        .on_response(
                            tower_http::trace::DefaultOnResponse::new()
                                .level(tracing::Level::TRACE)  // 只在 TRACE 级别记录
                                .include_headers(false),
                        )
                        .on_failure(
                            tower_http::trace::DefaultOnFailure::new()
                                .level(tracing::Level::ERROR)  // 错误时记录
                        )
                }
            })
    }

    /// 启动服务器
    pub async fn start(self) {
        let addr = SocketAddr::from(([127, 0, 0, 1], 8000));

        // 企业级配置信息展示（仅开发环境）
        self.print_configuration();

        let router = self.build_router().await;

        // 启动 HTTP 服务器
        Self::start_server(addr, router).await;
    }

    /// 打印配置信息
    fn print_configuration(&self) {
        if !self.state.config.is_dev() {
            return;
        }

        println!("{}", "─".repeat(80));
        println!("{}", format!("{:^80}", "系统配置").bold());
        println!("{}", "─".repeat(80));

        // 数据库配置
        println!("{}", "  数据库配置".cyan());
        println!("      类型:        {:?}", self.state.config.database_type);
        println!("      主机:        {}", self.state.config.database_host);
        println!("      端口:        {}", self.state.config.database_port);
        println!("      数据库:      {}", self.state.config.database_name);
        println!("      连接池大小:  {}", self.state.config.database_pool_size);

        // Redis 配置
        println!("\n{}", "  Redis 配置".cyan());
        println!("      主机:        {}", self.state.config.redis_host);
        println!("      端口:        {}", self.state.config.redis_port);
        println!("      数据库:      {}", self.state.config.redis_database);

        // 应用配置
        println!("\n{}", "  应用配置".cyan());
        println!("      CORS:        {}", if self.state.config.middleware_cors { "已启用" } else { "已禁用" });
        println!("      API 文档:    {}", self.state.config.docs_url);

        println!("{}", "─".repeat(80));
        println!();
    }

    /// 启动服务器
    async fn start_server(addr: SocketAddr, router: Router) {
        println!("{}", "─".repeat(80));
        println!("{}", format!("{:^80}", "正在启动服务器...").bold());
        println!("{}", "─".repeat(80));

        let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

        println!("{}", format!("  绑定地址:  http://{}", addr).green());
        println!("{}", "  服务器启动中...".to_string().green());
        println!("{}", "─".repeat(80));
        println!();

        // axum 0.8 的启动方式
        axum::serve(listener, router)
            .await
            .expect("Server failed to start");
    }
}

impl Default for AppRegistrar {
    fn default() -> Self {
        Self::new()
    }
}
