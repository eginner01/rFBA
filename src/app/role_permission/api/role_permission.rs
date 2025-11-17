/// 角色-权限关联相关API

use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use crate::common::response::ApiResult;
use crate::app::role_permission::dto::{
    AssignRolePermissionsRequest,
};
use crate::app::role_permission::service::RolePermissionService;
use crate::database::DatabaseManager;
use crate::common::response::api_response;

/// 分配角色权限
/// POST /api/v1/role-permissions/assign
pub async fn assign_role_permissions(
    Json(request): Json<AssignRolePermissionsRequest>,
) -> ApiResult<impl IntoResponse> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建角色-权限关联服务
    let role_permission_service = RolePermissionService::new(db_conn.clone());

    // 分配角色权限
    let result = role_permission_service.assign_role_permissions(&request).await?;

    Ok((StatusCode::OK, Json(api_response(result))))
}

/// 获取角色权限
/// GET /api/v1/role-permissions/{role_id}
pub async fn get_role_permissions(
    Path(role_id): Path<i64>,
) -> ApiResult<impl IntoResponse> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建角色-权限关联服务
    let role_permission_service = RolePermissionService::new(db_conn.clone());

    // 获取角色权限
    let result = role_permission_service.get_role_permissions(role_id).await?;

    Ok((StatusCode::OK, Json(api_response(result))))
}

/// 检查角色是否有指定权限
/// GET /api/v1/role-permissions/{role_id}/check/{permission_code}
pub async fn check_role_has_permission(
    Path((role_id, permission_code)): Path<(i64, String)>,
) -> ApiResult<impl IntoResponse> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建角色-权限关联服务
    let role_permission_service = RolePermissionService::new(db_conn.clone());

    // 检查角色是否有指定权限
    let has_permission = role_permission_service.role_has_permission(role_id, &permission_code).await?;

    Ok((StatusCode::OK, Json(api_response(has_permission))))
}
