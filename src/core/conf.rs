/// 全局配置文件
/// 读取环境变量和配置文件

use std::collections::HashMap;

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

/// 数据库类型
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum DatabaseType {
    #[serde(alias = "mysql")]
    MySQL,
    #[serde(alias = "postgresql", alias = "postgres")]
    PostgreSQL,
    #[serde(alias = "sqlite", alias = "sqlite3")]
    SQLite,
}

impl From<String> for DatabaseType {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "mysql" => DatabaseType::MySQL,
            "postgresql" | "postgres" => DatabaseType::PostgreSQL,
            "sqlite" | "sqlite3" => DatabaseType::SQLite,
            _ => {
                eprintln!("Warning: Unknown database type '{}', using SQLite as default", s);
                DatabaseType::SQLite
            }
        }
    }
}

/// 运行环境
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
#[derive(Default)]
pub enum EnvironmentType {
    #[default]
    Dev,
    Prod,
}

impl std::fmt::Display for EnvironmentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EnvironmentType::Dev => write!(f, "dev"),
            EnvironmentType::Prod => write!(f, "prod"),
        }
    }
}


impl From<String> for EnvironmentType {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "prod" | "production" => EnvironmentType::Prod,
            _ => EnvironmentType::Dev,
        }
    }
}

/// 全局配置结构体
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct Settings {
    // ===== 环境配置 =====
    /// 运行环境
    #[serde(default = "default_environment")]
    #[serde(alias = "FBA_ENVIRONMENT")]
    pub environment: EnvironmentType,

    // ===== FastAPI/Web 配置 =====
    /// API v1 路径前缀
    #[serde(default = "default_api_v1_path")]
    #[serde(alias = "FASTAPI_API_V1_PATH", alias = "FBA_API_V1_PATH")]
    pub api_v1_path: String,
    /// 应用标题
    #[serde(default = "default_app_title")]
    #[serde(alias = "FASTAPI_TITLE", alias = "FBA_APP_TITLE")]
    pub app_title: String,
    /// 应用描述
    #[serde(default = "default_app_description")]
    #[serde(alias = "FASTAPI_DESCRIPTION", alias = "FBA_APP_DESCRIPTION")]
    pub app_description: String,
    /// API 文档 URL
    #[serde(default = "default_docs_url")]
    #[serde(alias = "FASTAPI_DOCS_URL", alias = "FBA_DOCS_URL")]
    pub docs_url: String,
    /// ReDoc URL
    #[serde(default = "default_redoc_url")]
    #[serde(alias = "FASTAPI_REDOC_URL", alias = "FBA_REDOC_URL")]
    pub redoc_url: String,
    /// OpenAPI JSON URL
    #[serde(default = "default_openapi_url")]
    #[serde(alias = "FASTAPI_OPENAPI_URL", alias = "FBA_OPENAPI_URL")]
    pub openapi_url: Option<String>,
    /// 是否启用静态文件
    #[serde(default = "default_static_files")]
    #[serde(alias = "FASTAPI_STATIC_FILES", alias = "FBA_STATIC_FILES")]
    pub static_files: bool,

    // ===== 数据库配置 =====
    /// 数据库类型
    #[serde(default = "default_database_type")]
    #[serde(alias = "DATABASE_TYPE", alias = "FBA_DATABASE_TYPE")]
    pub database_type: DatabaseType,
    /// 数据库主机
    #[serde(default = "default_database_host")]
    #[serde(alias = "DATABASE_HOST", alias = "FBA_DATABASE_HOST")]
    pub database_host: String,
    /// 数据库端口
    #[serde(default = "default_database_port")]
    #[serde(alias = "DATABASE_PORT", alias = "FBA_DATABASE_PORT")]
    pub database_port: u16,
    /// 数据库用户名
    #[serde(default = "default_database_user")]
    #[serde(alias = "DATABASE_USER", alias = "FBA_DATABASE_USER")]
    pub database_user: String,
    /// 数据库密码
    #[serde(default = "default_database_password")]
    #[serde(alias = "DATABASE_PASSWORD", alias = "FBA_DATABASE_PASSWORD")]
    pub database_password: String,
    /// 数据库名称
    #[serde(default = "default_database_name")]
    #[serde(alias = "DATABASE_NAME", alias = "FBA_DATABASE_NAME")]
    pub database_name: String,
    /// 数据库字符集
    #[serde(default = "default_database_charset")]
    #[serde(alias = "DATABASE_CHARSET", alias = "FBA_DATABASE_CHARSET")]
    pub database_charset: String,
    /// 数据库模式
    #[serde(default = "default_database_schema")]
    #[serde(alias = "DATABASE_SCHEMA", alias = "FBA_DATABASE_SCHEMA")]
    pub database_schema: String,
    /// 数据库连接池大小
    #[serde(default = "default_database_pool_size")]
    #[serde(alias = "DATABASE_POOL_SIZE", alias = "FBA_DATABASE_POOL_SIZE")]
    pub database_pool_size: u32,
    /// 数据库超时（秒）
    #[serde(default = "default_database_timeout")]
    #[serde(alias = "DATABASE_TIMEOUT", alias = "FBA_DATABASE_TIMEOUT")]
    pub database_timeout: u64,
    /// 是否打印 SQL
    #[serde(default = "default_database_echo")]
    #[serde(alias = "DATABASE_ECHO", alias = "FBA_DATABASE_ECHO")]
    pub database_echo: bool,
    /// 是否打印连接池信息
    #[serde(default = "default_database_pool_echo")]
    #[serde(alias = "DATABASE_POOL_ECHO", alias = "FBA_DATABASE_POOL_ECHO")]
    pub database_pool_echo: bool,

    // ===== Redis 配置 =====
    /// Redis 主机
    #[serde(default = "default_redis_host")]
    #[serde(alias = "REDIS_HOST", alias = "FBA_REDIS_HOST")]
    pub redis_host: String,
    /// Redis 端口
    #[serde(default = "default_redis_port")]
    #[serde(alias = "REDIS_PORT", alias = "FBA_REDIS_PORT")]
    pub redis_port: u16,
    /// Redis 密码
    #[serde(default = "default_redis_password")]
    #[serde(alias = "REDIS_PASSWORD", alias = "FBA_REDIS_PASSWORD")]
    pub redis_password: String,
    /// Redis 数据库编号
    #[serde(default = "default_redis_database")]
    #[serde(alias = "REDIS_DATABASE", alias = "FBA_REDIS_DATABASE")]
    pub redis_database: i64,
    /// Redis 超时（秒）
    #[serde(default = "default_redis_timeout")]
    #[serde(alias = "REDIS_TIMEOUT", alias = "FBA_REDIS_TIMEOUT")]
    pub redis_timeout: u64,
    /// Redis 连接池大小
    #[serde(default = "default_redis_pool_size")]
    #[serde(alias = "REDIS_POOL_SIZE", alias = "FBA_REDIS_POOL_SIZE")]
    pub redis_pool_size: u32,

    // ===== JWT/Token 配置 =====
    /// JWT 密钥
    #[serde(default = "default_token_secret_key")]
    #[serde(alias = "TOKEN_SECRET_KEY", alias = "FBA_TOKEN_SECRET_KEY")]
    pub token_secret_key: String,
    /// JWT 算法
    #[serde(default = "default_token_algorithm")]
    #[serde(alias = "TOKEN_ALGORITHM", alias = "FBA_TOKEN_ALGORITHM")]
    pub token_algorithm: String,
    /// Access Token 过期时间（秒）
    #[serde(default = "default_token_expire_seconds")]
    #[serde(alias = "TOKEN_EXPIRE_SECONDS", alias = "FBA_TOKEN_EXPIRE_SECONDS")]
    pub token_expire_seconds: i64,
    /// Refresh Token 过期时间（秒）
    #[serde(default = "default_token_refresh_expire_seconds")]
    #[serde(alias = "TOKEN_REFRESH_EXPIRE_SECONDS", alias = "FBA_TOKEN_REFRESH_EXPIRE_SECONDS")]
    pub token_refresh_expire_seconds: i64,
    /// Token Redis 前缀
    #[serde(default = "default_token_redis_prefix")]
    #[serde(alias = "TOKEN_REDIS_PREFIX", alias = "FBA_TOKEN_REDIS_PREFIX")]
    pub token_redis_prefix: String,
    /// Token 额外信息 Redis 前缀
    #[serde(default = "default_token_extra_info_redis_prefix")]
    #[serde(alias = "TOKEN_EXTRA_INFO_REDIS_PREFIX", alias = "FBA_TOKEN_EXTRA_INFO_REDIS_PREFIX")]
    pub token_extra_info_redis_prefix: String,
    /// Token 在线用户 Redis 前缀
    #[serde(default = "default_token_online_redis_prefix")]
    #[serde(alias = "TOKEN_ONLINE_REDIS_PREFIX", alias = "FBA_TOKEN_ONLINE_REDIS_PREFIX")]
    pub token_online_redis_prefix: String,
    /// Token 刷新 Redis 前缀
    #[serde(default = "default_token_refresh_redis_prefix")]
    #[serde(alias = "TOKEN_REFRESH_REDIS_PREFIX", alias = "FBA_TOKEN_REFRESH_REDIS_PREFIX")]
    pub token_refresh_redis_prefix: String,
    /// JWT 用户信息 Redis 前缀
    #[serde(default = "default_jwt_user_redis_prefix")]
    #[serde(alias = "JWT_USER_REDIS_PREFIX", alias = "FBA_JWT_USER_REDIS_PREFIX")]
    pub jwt_user_redis_prefix: String,
    /// Token 请求白名单路径
    #[serde(default = "default_token_exclude_paths")]
    #[serde(alias = "TOKEN_EXCLUDE_PATHS", alias = "FBA_TOKEN_EXCLUDE_PATHS")]
    pub token_exclude_paths: Vec<String>,

    // ===== Cookie 配置 =====
    /// Cookie 中的刷新 Token 键名
    #[serde(default = "default_cookie_refresh_token_key")]
    #[serde(alias = "COOKIE_REFRESH_TOKEN_KEY", alias = "FBA_COOKIE_REFRESH_TOKEN_KEY")]
    pub cookie_refresh_token_key: String,
    /// Cookie 中刷新 Token 过期时间（秒）
    #[serde(default = "default_cookie_refresh_token_expire_seconds")]
    #[serde(alias = "COOKIE_REFRESH_TOKEN_EXPIRE_SECONDS", alias = "FBA_COOKIE_REFRESH_TOKEN_EXPIRE_SECONDS")]
    pub cookie_refresh_token_expire_seconds: i64,

    // ===== 验证码配置 =====
    /// 登录验证码 Redis 前缀
    #[serde(default = "default_captcha_login_redis_prefix")]
    #[serde(alias = "CAPTCHA_LOGIN_REDIS_PREFIX", alias = "FBA_CAPTCHA_LOGIN_REDIS_PREFIX")]
    pub captcha_login_redis_prefix: String,
    /// 验证码过期时间（秒）
    #[serde(default = "default_captcha_expire_seconds")]
    #[serde(alias = "CAPTCHA_LOGIN_EXPIRE_SECONDS", alias = "FBA_CAPTCHA_EXPIRE_SECONDS")]
    pub captcha_expire_seconds: i64,

    // ===== RBAC 配置 =====
    /// 是否启用角色菜单模式
    #[serde(default = "default_rbac_role_menu_mode")]
    #[serde(alias = "RBAC_ROLE_MENU_MODE", alias = "FBA_RBAC_ROLE_MENU_MODE")]
    pub rbac_role_menu_mode: bool,
    /// 角色菜单排除列表
    #[serde(default = "default_rbac_role_menu_exclude")]
    #[serde(alias = "RBAC_ROLE_MENU_EXCLUDE", alias = "FBA_RBAC_ROLE_MENU_EXCLUDE")]
    pub rbac_role_menu_exclude: Vec<String>,

    // ===== 数据权限配置 =====
    /// 允许进行数据过滤的模型映射
    #[serde(default = "default_data_permission_models")]
    #[serde(alias = "DATA_PERMISSION_MODELS", alias = "FBA_DATA_PERMISSION_MODELS")]
    pub data_permission_models: HashMap<String, String>,
    /// 数据权限排除列
    #[serde(default = "default_data_permission_column_exclude")]
    #[serde(alias = "DATA_PERMISSION_COLUMN_EXCLUDE", alias = "FBA_DATA_PERMISSION_COLUMN_EXCLUDE")]
    pub data_permission_column_exclude: Vec<String>,

    // ===== WebSocket 配置 =====
    /// WebSocket 免授权直连标记（用于测试）
    #[serde(default)]
    #[serde(alias = "WS_NO_AUTH_MARKER", alias = "FBA_WS_NO_AUTH_MARKER")]
    pub ws_no_auth_marker: Option<String>,

    // ===== 日志配置 =====
    /// 调试模式（开启后显示详细日志：路由、SQL、响应时间等）
    #[serde(default = "default_debug_mode")]
    #[serde(alias = "DEBUG_MODE", alias = "FBA_DEBUG_MODE")]
    pub debug_mode: bool,
    /// 日志级别
    #[serde(default = "default_log_level")]
    #[serde(alias = "LOG_STD_LEVEL", alias = "FBA_LOG_LEVEL")]
    pub log_level: String,
    /// 是否启用 JSON 格式日志
    #[serde(default = "default_log_json")]
    #[serde(alias = "LOG_JSON", alias = "FBA_LOG_JSON")]
    pub log_json: bool,
    /// 日志文件路径
    #[serde(default)]
    #[serde(alias = "LOG_FILE", alias = "FBA_LOG_FILE")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_file: Option<String>,
    /// 日志文件最大大小（MB）
    #[serde(default = "default_log_file_max_size")]
    #[serde(alias = "LOG_FILE_MAX_SIZE", alias = "FBA_LOG_FILE_MAX_SIZE")]
    pub log_file_max_size: u64,
    /// 日志保留天数
    #[serde(default = "default_log_file_keep_days")]
    #[serde(alias = "LOG_FILE_KEEP_DAYS", alias = "FBA_LOG_FILE_KEEP_DAYS")]
    pub log_file_keep_days: u64,

    // ===== 文件上传配置 =====
    /// 图片上传大小限制（MB）
    #[serde(default = "default_upload_image_max_size")]
    #[serde(alias = "UPLOAD_IMAGE_SIZE_MAX", alias = "FBA_UPLOAD_IMAGE_MAX_SIZE")]
    pub upload_image_max_size: u64,
    /// 视频上传大小限制（MB）
    #[serde(default = "default_upload_video_max_size")]
    #[serde(alias = "UPLOAD_VIDEO_SIZE_MAX", alias = "FBA_UPLOAD_VIDEO_MAX_SIZE")]
    pub upload_video_max_size: u64,
    /// 允许的图片扩展名
    #[serde(default = "default_upload_image_extensions")]
    #[serde(alias = "UPLOAD_IMAGE_EXT_INCLUDE", alias = "FBA_UPLOAD_IMAGE_EXTENSIONS")]
    pub upload_image_extensions: Vec<String>,
    /// 允许的视频扩展名
    #[serde(default = "default_upload_video_extensions")]
    #[serde(alias = "UPLOAD_VIDEO_EXT_INCLUDE", alias = "FBA_UPLOAD_VIDEO_EXTENSIONS")]
    pub upload_video_extensions: Vec<String>,

    // ===== IP 定位配置 =====
    /// IP 定位方式
    #[serde(default = "default_ip_location_mode")]
    #[serde(alias = "IP_LOCATION_PARSE", alias = "FBA_IP_LOCATION_MODE")]
    pub ip_location_mode: String,

    // ===== 限流配置 =====
    /// 限流 Redis 前缀
    #[serde(default = "default_rate_limit_redis_prefix")]
    #[serde(alias = "REQUEST_LIMITER_REDIS_PREFIX", alias = "FBA_RATE_LIMIT_REDIS_PREFIX")]
    pub rate_limit_redis_prefix: String,
    
    // ===== 插件配置 =====
    /// 插件 Redis 前缀
    #[serde(default = "default_plugin_redis_prefix")]
    #[serde(alias = "PLUGIN_REDIS_PREFIX", alias = "FBA_PLUGIN_REDIS_PREFIX")]
    pub plugin_redis_prefix: String,

    // ===== 中间件配置 =====
    /// 是否启用 CORS
    #[serde(default = "default_middleware_cors")]
    #[serde(alias = "MIDDLEWARE_CORS", alias = "FBA_MIDDLEWARE_CORS")]
    pub middleware_cors: bool,
    /// 是否启用操作日志
    #[serde(default = "default_middleware_opera_log")]
    #[serde(alias = "MIDDLEWARE_OPERA_LOG", alias = "FBA_MIDDLEWARE_OPERA_LOG")]
    pub middleware_opera_log: bool,
    /// 是否启用访问日志
    #[serde(default = "default_middleware_access_log")]
    #[serde(alias = "MIDDLEWARE_ACCESS_LOG", alias = "FBA_MIDDLEWARE_ACCESS_LOG")]
    pub middleware_access_log: bool,
    /// 是否启用国际化
    #[serde(default = "default_middleware_i18n")]
    #[serde(alias = "MIDDLEWARE_I18N", alias = "FBA_MIDDLEWARE_I18N")]
    pub middleware_i18n: bool,

    // ===== 操作日志配置 =====
    /// 操作日志加密密钥
    #[serde(default = "default_opera_log_encrypt_secret_key")]
    #[serde(alias = "OPERA_LOG_ENCRYPT_SECRET_KEY", alias = "FBA_OPERA_LOG_ENCRYPT_SECRET_KEY")]
    pub opera_log_encrypt_secret_key: String,
    /// 操作日志排除路径
    #[serde(default = "default_opera_log_path_exclude")]
    #[serde(alias = "OPERA_LOG_PATH_EXCLUDE", alias = "FBA_OPERA_LOG_PATH_EXCLUDE")]
    pub opera_log_path_exclude: Vec<String>,
    /// 操作日志加密类型
    #[serde(default = "default_opera_log_encrypt_type")]
    #[serde(alias = "OPERA_LOG_ENCRYPT_TYPE", alias = "FBA_OPERA_LOG_ENCRYPT_TYPE")]
    pub opera_log_encrypt_type: i32,
    /// 操作日志加密键包含
    #[serde(default = "default_opera_log_encrypt_key_include")]
    #[serde(alias = "OPERA_LOG_ENCRYPT_KEY_INCLUDE", alias = "FBA_OPERA_LOG_ENCRYPT_KEY_INCLUDE")]
    pub opera_log_encrypt_key_include: Vec<String>,
    /// 操作日志队列批次消费大小
    #[serde(default = "default_opera_log_queue_batch_consume_size")]
    #[serde(alias = "OPERA_LOG_QUEUE_BATCH_CONSUME_SIZE", alias = "FBA_OPERA_LOG_QUEUE_BATCH_CONSUME_SIZE")]
    pub opera_log_queue_batch_consume_size: i32,
    /// 操作日志队列超时
    #[serde(default = "default_opera_log_queue_timeout")]
    #[serde(alias = "OPERA_LOG_QUEUE_TIMEOUT", alias = "FBA_OPERA_LOG_QUEUE_TIMEOUT")]
    pub opera_log_queue_timeout: i32,

    // ===== 新增：Trace ID 配置 =====
    /// Trace ID 请求头键名
    #[serde(default = "default_trace_id_request_header_key")]
    #[serde(alias = "TRACE_ID_REQUEST_HEADER_KEY", alias = "FBA_TRACE_ID_REQUEST_HEADER_KEY")]
    pub trace_id_request_header_key: String,
    /// Trace ID 日志长度
    #[serde(default = "default_trace_id_log_length")]
    #[serde(alias = "TRACE_ID_LOG_LENGTH", alias = "FBA_TRACE_ID_LOG_LENGTH")]
    pub trace_id_log_length: usize,
    /// Trace ID 日志默认值
    #[serde(default = "default_trace_id_log_default_value")]
    #[serde(alias = "TRACE_ID_LOG_DEFAULT_VALUE", alias = "FBA_TRACE_ID_LOG_DEFAULT_VALUE")]
    pub trace_id_log_default_value: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            environment: default_environment(),
            api_v1_path: default_api_v1_path(),
            app_title: default_app_title(),
            app_description: default_app_description(),
            docs_url: default_docs_url(),
            redoc_url: default_redoc_url(),
            openapi_url: default_openapi_url(),
            static_files: default_static_files(),

            database_type: default_database_type(),
            database_host: default_database_host(),
            database_port: default_database_port(),
            database_user: default_database_user(),
            database_password: default_database_password(),
            database_name: default_database_name(),
            database_charset: default_database_charset(),
            database_schema: default_database_schema(),
            database_pool_size: default_database_pool_size(),
            database_timeout: default_database_timeout(),
            database_echo: default_database_echo(),
            database_pool_echo: default_database_pool_echo(),

            redis_host: default_redis_host(),
            redis_port: default_redis_port(),
            redis_password: default_redis_password(),
            redis_database: default_redis_database(),
            redis_timeout: default_redis_timeout(),
            redis_pool_size: default_redis_pool_size(),

            token_secret_key: default_token_secret_key(),
            token_algorithm: default_token_algorithm(),
            token_expire_seconds: default_token_expire_seconds(),
            token_refresh_expire_seconds: default_token_refresh_expire_seconds(),
            token_redis_prefix: default_token_redis_prefix(),
            token_extra_info_redis_prefix: default_token_extra_info_redis_prefix(),
            token_online_redis_prefix: default_token_online_redis_prefix(),
            token_refresh_redis_prefix: default_token_refresh_redis_prefix(),
            jwt_user_redis_prefix: default_jwt_user_redis_prefix(),
            token_exclude_paths: default_token_exclude_paths(),

            cookie_refresh_token_key: default_cookie_refresh_token_key(),
            cookie_refresh_token_expire_seconds: default_cookie_refresh_token_expire_seconds(),

            captcha_login_redis_prefix: default_captcha_login_redis_prefix(),
            captcha_expire_seconds: default_captcha_expire_seconds(),

            rbac_role_menu_mode: default_rbac_role_menu_mode(),
            rbac_role_menu_exclude: default_rbac_role_menu_exclude(),

            data_permission_models: default_data_permission_models(),
            data_permission_column_exclude: default_data_permission_column_exclude(),

            ws_no_auth_marker: None,

            debug_mode: default_debug_mode(),
            log_level: default_log_level(),
            log_json: default_log_json(),
            log_file: None,
            log_file_max_size: default_log_file_max_size(),
            log_file_keep_days: default_log_file_keep_days(),

            upload_image_max_size: default_upload_image_max_size(),
            upload_video_max_size: default_upload_video_max_size(),
            upload_image_extensions: default_upload_image_extensions(),
            upload_video_extensions: default_upload_video_extensions(),

            ip_location_mode: default_ip_location_mode(),

            rate_limit_redis_prefix: default_rate_limit_redis_prefix(),
            plugin_redis_prefix: default_plugin_redis_prefix(),

            middleware_cors: default_middleware_cors(),
            middleware_opera_log: default_middleware_opera_log(),
            middleware_access_log: default_middleware_access_log(),
            middleware_i18n: default_middleware_i18n(),

            opera_log_encrypt_secret_key: default_opera_log_encrypt_secret_key(),
            opera_log_path_exclude: default_opera_log_path_exclude(),
            opera_log_encrypt_type: default_opera_log_encrypt_type(),
            opera_log_encrypt_key_include: default_opera_log_encrypt_key_include(),
            opera_log_queue_batch_consume_size: default_opera_log_queue_batch_consume_size(),
            opera_log_queue_timeout: default_opera_log_queue_timeout(),

            trace_id_request_header_key: default_trace_id_request_header_key(),
            trace_id_log_length: default_trace_id_log_length(),
            trace_id_log_default_value: default_trace_id_log_default_value(),
        }
    }
}

