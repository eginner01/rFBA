/// 权限管理相关API

use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use crate::common::response::ApiResult;
use crate::app::permission::dto::{
    CreatePermissionRequest, UpdatePermissionRequest, PermissionPaginationQuery,
};
use crate::app::permission::service::PermissionService;
use crate::database::DatabaseManager;
use crate::common::response::api_response;

/// 获取权限列表（分页）
/// GET /api/v1/permissions
pub async fn get_permissions(
    Query(query): Query<PermissionPaginationQuery>,
) -> ApiResult<impl IntoResponse> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建权限服务
    let permission_service = PermissionService::new(db_conn.clone());

    // 查询权限列表
    let result = permission_service.get_permissions_paginated(&query).await?;

    Ok((StatusCode::OK, Json(api_response(result))))
}

/// 获取权限详情
/// GET /api/v1/permissions/{id}
pub async fn get_permission(
    Path(id): Path<i64>,
) -> ApiResult<impl IntoResponse> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建权限服务
    let permission_service = PermissionService::new(db_conn.clone());

    // 查询权限详情
    let result = permission_service.get_permission_detail(id).await?;

    Ok((StatusCode::OK, Json(api_response(result))))
}

/// 创建权限
/// POST /api/v1/permissions
pub async fn create_permission(
    Json(request): Json<CreatePermissionRequest>,
) -> ApiResult<impl IntoResponse> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建权限服务
    let permission_service = PermissionService::new(db_conn.clone());

    // 创建权限
    let result = permission_service.create_permission(&request).await?;

    Ok((StatusCode::CREATED, Json(api_response(result))))
}

/// 更新权限
/// PUT /api/v1/permissions/{id}
pub async fn update_permission(
    Path(id): Path<i64>,
    Json(request): Json<UpdatePermissionRequest>,
) -> ApiResult<impl IntoResponse> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建权限服务
    let permission_service = PermissionService::new(db_conn.clone());

    // 更新权限
    permission_service.update_permission(id, &request).await?;

    Ok((StatusCode::OK, Json(api_response("权限更新成功".to_string()))))
}

/// 删除权限
/// DELETE /api/v1/permissions/{id}
pub async fn delete_permission(
    Path(id): Path<i64>,
) -> ApiResult<impl IntoResponse> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建权限服务
    let permission_service = PermissionService::new(db_conn.clone());

    // 删除权限
    permission_service.delete_permission(id).await?;

    Ok((StatusCode::OK, Json(api_response("权限删除成功".to_string()))))
}

/// 获取权限树
/// GET /api/v1/permissions/tree
pub async fn get_permission_tree() -> ApiResult<impl IntoResponse> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建权限服务
    let permission_service = PermissionService::new(db_conn.clone());

    // 获取权限树
    let result = permission_service.get_permission_tree().await?;

    Ok((StatusCode::OK, Json(api_response(result))))
}

/// 获取权限列表（平铺）
/// GET /api/v1/permissions/list
pub async fn get_permission_list() -> ApiResult<impl IntoResponse> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建权限服务
    let permission_service = PermissionService::new(db_conn.clone());

    // 获取权限列表
    let result = permission_service.get_permission_list().await?;

    Ok((StatusCode::OK, Json(api_response(result))))
}
