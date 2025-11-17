/// API v1 路由
/// 版本1的所有API路由

use axum::Router;

/// 聚合所有 v1 路由
pub fn router() -> Router {
    Router::new()
        // 这里可以添加各个模块的路由
        // .nest("/auth", auth_router())
        // .nest("/sys", sys_router())
        // .nest("/log", log_router())
        // .nest("/monitor", monitor_router())
}
