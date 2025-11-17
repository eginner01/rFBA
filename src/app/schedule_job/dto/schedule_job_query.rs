/// 任务调度查询 DTO

use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate, Default)]
pub struct ScheduleJobPaginationQuery {
    /// 页码（从1开始）
    #[validate(range(min = 1))]
    pub page: Option<usize>,

    /// 每页数量
    #[validate(range(min = 1, max = 100))]
    pub size: Option<usize>,

    /// 关键词搜索（任务名称、任务组名、任务描述）
    pub keyword: Option<String>,

    /// 任务名称
    pub job_name: Option<String>,

    /// 任务组名
    pub job_group: Option<String>,

    /// 任务执行类
    pub bean_name: Option<String>,

    /// 任务状态（0: 正常, 1: 暂停）
    pub status: Option<i32>,

    /// 开始时间
    pub start_time: Option<chrono::DateTime<chrono::Utc>>,

    /// 结束时间
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,

    /// 排序字段
    pub sort_by: Option<ScheduleJobSortField>,

    /// 排序方向
    pub sort_order: Option<SortOrder>,
}

/// 排序字段
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScheduleJobSortField {
    /// 按ID排序
    Id,
    /// 按任务名称排序
    JobName,
    /// 按任务组名排序
    JobGroup,
    /// 按执行类排序
    BeanName,
    /// 按状态排序
    Status,
    /// 按创建时间排序
    CreatedTime,
    /// 按更新时间排序
    UpdatedTime,
}

impl Default for ScheduleJobSortField {
    fn default() -> Self {
        ScheduleJobSortField::CreatedTime
    }
}

impl std::fmt::Display for ScheduleJobSortField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScheduleJobSortField::Id => write!(f, "id"),
            ScheduleJobSortField::JobName => write!(f, "job_name"),
            ScheduleJobSortField::JobGroup => write!(f, "job_group"),
            ScheduleJobSortField::BeanName => write!(f, "bean_name"),
            ScheduleJobSortField::Status => write!(f, "status"),
            ScheduleJobSortField::CreatedTime => write!(f, "created_time"),
            ScheduleJobSortField::UpdatedTime => write!(f, "updated_time"),
        }
    }
}

/// 排序方向
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SortOrder {
    /// 升序
    Asc,
    /// 降序
    Desc,
}

impl Default for SortOrder {
    fn default() -> Self {
        SortOrder::Desc
    }
}

impl std::fmt::Display for SortOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SortOrder::Asc => write!(f, "asc"),
            SortOrder::Desc => write!(f, "desc"),
        }
    }
}

/// 任务调度分页查询响应
#[derive(Debug, Serialize)]
pub struct ScheduleJobPaginationResponse {
    /// 任务调度列表
    pub list: Vec<ScheduleJobListItem>,

    /// 总数量
    pub total: usize,

    /// 当前页码
    pub page: usize,

    /// 每页数量
    pub size: usize,

    /// 总页数
    pub pages: usize,
}

/// 任务调度列表项
#[derive(Debug, Serialize, Deserialize)]
pub struct ScheduleJobListItem {
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
    /// cron执行表达式
    pub cron_expression: String,
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
    /// 任务描述
    pub description: Option<String>,
    /// 创建时间
    pub created_time: chrono::DateTime<chrono::Utc>,
    /// 更新时间
    pub updated_time: chrono::DateTime<chrono::Utc>,
}