// ===== Default 函数实现 =====

fn default_environment() -> EnvironmentType { EnvironmentType::Dev }
fn default_api_v1_path() -> String { "/api/v1".to_string() }
fn default_app_title() -> String { "FastAPI Best Architecture".to_string() }
fn default_app_description() -> String { "FastAPI Best Architecture".to_string() }
fn default_docs_url() -> String { "/docs".to_string() }
fn default_redoc_url() -> String { "/redoc".to_string() }
fn default_openapi_url() -> Option<String> { Some("/openapi".to_string()) }
fn default_static_files() -> bool { true }

fn default_database_type() -> DatabaseType { DatabaseType::MySQL }
fn default_database_host() -> String { "127.0.0.1".to_string() }
fn default_database_port() -> u16 { 5432 }
fn default_database_user() -> String { "postgres".to_string() }
fn default_database_password() -> String { "123456".to_string() }
fn default_database_name() -> String { "./fba.db".to_string() }
fn default_database_charset() -> String { "utf8mb4".to_string() }
fn default_database_schema() -> String { "public".to_string() }
fn default_database_pool_size() -> u32 { 10 }
fn default_database_timeout() -> u64 { 30 }
fn default_database_echo() -> bool { false }
fn default_database_pool_echo() -> bool { false }

fn default_redis_host() -> String { "127.0.0.1".to_string() }
fn default_redis_port() -> u16 { 6379 }
fn default_redis_password() -> String { "".to_string() }
fn default_redis_database() -> i64 { 0 }
fn default_redis_timeout() -> u64 { 5 }
fn default_redis_pool_size() -> u32 { 10 }

