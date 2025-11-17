/// 任务控制相关 DTO

use serde::{Serialize, Deserialize};

/// 已注册任务详情
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisteredTaskDetail {
    /// 任务名称
    pub name: String,
    /// 任务标识
    pub task: String,
}
