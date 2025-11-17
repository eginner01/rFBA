/// 任务结果响应 DTO

use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

/// 任务结果详情响应
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskResultDetailResponse {
    /// 任务结果 ID
    pub id: i64,
    /// 任务 ID
    pub task_id: String,
    /// 执行状态
    pub status: String,
    /// 执行结果
    pub result: Option<serde_json::Value>,
    /// 结束时间
    pub date_done: Option<DateTime<Utc>>,
    /// 错误回溯
    pub traceback: Option<String>,
    /// 任务名称
    pub name: Option<String>,
    /// 任务位置参数
    pub args: Option<String>,
    /// 任务关键字参数
    pub kwargs: Option<String>,
    /// 运行 Worker
    pub worker: Option<String>,
    /// 重试次数
    pub retries: Option<i32>,
    /// 运行队列
    pub queue: Option<String>,
}

impl TaskResultDetailResponse {
    pub fn from_model(model: &crate::database::entity::task_result::Model) -> Self {
        // 解码参数（简化处理，实际应该使用与 Celery 相同的解码逻辑）
        let args_str = model.args.as_ref().map(|v| {
            String::from_utf8_lossy(v).to_string()
        });

        let kwargs_str = model.kwargs.as_ref().map(|v| {
            String::from_utf8_lossy(v).to_string()
        });

        Self {
            id: model.id,
            task_id: model.task_id.clone(),
            status: model.status.clone(),
            result: model.result.clone(),
            date_done: model.date_done,
            traceback: model.traceback.clone(),
            name: model.name.clone(),
            args: args_str,
            kwargs: kwargs_str,
            worker: model.worker.clone(),
            retries: model.retries,
            queue: model.queue.clone(),
        }
    }
}

/// 任务结果列表项
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskResultListItem {
    /// 任务结果 ID
    pub id: i64,
    /// 任务 ID
    pub task_id: String,
    /// 任务名称
    pub name: Option<String>,
    /// 执行状态
    pub status: String,
    /// 结束时间
    pub date_done: Option<DateTime<Utc>>,
    /// 重试次数
    pub retries: Option<i32>,
    /// 运行 Worker
    pub worker: Option<String>,
}

impl TaskResultListItem {
    pub fn from_model(model: &crate::database::entity::task_result::Model) -> Self {
        Self {
            id: model.id,
            task_id: model.task_id.clone(),
            name: model.name.clone(),
            status: model.status.clone(),
            date_done: model.date_done,
            retries: model.retries,
            worker: model.worker.clone(),
        }
    }
}
