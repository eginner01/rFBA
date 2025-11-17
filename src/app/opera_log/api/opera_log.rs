use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use crate::common::response::api_response;
use crate::common::exception::AppError;
use crate::database::DatabaseManager;
use crate::app::opera_log::dto::{
    OperaLogPaginationQuery, CreateOperaLogRequest, DeleteOperaLogsRequest,
};
use crate::app::opera_log::service::OperaLogService;

/// 获取操作日志列表
pub async fn get_opera_logs(
    Query(query): Query<OperaLogPaginationQuery>,
) -> Result<impl IntoResponse, AppError> {
    let db_conn = DatabaseManager::get_connection().await;
    let service = OperaLogService::new(db_conn.clone());
    let result = service.get_opera_logs_paginated(&query).await?;
    Ok((StatusCode::OK, Json(api_response(result))))
}

/// 获取操作日志详情
pub async fn get_opera_log(
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let db_conn = DatabaseManager::get_connection().await;
    let service = OperaLogService::new(db_conn.clone());
    let result = service.get_opera_log_detail(id).await?;
    Ok((StatusCode::OK, Json(api_response(result))))
}

/// 创建操作日志
pub async fn create_opera_log(
    Json(request): Json<CreateOperaLogRequest>,
) -> Result<impl IntoResponse, AppError> {
    let db_conn = DatabaseManager::get_connection().await;
    let service = OperaLogService::new(db_conn.clone());
    let result = service.create_opera_log(&request).await?;
    Ok((StatusCode::CREATED, Json(api_response(result))))
}

/// 删除操作日志
pub async fn delete_opera_log(
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let db_conn = DatabaseManager::get_connection().await;
    let service = OperaLogService::new(db_conn.clone());
    service.delete_opera_log(id).await?;
    Ok((StatusCode::NO_CONTENT, Json(api_response("删除成功"))))
}

/// 批量删除操作日志
pub async fn batch_delete_opera_logs(
    Json(request): Json<DeleteOperaLogsRequest>,
) -> Result<impl IntoResponse, AppError> {
    let db_conn = DatabaseManager::get_connection().await;
    let service = OperaLogService::new(db_conn.clone());
    let result = service.delete_opera_logs_batch(&request.ids).await?;
    Ok((StatusCode::OK, Json(api_response(format!("成功删除 {} 条记录", result)))))
}

/// 清空操作日志
pub async fn clear_opera_logs() -> Result<impl IntoResponse, AppError> {
    let db_conn = DatabaseManager::get_connection().await;
    let service = OperaLogService::new(db_conn.clone());
    let result = service.clear_opera_logs().await?;
    Ok((StatusCode::OK, Json(api_response(format!("成功清空 {} 条记录", result)))))
}
