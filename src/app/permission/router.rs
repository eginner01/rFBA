/// 组装所有权限相关的API路由

use axum::{routing::{get, post, put, delete}, Router};
use crate::app::permission::api::{
    get_permissions, get_permission, create_permission, update_permission, delete_permission,
    get_permission_tree, get_permission_list,
};

/// 组合权限管理相关的路由
pub fn permission_routes() -> Router {
    Router::new()
        .route("/", get(get_permissions))
        .route("/", post(create_permission))
        .route("/{id}", get(get_permission))
        .route("/{id}", put(update_permission))
        .route("/{id}", delete(delete_permission))
        .route("/tree", get(get_permission_tree))
        .route("/list", get(get_permission_list))
}
