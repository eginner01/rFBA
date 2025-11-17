/// 组装所有角色相关的API路由

use axum::{routing::{get, post, put, delete}, Router};
use axum::response::IntoResponse;
use axum::extract::Query;
use crate::app::role::dto::{
    CreateRoleRequest, UpdateRoleRequest, RolePaginationQuery,
};
use axum::{extract::Path, Json, http::StatusCode};
use crate::common::exception::AppError;
use crate::common::response::api_response;
use crate::database::DatabaseManager;

/// 组合角色管理相关的路由
/// 注意：此 Router 会被挂载到 `/api/v1/sys/roles` 下，所以这里的路径应为相对路径
pub fn role_routes() -> Router {
    Router::new()
        .route("/", get(get_roles_handler))  // GET /api/v1/sys/roles
        .route("/", post(create_role_handler))  // POST /api/v1/sys/roles
        .route("/", delete(batch_delete_roles_handler))  // DELETE /api/v1/sys/roles
        .route("/all", get(get_all_roles_handler))  // GET /api/v1/sys/roles/all
        .route("/{id}", get(get_role_handler))  // GET /api/v1/sys/roles/{id}
        .route("/{id}", put(update_role_handler))  // PUT /api/v1/sys/roles/{id}
        .route("/{id}", delete(delete_role_handler))  // DELETE /api/v1/sys/roles/{id}
        .route("/{id}/permissions", get(get_role_permissions_handler))
        .route("/{id}/permissions", post(assign_role_permissions_handler))
        .route("/{id}/menus", get(get_role_menus_handler))
        .route("/{id}/menus", put(update_role_menus_handler))
        .route("/{id}/scopes", get(get_role_scopes_handler))
        .route("/{id}/scopes", put(update_role_scopes_handler))
}

/// 获取角色列表（分页）
/// GET /api/v1/roles
async fn get_roles_handler(
    Query(query): Query<RolePaginationQuery>,
) -> Result<impl IntoResponse, AppError> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建角色服务
    let role_service = crate::app::role::service::RoleService::new(db_conn.clone());

    // 查询角色列表
    let result = role_service.get_roles_paginated(&query).await?;

    Ok((StatusCode::OK, Json(api_response(result))))
}

/// 获取角色详情
/// GET /api/v1/roles/{id}
async fn get_role_handler(
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建角色服务
    let role_service = crate::app::role::service::RoleService::new(db_conn.clone());

    // 查询角色详情
    let result = role_service.get_role_detail(id).await?;

    Ok((StatusCode::OK, Json(api_response(result))))
}

/// 创建角色
/// POST /api/v1/roles
async fn create_role_handler(
    Json(request): Json<CreateRoleRequest>,
) -> Result<impl IntoResponse, AppError> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建角色服务
    let role_service = crate::app::role::service::RoleService::new(db_conn.clone());

    // 创建角色
    let result = role_service.create_role(&request).await?;

    Ok((StatusCode::CREATED, Json(api_response(result))))
}

/// 更新角色
/// PUT /api/v1/roles/{id}
async fn update_role_handler(
    Path(id): Path<i64>,
    Json(request): Json<UpdateRoleRequest>,
) -> Result<impl IntoResponse, AppError> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建角色服务
    let role_service = crate::app::role::service::RoleService::new(db_conn.clone());

    // 更新角色
    role_service.update_role(id, &request).await?;

    Ok((StatusCode::OK, Json(api_response("角色更新成功".to_string()))))
}

/// 删除角色
/// DELETE /api/v1/roles/{id}
async fn delete_role_handler(
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建角色服务
    let role_service = crate::app::role::service::RoleService::new(db_conn.clone());

    // 删除角色
    role_service.delete_role(id).await?;

    Ok((StatusCode::OK, Json(api_response("角色删除成功".to_string()))))
}

/// 获取角色权限树
/// GET /api/v1/roles/{id}/permissions
async fn get_role_permissions_handler(
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建角色服务
    let role_service = crate::app::role::service::RoleService::new(db_conn.clone());

    // 获取角色权限树
    let result = role_service.get_role_permission_tree(id).await?;

    Ok((StatusCode::OK, Json(api_response(result))))
}

/// 分配角色权限
/// POST /api/v1/roles/{id}/permissions
async fn assign_role_permissions_handler(
    Path(id): Path<i64>,
    Json(permission_ids): Json<Vec<i64>>,
) -> Result<impl IntoResponse, AppError> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建角色服务
    let role_service = crate::app::role::service::RoleService::new(db_conn.clone());

    // 分配角色权限
    role_service.assign_role_permissions(id, &permission_ids).await?;

    Ok((StatusCode::OK, Json(api_response("角色权限分配成功".to_string()))))
}

/// 批量删除角色
/// DELETE /api/v1/sys/roles
async fn batch_delete_roles_handler(
    Json(role_ids): Json<Vec<i64>>,
) -> Result<impl IntoResponse, AppError> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建角色服务
    let role_service = crate::app::role::service::RoleService::new(db_conn.clone());

    // 批量删除角色
    role_service.batch_delete_roles(&role_ids).await?;

    Ok((StatusCode::OK, Json(api_response("批量删除成功".to_string()))))
}

/// 获取所有角色
/// GET /api/v1/sys/roles/all
async fn get_all_roles_handler() -> Result<impl IntoResponse, AppError> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建角色服务
    let role_service = crate::app::role::service::RoleService::new(db_conn.clone());

    // 获取所有角色
    let result = role_service.get_all_roles().await?;

    Ok((StatusCode::OK, Json(api_response(result))))
}

/// 获取角色菜单
/// GET /api/v1/sys/roles/{id}/menus
async fn get_role_menus_handler(
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建角色服务
    let role_service = crate::app::role::service::RoleService::new(db_conn.clone());

    // 获取角色菜单树
    let result = role_service.get_role_menus(id).await?;

    Ok((StatusCode::OK, Json(api_response(result))))
}

/// 更新角色菜单
/// PUT /api/v1/sys/roles/{id}/menus
async fn update_role_menus_handler(
    Path(id): Path<i64>,
    Json(request): Json<crate::app::role::dto::UpdateRoleMenuRequest>,
) -> Result<impl IntoResponse, AppError> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建角色服务
    let role_service = crate::app::role::service::RoleService::new(db_conn.clone());

    // 更新角色菜单
    role_service.update_role_menus(id, &request.menus).await?;

    Ok((StatusCode::OK, Json(api_response("角色菜单更新成功".to_string()))))
}

/// 获取角色数据权限
/// GET /api/v1/sys/roles/{id}/scopes
async fn get_role_scopes_handler(
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建角色服务
    let role_service = crate::app::role::service::RoleService::new(db_conn.clone());

    // 获取角色数据权限
    let result = role_service.get_role_scopes(id).await?;

    Ok((StatusCode::OK, Json(api_response(result))))
}

/// 更新角色数据权限
/// PUT /api/v1/sys/roles/{id}/scopes
async fn update_role_scopes_handler(
    Path(id): Path<i64>,
    Json(request): Json<crate::app::role::dto::UpdateRoleScopeRequest>,
) -> Result<impl IntoResponse, AppError> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建角色服务
    let role_service = crate::app::role::service::RoleService::new(db_conn.clone());

    // 更新角色数据权限
    role_service.update_role_scopes(id, &request.scopes).await?;

    Ok((StatusCode::OK, Json(api_response("角色数据权限更新成功".to_string()))))
}