fn default_token_secret_key() -> String {
    // 尝试从环境变量读取
    if let Ok(secret) = std::env::var("TOKEN_SECRET_KEY") {
        if !secret.is_empty() {
            return secret;
        }
    }

    // 如果没有配置环境变量，使用默认值
    "1VkVF75nsNABBjK_7-qz7GtzNy3AMvktc9TCPwKczCk".to_string()
}
fn default_token_algorithm() -> String { "HS256".to_string() }
fn default_token_expire_seconds() -> i64 { 60 * 60 * 24 }
fn default_token_refresh_expire_seconds() -> i64 { 60 * 60 * 24 * 7 }
fn default_token_redis_prefix() -> String { "fba:token".to_string() }
fn default_token_extra_info_redis_prefix() -> String { "fba:token_extra_info".to_string() }
fn default_token_online_redis_prefix() -> String { "fba:token_online".to_string() }
fn default_token_refresh_redis_prefix() -> String { "fba:refresh_token".to_string() }
fn default_jwt_user_redis_prefix() -> String { "fba:user".to_string() }
fn default_token_exclude_paths() -> Vec<String> { vec!["/api/v1/auth/login".to_string()] }

fn default_cookie_refresh_token_key() -> String { "fba_refresh_token".to_string() }
fn default_cookie_refresh_token_expire_seconds() -> i64 { 60 * 60 * 24 * 7 }

