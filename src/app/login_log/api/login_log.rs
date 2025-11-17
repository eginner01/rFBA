use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use crate::common::response::api_response;
use crate::common::exception::AppError;
use crate::database::DatabaseManager;
use crate::app::login_log::dto::{
    LoginLogPaginationQuery, CreateLoginLogRequest, CreateLogoutLogRequest,
    DeleteLoginLogsRequest,
};
use crate::app::login_log::service::LoginLogService;

/// 获取登录日志列表
pub async fn get_login_logs_paginated(
    Query(query): Query<LoginLogPaginationQuery>,
) -> Result<impl IntoResponse, AppError> {
    let db_conn = DatabaseManager::get_connection().await;
    let service = LoginLogService::new(db_conn.clone());
    let result = service.get_login_logs_paginated(&query).await?;
    Ok((StatusCode::OK, Json(api_response(result))))
}

/// 获取登录日志详情
pub async fn get_login_log_detail(
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let db_conn = DatabaseManager::get_connection().await;
    let service = LoginLogService::new(db_conn.clone());
    let result = service.get_login_log_detail(id).await?;
    Ok((StatusCode::OK, Json(api_response(result))))
}

/// 创建登录日志
pub async fn create_login_log(
    Json(request): Json<CreateLoginLogRequest>,
) -> Result<impl IntoResponse, AppError> {
    let db_conn = DatabaseManager::get_connection().await;
    let service = LoginLogService::new(db_conn.clone());
    let result = service.create_login_log(&request).await?;
    Ok((StatusCode::CREATED, Json(api_response(result))))
}

/// 创建注销日志
pub async fn create_logout_log(
    Json(request): Json<CreateLogoutLogRequest>,
) -> Result<impl IntoResponse, AppError> {
    let db_conn = DatabaseManager::get_connection().await;
    let service = LoginLogService::new(db_conn.clone());
    let result = service.create_logout_log(&request).await?;
    Ok((StatusCode::CREATED, Json(api_response(result))))
}

/// 删除登录日志
pub async fn delete_login_log(
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let db_conn = DatabaseManager::get_connection().await;
    let service = LoginLogService::new(db_conn.clone());
    service.delete_login_log(id).await?;
    Ok((StatusCode::NO_CONTENT, Json(api_response("删除成功".to_string()))))
}

/// 批量删除登录日志
pub async fn delete_login_logs_batch(
    Json(request): Json<DeleteLoginLogsRequest>,
) -> Result<impl IntoResponse, AppError> {
    let db_conn = DatabaseManager::get_connection().await;
    let service = LoginLogService::new(db_conn.clone());
    let result = service.delete_login_logs_batch(&request.ids).await?;
    Ok((StatusCode::OK, Json(api_response(format!("成功删除 {} 条记录", result)))))
}

/// 清空登录日志
pub async fn clear_login_logs(
) -> Result<impl IntoResponse, AppError> {
    let db_conn = DatabaseManager::get_connection().await;
    let service = LoginLogService::new(db_conn.clone());
    let result = service.clear_login_logs().await?;
    Ok((StatusCode::OK, Json(api_response(format!("成功清空 {} 条记录", result)))))
}

/// 获取登录日志统计
pub async fn get_login_log_statistics(
) -> Result<impl IntoResponse, AppError> {
    let db_conn = DatabaseManager::get_connection().await;
    let service = LoginLogService::new(db_conn.clone());
    let result = service.get_login_log_statistics(None, None).await?;
    Ok((StatusCode::OK, Json(api_response(result))))
}
