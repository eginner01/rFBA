/// 任务结果查询参数 DTO

use serde::{Deserialize, Serialize};

/// 任务结果列表查询参数
#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TaskResultListQuery {
    /// 任务名称
    pub name: Option<String>,
    /// 任务 ID
    pub task_id: Option<String>,
    /// 执行状态
    pub status: Option<String>,
    /// 页码
    pub page: Option<i64>,
    /// 每页数量
    pub size: Option<i64>,
}
