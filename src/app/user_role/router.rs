/// 组装所有用户-角色相关的API路由

use axum::{routing::{get, post}, Router};
use crate::app::user_role::api::{
    assign_user_roles, get_user_roles, check_user_has_role,
};

/// 组合用户-角色关联相关的路由
pub fn user_role_routes() -> Router {
    Router::new()
        .route("/user-roles/assign", post(assign_user_roles))
        .route("/user-roles/{user_id}", get(get_user_roles))
        .route("/user-roles/{user_id}/check/{role_code}", get(check_user_has_role))
}
