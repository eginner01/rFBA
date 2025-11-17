use axum::{
    routing::*,
    Router,
    extract::{Path, Query, State},
    response::IntoResponse,
};

use crate::common::response::ResponseModel;
use crate::common::exception::AppError;
use crate::app::monitor::service::MonitorService;
use redis::Client as RedisClient;

pub fn monitor_routes(redis_client: Option<RedisClient>) -> Router {
    Router::new()
        // 服务器监控信息（CPU、内存、磁盘等）
        .route("/monitors/server", get(get_server_metrics))
        // Redis监控信息
        .route("/monitors/redis", get(get_redis_metrics))
        // 在线用户列表
        .route("/monitors/sessions", get(get_online_sessions))
        // 踢出指定在线用户
        .route("/monitors/sessions/{id}", get(get_session_detail).delete(kick_out_session))
        // 获取已注册任务列表
        .route("/monitors/registered", get(get_registered_tasks))
        // 系统状态
        .route("/monitors/status", get(get_system_status))
        // API指标
        .route("/monitors/api-metrics", get(get_api_metrics))
        // 健康检查
        .route("/monitors/health", get(health_check))
        .with_state(redis_client)
}

/// 获取服务器监控信息
async fn get_server_metrics(
    State(redis_client): State<Option<RedisClient>>,
) -> Result<impl IntoResponse, AppError> {
    let service = MonitorService::new(redis_client);
    let result = service.get_system_metrics().await?;
    Ok(ResponseModel::success(result))
}

/// 获取Redis监控信息
async fn get_redis_metrics(
    State(redis_client): State<Option<RedisClient>>,
) -> Result<impl IntoResponse, AppError> {
    let service = MonitorService::new(redis_client);
    let result = service.get_redis_metrics().await?;
    Ok(ResponseModel::success(result))
}

/// 获取在线用户列表
async fn get_online_sessions(
    State(redis_client): State<Option<RedisClient>>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<impl IntoResponse, AppError> {
    let username = params.get("username").map(|s| s.to_string());
    let service = MonitorService::new(redis_client);
    let result = service.get_online_sessions(username).await?;
    Ok(ResponseModel::success(result))
}

/// 获取在线用户详情
async fn get_session_detail(
    State(_redis_client): State<Option<RedisClient>>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    // 这里是简化实现，实际应该查询具体会话信息
    Ok(ResponseModel::success(format!("会话ID: {}", id)))
}

/// 踢出指定在线用户
async fn kick_out_session(
    State(redis_client): State<Option<RedisClient>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let service = MonitorService::new(redis_client);
    let result = service.kick_out_session(&id).await?;
    Ok(ResponseModel::success(result))
}

/// 获取已注册任务列表
async fn get_registered_tasks(
    State(redis_client): State<Option<RedisClient>>,
) -> Result<impl IntoResponse, AppError> {
    let service = MonitorService::new(redis_client);
    let result = service.get_registered_tasks().await?;
    Ok(ResponseModel::success(result))
}

/// 获取系统状态
async fn get_system_status(
    State(redis_client): State<Option<RedisClient>>,
) -> Result<impl IntoResponse, AppError> {
    let service = MonitorService::new(redis_client);
    let result = service.get_system_status().await?;
    Ok(ResponseModel::success(result))
}

/// 获取API指标
async fn get_api_metrics(
    State(redis_client): State<Option<RedisClient>>,
) -> Result<impl IntoResponse, AppError> {
    let service = MonitorService::new(redis_client);
    let result = service.get_api_metrics().await?;
    Ok(ResponseModel::success(result))
}

/// 健康检查
async fn health_check(
    State(redis_client): State<Option<RedisClient>>,
) -> Result<impl IntoResponse, AppError> {
    let service = MonitorService::new(redis_client);
    let result = service.health_check().await?;
    Ok(ResponseModel::success(result))
}
