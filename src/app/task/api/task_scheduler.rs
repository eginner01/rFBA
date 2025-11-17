/// 任务调度器 API 处理器

use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use crate::common::response::api_response;
use crate::common::exception::AppError;
use crate::database::DatabaseManager;
use crate::app::task::dto::{
    CreateTaskSchedulerRequest,
    UpdateTaskSchedulerRequest,
    TaskSchedulerListQuery,
};
use crate::app::task::service::TaskSchedulerService;

/// 状态更新请求 DTO
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusChangeRequest {
    pub status: i32,
}

/// 获取所有任务调度器
pub async fn get_all_task_schedulers() -> Result<impl IntoResponse, AppError> {
    let db_conn = DatabaseManager::get_connection().await.clone();
    let service = TaskSchedulerService::new(db_conn);

    let result = service.get_all().await?;

    Ok((StatusCode::OK, Json(api_response(result))))
}

/// 获取任务调度器详情
pub async fn get_task_scheduler(
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let db_conn = DatabaseManager::get_connection().await.clone();
    let service = TaskSchedulerService::new(db_conn);

    let result = service.get_by_id(id).await?;

    Ok((StatusCode::OK, Json(api_response(result))))
}

/// 任务调度器列表（分页）
pub async fn get_task_scheduler_paginated(
    Query(query): Query<TaskSchedulerListQuery>,
) -> Result<impl IntoResponse, AppError> {
    use crate::common::pagination::PageData;
    
    let db_conn = DatabaseManager::get_connection().await.clone();
    let service = TaskSchedulerService::new(db_conn);

    let (list, total) = service.get_list(&query).await?;
    
    // 使用 PageData 返回分页格式
    let page_data = PageData::new(
        list,
        total as i64,
        query.page.unwrap_or(1),
        query.size.unwrap_or(20),
    );

    Ok((
        StatusCode::OK,
        Json(api_response(page_data)),
    ))
}

/// 创建任务调度器
pub async fn create_task_scheduler(
    Json(request): Json<CreateTaskSchedulerRequest>,
) -> Result<impl IntoResponse, AppError> {
    let db_conn = DatabaseManager::get_connection().await.clone();
    let service = TaskSchedulerService::new(db_conn);

    let result = service.create(&request).await?;

    Ok((StatusCode::CREATED, Json(api_response(result))))
}

/// 更新任务调度器
pub async fn update_task_scheduler(
    Path(id): Path<i64>,
    Json(request): Json<UpdateTaskSchedulerRequest>,
) -> Result<impl IntoResponse, AppError> {
    let db_conn = DatabaseManager::get_connection().await.clone();
    let service = TaskSchedulerService::new(db_conn);

    // ID 从 URL 路径获取
    let result = service.update(id, &request).await?;

    Ok((StatusCode::OK, Json(api_response(result))))
}

/// 更新任务调度器状态
pub async fn update_task_scheduler_status(
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let db_conn = DatabaseManager::get_connection().await.clone();
    let service = TaskSchedulerService::new(db_conn);

    service.update_status(id).await?;

    Ok((StatusCode::OK, Json(api_response("状态更新成功".to_string()))))
}

/// 删除任务调度器
pub async fn delete_task_scheduler(
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let db_conn = DatabaseManager::get_connection().await.clone();
    let service = TaskSchedulerService::new(db_conn);

    service.delete(id).await?;

    // 返回 200 OK
    Ok((StatusCode::OK, Json(api_response("删除成功".to_string()))))
}

/// 手动执行任务
pub async fn execute_task(
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let db_conn = DatabaseManager::get_connection().await.clone();
    let service = TaskSchedulerService::new(db_conn);

    service.execute(id).await?;

    Ok((StatusCode::OK, Json(api_response("任务执行成功".to_string()))))
}
