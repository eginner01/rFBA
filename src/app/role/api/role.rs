/// 角色管理相关API

use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use crate::common::response::api_response;
use crate::app::role::dto::{
    CreateRoleRequest, UpdateRoleRequest, RolePaginationQuery,
};
use crate::app::role::service::RoleService;
use crate::database::DatabaseManager;
use crate::common::response::ApiResult;

/// 获取角色列表（分页）
/// GET /api/v1/roles
pub async fn get_roles(
    Query(query): Query<RolePaginationQuery>,
) -> ApiResult<impl IntoResponse> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建角色服务
    let role_service = RoleService::new(db_conn.clone());

    // 查询角色列表
    let result = role_service.get_roles_paginated(&query).await?;

    Ok((StatusCode::OK, Json(api_response(result))))
}

/// 获取角色详情
/// GET /api/v1/roles/{id}
pub async fn get_role(
    Path(id): Path<i64>,
) -> ApiResult<impl IntoResponse> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建角色服务
    let role_service = RoleService::new(db_conn.clone());

    // 查询角色详情
    let result = role_service.get_role_detail(id).await?;

    Ok((StatusCode::OK, Json(api_response(result))))
}

/// 创建角色
/// POST /api/v1/roles
pub async fn create_role(
    Json(request): Json<CreateRoleRequest>,
) -> ApiResult<impl IntoResponse> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建角色服务
    let role_service = RoleService::new(db_conn.clone());

    // 创建角色
    let result = role_service.create_role(&request).await?;

    Ok((StatusCode::CREATED, Json(api_response(result))))
}

/// 更新角色
/// PUT /api/v1/roles/{id}
pub async fn update_role(
    Path(id): Path<i64>,
    Json(request): Json<UpdateRoleRequest>,
) -> ApiResult<impl IntoResponse> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建角色服务
    let role_service = RoleService::new(db_conn.clone());

    // 更新角色
    role_service.update_role(id, &request).await?;

    Ok((StatusCode::OK, Json(api_response("角色更新成功".to_string()))))
}

/// 删除角色
/// DELETE /api/v1/roles/{id}
pub async fn delete_role(
    Path(id): Path<i64>,
) -> ApiResult<impl IntoResponse> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建角色服务
    let role_service = RoleService::new(db_conn.clone());

    // 删除角色
    role_service.delete_role(id).await?;

    Ok((StatusCode::OK, Json(api_response("角色删除成功".to_string()))))
}

/// 获取角色权限树
/// GET /api/v1/roles/{id}/permissions
pub async fn get_role_permissions(
    Path(id): Path<i64>,
) -> ApiResult<impl IntoResponse> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建角色服务
    let role_service = RoleService::new(db_conn.clone());

    // 获取角色权限树
    let result = role_service.get_role_permission_tree(id).await?;

    Ok((StatusCode::OK, Json(api_response(result))))
}

/// 分配角色权限
/// POST /api/v1/roles/{id}/permissions
pub async fn assign_role_permissions(
    Path(id): Path<i64>,
    Json(permission_ids): Json<Vec<i64>>,
) -> ApiResult<impl IntoResponse> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建角色服务
    let role_service = RoleService::new(db_conn.clone());

    // 分配角色权限
    role_service.assign_role_permissions(id, &permission_ids).await?;

    Ok((StatusCode::OK, Json(api_response("角色权限分配成功".to_string()))))
}
