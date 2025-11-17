/// 用户-角色关联相关API

use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use crate::common::response::ApiResult;
use crate::app::user_role::dto::{
    AssignUserRolesRequest,
};
use crate::app::user_role::service::UserRoleService;
use crate::database::DatabaseManager;
use crate::common::response::api_response;

/// 分配用户角色
/// POST /api/v1/user-roles/assign
pub async fn assign_user_roles(
    Json(request): Json<AssignUserRolesRequest>,
) -> ApiResult<impl IntoResponse> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建用户-角色关联服务
    let user_role_service = UserRoleService::new(db_conn.clone());

    // 分配用户角色
    let result = user_role_service.assign_user_roles(&request).await?;

    Ok((StatusCode::OK, Json(api_response(result))))
}

/// 获取用户角色
/// GET /api/v1/user-roles/{user_id}
pub async fn get_user_roles(
    Path(user_id): Path<i64>,
) -> ApiResult<impl IntoResponse> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建用户-角色关联服务
    let user_role_service = UserRoleService::new(db_conn.clone());

    // 获取用户角色
    let result = user_role_service.get_user_roles(user_id).await?;

    Ok((StatusCode::OK, Json(api_response(result))))
}

/// 检查用户是否有指定角色
/// GET /api/v1/user-roles/{user_id}/check/{role_code}
pub async fn check_user_has_role(
    Path((user_id, role_code)): Path<(i64, String)>,
) -> ApiResult<impl IntoResponse> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建用户-角色关联服务
    let user_role_service = UserRoleService::new(db_conn.clone());

    // 检查用户是否有指定角色
    let has_role = user_role_service.user_has_role(user_id, &role_code).await?;

    Ok((StatusCode::OK, Json(api_response(has_role))))
}
