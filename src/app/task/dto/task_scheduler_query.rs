/// 任务调度器查询参数 DTO

use serde::{Deserialize, Serialize};

/// 任务调度器列表查询参数
#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TaskSchedulerListQuery {
    /// 任务调度名称
    pub name: Option<String>,
    /// 任务调度类型（0间隔 1定时）
    pub scheduler_type: Option<i32>,
    /// 是否启用
    pub enabled: Option<bool>,
    /// 页码
    pub page: Option<i64>,
    /// 每页数量
    pub size: Option<i64>,
}