fn default_captcha_login_redis_prefix() -> String { "fba:login:captcha".to_string() }
fn default_captcha_expire_seconds() -> i64 { 300 }

fn default_rbac_role_menu_mode() -> bool { true }
fn default_rbac_role_menu_exclude() -> Vec<String> {
    vec!["sys:monitor:redis".to_string(), "sys:monitor:server".to_string()]
}
fn default_data_permission_models() -> HashMap<String, String> { HashMap::new() }
fn default_data_permission_column_exclude() -> Vec<String> {
    vec!["id".to_string(), "sort".to_string(), "del_flag".to_string(), "created_time".to_string(), "updated_time".to_string()]
}

fn default_debug_mode() -> bool { false }
fn default_log_level() -> String { "INFO".to_string() }
fn default_log_json() -> bool { false }
fn default_log_file_max_size() -> u64 { 100 }
fn default_log_file_keep_days() -> u64 { 30 }

fn default_upload_image_max_size() -> u64 { 5 }
fn default_upload_video_max_size() -> u64 { 100 }
fn default_upload_image_extensions() -> Vec<String> {
    vec!["jpg".to_string(), "jpeg".to_string(), "png".to_string(), "gif".to_string(), "webp".to_string()]
}
fn default_upload_video_extensions() -> Vec<String> {
    vec!["mp4".to_string(), "mov".to_string(), "avi".to_string(), "flv".to_string()]
}

