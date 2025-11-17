/// 任务调度器路由

use axum::{routing::*, Router};
use crate::app::task::api::{
    get_all_task_schedulers,
    get_task_scheduler,
    get_task_scheduler_paginated,
    create_task_scheduler,
    update_task_scheduler,
    update_task_scheduler_status,
    delete_task_scheduler,
    execute_task,
};

/// 创建任务调度器路由
pub fn create_task_scheduler_router() -> Router {
    Router::new()
        .route("/", get(get_task_scheduler_paginated).post(create_task_scheduler))
        .route("/all", get(get_all_task_schedulers))
        .route("/{id}", get(get_task_scheduler).put(update_task_scheduler).delete(delete_task_scheduler))
        .route("/{id}/status", put(update_task_scheduler_status))
        .route("/{id}/executions", post(execute_task))
}
