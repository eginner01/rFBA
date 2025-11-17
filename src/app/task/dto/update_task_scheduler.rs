/// 更新任务调度器 DTO

use serde::{Deserialize, Serialize};
use validator::Validate;
use chrono::{DateTime, Utc};

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTaskSchedulerRequest {
    /// 任务名称
    #[validate(length(min = 1, max = 64))]
    pub name: String,

    /// 要运行的任务
    #[validate(length(min = 1, max = 256))]
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
    #[validate(range(min = 0, max = 1))]
    pub scheduler_type: i32,

    /// 任务再次运行前的间隔周期数
    pub interval_every: Option<i32>,

    /// 任务运行之间的周期类型
    pub interval_period: Option<String>,

    /// 运行的 Crontab 表达式
    pub crontab: Option<String>,

    /// 是否仅运行一次
    pub one_off: Option<bool>,

    /// 备注
    pub remark: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UpdateTaskSchedulerResponse {
    pub id: i64,
    pub name: String,
    pub updated_time: DateTime<Utc>,
}