fn default_ip_location_mode() -> String { "offline".to_string() }
fn default_rate_limit_redis_prefix() -> String { "fba:limiter".to_string() }
fn default_plugin_redis_prefix() -> String { "fba:plugin".to_string() }

fn default_middleware_cors() -> bool { true }
fn default_middleware_opera_log() -> bool { true }
fn default_middleware_access_log() -> bool { true }
fn default_middleware_i18n() -> bool { true }

fn default_opera_log_encrypt_secret_key() -> String {
    "d77b25790a804c2b4a339dd0207941e4cefa5751935a33735bc73bb7071a005b".to_string()
}
fn default_opera_log_path_exclude() -> Vec<String> {
    vec!["/favicon.ico".to_string(), "/docs".to_string(), "/redoc".to_string(),
         "/openapi".to_string(), "/api/v1/auth/login/swagger".to_string()]
}
fn default_opera_log_encrypt_type() -> i32 { 1 }
fn default_opera_log_encrypt_key_include() -> Vec<String> {
    vec!["password".to_string(), "old_password".to_string(), "new_password".to_string(), "confirm_password".to_string()]
}
fn default_opera_log_queue_batch_consume_size() -> i32 { 100 }
fn default_opera_log_queue_timeout() -> i32 { 60 }

fn default_trace_id_request_header_key() -> String { "X-Request-ID".to_string() }
fn default_trace_id_log_length() -> usize { 32 }
fn default_trace_id_log_default_value() -> String { "-".to_string() }


