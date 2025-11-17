/// 任务结果 API 处理器

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
    TaskResultListQuery, DeleteTaskRequest,
};
use crate::app::task::service::TaskResultService;

/// 获取任务结果详情
pub async fn get_task_result(
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let db_conn = DatabaseManager::get_connection().await.clone();
    let service = TaskResultService::new(db_conn);

    let result = service.get_by_id(id).await?;

    Ok((StatusCode::OK, Json(api_response(result))))
}

/// 任务结果列表（分页）
pub async fn get_task_results_paginated(
    Query(query): Query<TaskResultListQuery>,
) -> Result<impl IntoResponse, AppError> {
    use crate::common::pagination::PageData;
    
    let db_conn = DatabaseManager::get_connection().await.clone();
    let service = TaskResultService::new(db_conn);

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

/// 批量删除任务结果
pub async fn delete_task_result(
    Json(request): Json<DeleteTaskRequest>,
) -> Result<impl IntoResponse, AppError> {
    let db_conn = DatabaseManager::get_connection().await.clone();
    let service = TaskResultService::new(db_conn);

    let count = service.delete_batch(&request).await?;

    Ok((StatusCode::OK, Json(api_response(format!("已删除 {} 条记录", count)))))
}
