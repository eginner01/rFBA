/// 任务主路由
/// 聚合所有任务相关路由

use axum::Router;
use crate::app::task::router::{
    task_scheduler_router::create_task_scheduler_router,
    task_result_router::create_task_result_router,
    task_control_router::create_task_control_router,
};

/// 创建任务相关路由
pub fn task_routes() -> Router {
    Router::new()
        .nest("/tasks/schedulers", create_task_scheduler_router())
        .nest("/tasks/results", create_task_result_router())
        // 任务控制路由直接挂载到 /tasks 下（/tasks/registered, /tasks/{task_id}/cancel）
        .merge(Router::new().nest("/tasks", create_task_control_router()))
}