impl Settings {
    /// 创建全局配置实例
    pub fn new() -> Self {
        // 优先从项目根目录加载 .env
        // CARGO_MANIFEST_DIR 在编译期固定为当前 crate 的根目录
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let root_env_path = std::path::Path::new(manifest_dir).join(".env");

        if root_env_path.exists() {
            dotenvy::from_path(root_env_path).ok();
        } else {
            // 回退到从当前工作目录向上查找 .env
            dotenvy::dotenv().ok();
        }

        // 从环境变量或 .env 文件读取配置
        envy::from_env::<Self>().unwrap_or_else(|err| {
            eprintln!("❌ 配置加载失败: {}", err);
            eprintln!("使用默认配置值");
            eprintln!("请检查 .env 文件格式（不要在值两边加引号）");
            Self::default()
        })
    }

    /// 获取数据库连接 URL
    pub fn database_url(&self) -> String {
        match self.database_type {
            DatabaseType::PostgreSQL => {
                format!(
                    "postgresql://{}:{}@{}:{}/{}?options=-c%20lc_messages=C",
                    self.database_user,
                    self.database_password,
                    self.database_host,
                    self.database_port,
                    self.database_name
                )
            }
            DatabaseType::MySQL => {
                format!(
                    "mysql://{}:{}@{}:{}/{}?charset={}",
                    self.database_user,
                    self.database_password,
                    self.database_host,
                    self.database_port,
                    self.database_name,
                    self.database_charset
                )
            }
            DatabaseType::SQLite => {
                // SQLite使用数据库名作为文件路径
                format!("sqlite:{}", self.database_name)
            }
        }
    }

    /// 获取 Redis 连接 URL
    pub fn redis_url(&self) -> String {
        if self.redis_password.is_empty() {
            format!("redis://{}:{}", self.redis_host, self.redis_port)
        } else {
            format!("redis://:{}@{}:{}", self.redis_password, self.redis_host, self.redis_port)
        }
    }

    /// 检查是否为开发环境
    pub fn is_dev(&self) -> bool {
        matches!(self.environment, EnvironmentType::Dev)
    }

    /// 检查是否为生产环境
    pub fn is_prod(&self) -> bool {
        matches!(self.environment, EnvironmentType::Prod)
    }

    /// 检查是否启用操作日志
    pub fn is_opera_log_enabled(&self) -> bool {
        self.middleware_opera_log || self.is_dev()
    }

    /// 检查是否启用访问日志
    pub fn is_access_log_enabled(&self) -> bool {
        self.middleware_access_log || self.is_dev()
    }
}

/// 全局配置实例
pub static SETTINGS: Lazy<Settings> = Lazy::new(Settings::new);
