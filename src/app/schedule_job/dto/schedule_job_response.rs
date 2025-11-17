/// 任务调度响应 DTO

use serde::{Deserialize, Serialize};

/// 任务调度详情响应
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScheduleJobDetailResponse {
    /// 任务ID
    pub id: i64,
    /// 任务名称
    pub job_name: String,
    /// 任务组名
    pub job_group: String,
    /// 任务执行类
    pub bean_name: String,
    /// 任务执行方法
    pub method_name: String,
    /// 任务参数
    pub method_params: Option<String>,
    /// cron执行表达式
    pub cron_expression: String,
    /// cron执行策略
    pub misfire_policy: i32,
    /// cron执行策略名称
    pub misfire_policy_name: String,
    /// 是否并发执行
    pub concurrent: i32,
    /// 任务状态
    pub status: i32,
    /// 任务状态名称
    pub status_name: String,
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
    /// 创建人
    pub create_by: Option<String>,
    /// 更新人
    pub update_by: Option<String>,
    /// 创建时间
    pub created_time: chrono::DateTime<chrono::Utc>,
    /// 更新时间
    pub updated_time: chrono::DateTime<chrono::Utc>,
}

/// 任务调度统计
#[derive(Debug, Serialize, Deserialize)]
pub struct ScheduleJobStatistics {
    /// 总任务数
    pub total_count: usize,
    /// 正常任务数
    pub normal_count: usize,
    /// 暂停任务数
    pub paused_count: usize,
    /// 今日执行次数
    pub today_execute_count: usize,
    /// 今日成功次数
    pub today_success_count: usize,
    /// 今日失败次数
    pub today_failure_count: usize,
    /// 本周执行次数
    pub week_execute_count: usize,
    /// 本月执行次数
    pub month_execute_count: usize,
}

/// 任务执行日志查询 DTO
#[derive(Debug, Deserialize, Validate, Default)]
pub struct ScheduleJobLogPaginationQuery {
    /// 页码（从1开始）
    #[validate(range(min = 1))]
    pub page: Option<usize>,

    /// 每页数量
    #[validate(range(min = 1, max = 100))]
    pub size: Option<usize>,

    /// 任务ID
    pub job_id: Option<i64>,

    /// 任务名称
    pub job_name: Option<String>,

    /// 执行状态（0: 成功, 1: 失败）
    pub status: Option<i32>,

    /// 开始时间
    pub start_time: Option<chrono::DateTime<chrono::Utc>>,

    /// 结束时间
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,

    /// 排序字段
    pub sort_by: Option<ScheduleJobLogSortField>,

    /// 排序方向
    pub sort_order: Option<SortOrder>,
}

/// 任务执行日志排序字段
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScheduleJobLogSortField {
    /// 按ID排序
    Id,
    /// 按任务ID排序
    JobId,
    /// 按执行时间排序
    ExecuteTime,
    /// 按耗时排序
    CostTime,
    /// 按状态排序
    Status,
}

impl Default for ScheduleJobLogSortField {
    fn default() -> Self {
        ScheduleJobLogSortField::ExecuteTime
    }
}

/// 任务执行日志分页响应
#[derive(Debug, Serialize)]
pub struct ScheduleJobLogPaginationResponse {
    /// 任务执行日志列表
    pub list: Vec<ScheduleJobLogListItem>,

    /// 总数量
    pub total: usize,

    /// 当前页码
    pub page: usize,

    /// 每页数量
    pub size: usize,

    /// 总页数
    pub pages: usize,
}

/// 任务执行日志列表项
#[derive(Debug, Serialize, Deserialize)]
pub struct ScheduleJobLogListItem {
    /// 日志ID
    pub id: i64,
    /// 任务ID
    pub job_id: i64,
    /// 任务名称
    pub job_name: String,
    /// 任务组名
    pub job_group: String,
    /// 任务执行类
    pub bean_name: String,
    /// 任务执行方法
    pub method_name: String,
    /// 执行状态
    pub status: i32,
    /// 执行状态名称
    pub status_name: String,
    /// 执行耗时（毫秒）
    pub cost_time: i64,
    /// 执行时间
    pub execute_time: chrono::DateTime<chrono::Utc>,
    /// 开始时间
    pub start_time: chrono::DateTime<chrono::Utc>,
    /// 结束时间
    pub end_time: chrono::DateTime<chrono::Utc>,
    /// 执行机器IP
    pub machine_ip: Option<String>,
    /// 任务参数
    pub method_params: Option<String>,
}

/// 任务执行日志详情响应
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScheduleJobLogDetailResponse {
    /// 日志ID
    pub id: i64,
    /// 任务ID
    pub job_id: i64,
    /// 任务名称
    pub job_name: String,
    /// 任务组名
    pub job_group: String,
    /// 任务执行类
    pub bean_name: String,
    /// 任务执行方法
    pub method_name: String,
    /// 任务参数
    pub method_params: Option<String>,
    /// 执行状态
    pub status: i32,
    /// 执行状态名称
    pub status_name: String,
    /// 异常信息
    pub exception: Option<String>,
    /// 异常信息详情
    pub exception_detail: Option<String>,
    /// 执行耗时（毫秒）
    pub cost_time: i64,
    /// 执行时间
    pub execute_time: chrono::DateTime<chrono::Utc>,
    /// 开始时间
    pub start_time: chrono::DateTime<chrono::Utc>,
    /// 结束时间
    pub end_time: chrono::DateTime<chrono::Utc>,
    /// 任务参数
    pub job_params: Option<String>,
    /// 执行机器IP
    pub machine_ip: Option<String>,
    /// 执行机器名
    pub machine_name: Option<String>,
}
