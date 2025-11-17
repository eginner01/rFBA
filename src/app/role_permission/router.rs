/// 组装所有角色-权限相关的API路由

use axum::{routing::{get, post}, Router};
use crate::app::role_permission::api::{
    assign_role_permissions, get_role_permissions, check_role_has_permission,
};

/// 组合角色-权限关联相关的路由
pub fn role_permission_routes() -> Router {
    Router::new()
        .route("/role-permissions/assign", post(assign_role_permissions))
        .route("/role-permissions/{role_id}", get(get_role_permissions))
        .route("/role-permissions/{role_id}/check/{permission_code}", get(check_role_has_permission))
}
