/// 数据权限路由模块
/// 组装所有数据权限相关的API路由

pub mod data_scope_router;
pub mod data_rule_router;

pub use data_rule_router::*;

/// 数据权限相关路由集合
/// 包括数据权限和数据规则的所有API路由
pub fn data_scope_routes() -> axum::Router {
    axum::Router::new()
        .nest("/data-scopes", data_scope_router::data_scope_routes())
        .nest("/data-rules", data_rule_router::data_rule_routes())
}
