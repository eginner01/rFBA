/// 任务调度器响应 DTO

use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

/// 任务调度器详情响应
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskSchedulerDetailResponse {
    /// 任务调度 ID
    pub id: i64,
    /// 任务名称
    pub name: String,
    /// 要运行的任务
    pub task: String,
    /// 任务可接收的位置参数
    pub args: Option<serde_json::Value>,
    /// 任务可接收的关键字参数
    pub kwargs: Option<serde_json::Value>,
    /// 队列名称
    pub queue: Option<String>,
    /// 低级别 AMQP 路由的交换机
    pub exchange: Option<String>,
    /// 低级别 AMQP 路由的路由密钥
    pub routing_key: Option<String>,
    /// 任务开始触发的时间
    pub start_time: Option<DateTime<Utc>>,
    /// 任务不再触发的截止时间
    pub expire_time: Option<DateTime<Utc>>,
    /// 任务不再触发的秒数时间差
    pub expire_seconds: Option<i32>,
    /// 任务调度类型（0间隔 1定时）
    #[serde(rename = "type")]
    pub scheduler_type: i32,
    /// 任务调度类型名称
    pub scheduler_type_name: String,
    /// 任务再次运行前的间隔周期数
    pub interval_every: Option<i32>,
    /// 任务运行之间的周期类型
    pub interval_period: Option<String>,
    /// 运行的 Crontab 表达式
    pub crontab: Option<String>,
    /// 是否仅运行一次
    pub one_off: bool,
    /// 是否启用任务
    pub enabled: bool,
    /// 已运行总次数
    pub total_run_count: i32,
    /// 最后运行时间
    pub last_run_time: Option<DateTime<Utc>>,
    /// 备注
    pub remark: Option<String>,
    /// 创建时间
    pub created_time: DateTime<Utc>,
    /// 更新时间
    pub updated_time: Option<DateTime<Utc>>,
}

impl TaskSchedulerDetailResponse {
    pub fn from_model(model: &crate::database::entity::task_scheduler::Model) -> Self {
        let scheduler_type_name = match model.scheduler_type {
            0 => "间隔调度".to_string(),
            1 => "定时调度".to_string(),
            _ => "未知".to_string(),
        };

        Self {
            id: model.id,
            name: model.name.clone(),
            task: model.task.clone(),
            args: model.args.clone(),
            kwargs: model.kwargs.clone(),
            queue: model.queue.clone(),
            exchange: model.exchange.clone(),
            routing_key: model.routing_key.clone(),
            start_time: model.start_time,
            expire_time: model.expire_time,
            expire_seconds: model.expire_seconds,
            scheduler_type: model.scheduler_type,
            scheduler_type_name,
            interval_every: model.interval_every,
            interval_period: model.interval_period.clone(),
            crontab: model.crontab.clone(),
            one_off: model.one_off,
            enabled: model.enabled,
            total_run_count: model.total_run_count,
            last_run_time: model.last_run_time,
            remark: model.remark.clone(),
            created_time: model.created_time,
            updated_time: model.updated_time,
        }
    }
}

/// 任务调度器列表项
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskSchedulerListItem {
    /// 任务调度 ID
    pub id: i64,
    /// 任务名称
    pub name: String,
    /// 要运行的任务
    pub task: String,
    /// 任务调度类型（0间隔 1定时）
    #[serde(rename = "type")]
    pub scheduler_type: i32,
    /// 任务调度类型名称
    pub scheduler_type_name: String,
    /// 是否启用任务
    pub enabled: bool,
    /// 是否启用任务名称
    pub enabled_name: String,
    /// 已运行总次数
    pub total_run_count: i32,
    /// 最后运行时间
    pub last_run_time: Option<DateTime<Utc>>,
    /// 备注
    pub remark: Option<String>,
    /// 创建时间
    pub created_time: DateTime<Utc>,
    /// 更新时间
    pub updated_time: Option<DateTime<Utc>>,
}

impl TaskSchedulerListItem {
    pub fn from_model(model: &crate::database::entity::task_scheduler::Model) -> Self {
        let scheduler_type_name = match model.scheduler_type {
            0 => "间隔调度".to_string(),
            1 => "定时调度".to_string(),
            _ => "未知".to_string(),
        };

        let enabled_name = if model.enabled { "启用".to_string() } else { "禁用".to_string() };

        Self {
            id: model.id,
            name: model.name.clone(),
            task: model.task.clone(),
            scheduler_type: model.scheduler_type,
            scheduler_type_name,
            enabled: model.enabled,
            enabled_name,
            total_run_count: model.total_run_count,
            last_run_time: model.last_run_time,
            remark: model.remark.clone(),
            created_time: model.created_time,
            updated_time: model.updated_time,
        }
    }
}
