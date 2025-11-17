use axum::{routing::{get, delete}, Router};
use crate::app::opera_log::api::{
    get_opera_logs, get_opera_log,
    delete_opera_log, batch_delete_opera_logs,
    clear_opera_logs,
};

pub fn opera_log_routes() -> Router {
    Router::new()
        // GET /api/v1/logs/opera - 获取操作日志列表
        .route("/", get(get_opera_logs))
        // DELETE /api/v1/logs/opera - 批量删除操作日志
        .route("/", delete(batch_delete_opera_logs))
        // DELETE /api/v1/logs/opera/all - 清空所有操作日志
        .route("/all", delete(clear_opera_logs))
        // GET /api/v1/logs/opera/{id} - 获取操作日志详情
        .route("/{id}", get(get_opera_log))
        // DELETE /api/v1/logs/opera/{id} - 删除单个操作日志
        .route("/{id}", delete(delete_opera_log))
}
