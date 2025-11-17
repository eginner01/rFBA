/// 任务结果路由

use axum::{routing::*, Router};
use crate::app::task::api::{
    get_task_result,
    get_task_results_paginated,
    delete_task_result,
};

/// 创建任务结果路由
pub fn create_task_result_router() -> Router {
    Router::new()
        .route("/", get(get_task_results_paginated).delete(delete_task_result))
        .route("/{id}", get(get_task_result))
}
