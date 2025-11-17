/// 创建任务调度 DTO

use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateScheduleJobRequest {
    /// 任务名称
    #[validate(length(min = 1, max = 64))]
    pub job_name: String,

    /// 任务组名
    #[validate(length(min = 1, max = 64))]
    pub job_group: String,

    /// 任务执行类
    #[validate(length(min = 1, max = 128))]
    pub bean_name: String,

    /// 任务执行方法
    #[validate(length(min = 1, max = 64))]
    pub method_name: String,

    /// 任务参数
    pub method_params: Option<String>,

    /// cron执行表达式
    #[validate(length(min = 1, max = 128))]
    pub cron_expression: String,

    /// cron执行策略（0: 默认, 1: 立即触发执行, 2: 触发一次, 3: 不触发立即执行）
    pub misfire_policy: i32,

    /// 是否并发执行（0: 禁止, 1: 允许）
    pub concurrent: i32,

    /// 任务状态（0: 正常, 1: 暂停）
    pub status: i32,

    /// 任务执行优先级
    pub priority: i32,

    /// 任务执行超时时间（秒）
    pub timeout: Option<i32>,

    /// 重试次数
    pub retry_count: i32,

    /// 重试间隔（秒）
    pub retry_interval: i32,

    /// 任务描述
    pub description: Option<String>,
}

/// 任务调度创建响应
#[derive(Debug, Serialize)]
pub struct CreateScheduleJobResponse {
    /// 任务ID
    pub id: i64,
    /// 任务名称
    pub job_name: String,
    /// 任务组名
    pub job_group: String,
    /// cron执行表达式
    pub cron_expression: String,
    /// 创建时间
    pub created_time: chrono::DateTime<chrono::Utc>,
}

/// 任务调度更新请求
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateScheduleJobRequest {
    /// 任务ID
    pub id: i64,

    /// 任务名称
    #[validate(length(min = 1, max = 64))]
    pub job_name: String,

    /// 任务组名
    #[validate(length(min = 1, max = 64))]
    pub job_group: String,

    /// 任务执行类
    #[validate(length(min = 1, max = 128))]
    pub bean_name: String,

    /// 任务执行方法
    #[validate(length(min = 1, max = 64))]
    pub method_name: String,

    /// 任务参数
    pub method_params: Option<String>,

    /// cron执行表达式
    #[validate(length(min = 1, max = 128))]
    pub cron_expression: String,

    /// cron执行策略
    pub misfire_policy: i32,

    /// 是否并发执行
    pub concurrent: i32,

    /// 任务状态
    pub status: i32,

    /// 任务执行优先级
    pub priority: i32,

    /// 任务执行超时时间（秒）
    pub timeout: Option<i32>,

    /// 重试次数
    pub retry_count: i32,

    /// 重试间隔（秒）
    pub retry_interval: i32,

    /// 任务描述
    pub description: Option<String>,
}

/// 任务调度更新响应
#[derive(Debug, Serialize)]
pub struct UpdateScheduleJobResponse {
    /// 任务ID
    pub id: i64,
    /// 任务名称
    pub job_name: String,
    /// 任务组名
    pub job_group: String,
    /// 更新时间
    pub updated_time: chrono::DateTime<chrono::Utc>,
}

/// 任务调度立即执行请求
#[derive(Debug, Deserialize)]
pub struct ExecuteScheduleJobRequest {
    /// 任务ID
    pub id: i64,
}

/// 任务调度立即执行响应
#[derive(Debug, Serialize)]
pub struct ExecuteScheduleJobResponse {
    /// 执行ID
    pub execute_id: String,
    /// 任务ID
    pub job_id: i64,
    /// 任务名称
    pub job_name: String,
    /// 执行时间
    pub execute_time: chrono::DateTime<chrono::Utc>,
}
