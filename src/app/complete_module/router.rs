/// 完整性检查路由
/// 组装所有完整性检查相关的API路由

use axum::{routing::get, Router};

use crate::app::complete_module::api::{
        get_system_status,
        get_module_info,
        health_check,
        get_system_summary,
    };

/// 组合完整性检查相关的路由
pub fn complete_routes() -> Router {
    Router::new()
        .route("/complete/system-status", get(get_system_status))
        .route("/complete/modules", get(get_module_info))
        .route("/complete/health", get(health_check))
        .route("/complete/summary", get(get_system_summary))
}
