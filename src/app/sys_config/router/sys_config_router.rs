use axum::{routing::{get, post, put, delete}, Router};
use crate::common::response::api_response;

pub fn sys_config_routes() -> Router {
    Router::new()
        .route("/sys-configs", get(|| async { api_response("系统配置列表") }))
        .route("/sys-configs", post(|| async { api_response("创建系统配置") }))
        .route("/sys-configs/{id}", get(|| async { api_response("系统配置详情") }))
        .route("/sys-configs/{id}", put(|| async { api_response("更新系统配置") }))
        .route("/sys-configs/{id}", delete(|| async { api_response("删除系统配置") }))
}
