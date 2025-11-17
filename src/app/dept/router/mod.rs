/// 部门管理路由配置

use axum::{routing::{get, put}, Router};
use crate::app::dept::api::dept::{
    get_dept_tree, get_dept_list, get_dept,
    create_dept, update_dept, delete_dept,
    change_dept_status,
};

pub fn dept_routes() -> Router {
    Router::new()
        // 获取部门树形结构
        .route("/tree", get(get_dept_tree))
        // 获取部门列表（扁平列表，带分页）
        .route("/", get(get_dept_list).post(create_dept))
        // 部门详情路由（合并多个HTTP方法到同一路径）
        .route(
            "/{id}",
            get(get_dept)
                .put(update_dept)
                .delete(delete_dept)
        )
        // 更改部门状态
        .route("/{id}/status", put(change_dept_status))
}
