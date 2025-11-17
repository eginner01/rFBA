//! 通知公告API处理器

use axum::{
    extract::{Path, Query, State},
    response::Json,
};
use sea_orm::DatabaseConnection;
use validator::Validate;

use crate::dto::*;
use crate::error::NoticeError;
use crate::service::NoticeService;

/// 获取所有通知公告
/// GET /api/v1/sys/notices/all
pub async fn get_all_notices(
    State(db): State<DatabaseConnection>,
) -> Result<Json<ApiResponse<Vec<NoticeDetail>>>, NoticeError> {
    let data = NoticeService::get_all(&db).await?;
    Ok(Json(ApiResponse::success(data)))
}

/// 获取通知公告详情
/// GET /api/v1/sys/notices/{pk}
pub async fn get_notice(
    State(db): State<DatabaseConnection>,
    Path(pk): Path<i64>,
) -> Result<Json<ApiResponse<NoticeDetail>>, NoticeError> {
    let data = NoticeService::get_by_id(&db, pk).await?;
    Ok(Json(ApiResponse::success(data)))
}

/// 获取显示的通知公告列表（公开接口）
/// GET /api/v1/sys/notices/visible
pub async fn get_visible_notices(
    State(db): State<DatabaseConnection>,
) -> Result<Json<ApiResponse<Vec<NoticeDetail>>>, NoticeError> {
    let data = NoticeService::get_visible(&db).await?;
    Ok(Json(ApiResponse::success(data)))
}

/// 分页获取所有通知公告
/// GET /api/v1/sys/notices
pub async fn get_notices_paginated(
    State(db): State<DatabaseConnection>,
    Query(query): Query<NoticeQuery>,
    Query(pagination): Query<PaginationQuery>,
) -> Result<Json<ApiResponse<PageData<NoticeDetail>>>, NoticeError> {
    let page_data = NoticeService::get_list(&db, query, pagination).await?;
    Ok(Json(ApiResponse::success(page_data)))
}

/// 创建通知公告
/// POST /api/v1/sys/notices
pub async fn create_notice(
    State(db): State<DatabaseConnection>,
    Json(param): Json<CreateNoticeParam>,
) -> Result<Json<ApiResponse<NoticeDetail>>, NoticeError> {
    // 验证参数
    param.validate()?;
    
    let data = NoticeService::create(&db, param).await?;
    Ok(Json(ApiResponse::success_with_msg("创建成功", data)))
}

/// 更新通知公告
/// PUT /api/v1/sys/notices/{pk}
pub async fn update_notice(
    State(db): State<DatabaseConnection>,
    Path(pk): Path<i64>,
    Json(param): Json<UpdateNoticeParam>,
) -> Result<Json<ApiResponse<()>>, NoticeError> {
    // 验证参数
    param.validate()?;
    
    let count = NoticeService::update(&db, pk, param).await?;
    if count > 0 {
        Ok(Json(ApiResponse::success_msg("更新成功")))
    } else {
        Err(NoticeError::OperationFailed("更新失败".to_string()))
    }
}

/// 批量删除通知公告
/// DELETE /api/v1/sys/notices
pub async fn delete_notices(
    State(db): State<DatabaseConnection>,
    Json(param): Json<DeleteBatchParam>,
) -> Result<Json<ApiResponse<()>>, NoticeError> {
    let count = NoticeService::delete_batch(&db, param.ids).await?;
    if count > 0 {
        Ok(Json(ApiResponse::success_msg("删除成功")))
    } else {
        Err(NoticeError::OperationFailed("删除失败".to_string()))
    }
}

/// 创建通知公告路由
pub fn notice_routes() -> axum::Router<DatabaseConnection> {
    use axum::routing::{delete, get, post, put};
    
    axum::Router::new()
        .route("/all", get(get_all_notices))
        .route("/{pk}", get(get_notice))
        .route("/visible", get(get_visible_notices))
        .route("/", get(get_notices_paginated))
        .route("/", post(create_notice))
        .route("/{pk}", put(update_notice))
        .route("/", delete(delete_notices))
}
