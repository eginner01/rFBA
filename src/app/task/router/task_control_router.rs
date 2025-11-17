/// 任务控制路由

use axum::{routing::*, Router};
use crate::app::task::api::{get_registered_tasks, revoke_task};

/// 创建任务控制路由
pub fn create_task_control_router() -> Router {
    Router::new()
        .route("/registered", get(get_registered_tasks))
        .route("/{task_id}/cancel", delete(revoke_task))
}
