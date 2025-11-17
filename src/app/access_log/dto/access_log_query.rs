/// 访问日志查询 DTO

use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate, Default)]
pub struct AccessLogPaginationQuery {
    /// 页码（从1开始）
    #[validate(range(min = 1))]
    pub page: Option<usize>,

    /// 每页数量
    #[validate(range(min = 1, max = 100))]
    pub size: Option<usize>,

    /// 关键词搜索（用户名称、URL、客户端IP）
    pub keyword: Option<String>,

    /// 用户名
    pub user_name: Option<String>,

    /// 请求URL
    pub url: Option<String>,

    /// 请求方法
    pub method: Option<String>,

    /// 客户端IP
    pub client_ip: Option<String>,

    /// 响应状态码
    pub status_code: Option<u16>,

    /// 是否异常
    pub is_error: Option<bool>,

    /// 开始时间
    pub start_time: Option<chrono::DateTime<chrono::Utc>>,

    /// 结束时间
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,

    /// 排序字段
    pub sort_by: Option<AccessLogSortField>,

    /// 排序方向
    pub sort_order: Option<SortOrder>,
}

/// 排序字段
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccessLogSortField {
    /// 按ID排序
    Id,
    /// 按用户排序
    UserName,
    /// 按URL排序
    Url,
    /// 按方法排序
    Method,
    /// 按状态码排序
    StatusCode,
    /// 按响应时间排序
    CostTime,
    /// 按访问时间排序
    AccessTime,
}

impl Default for AccessLogSortField {
    fn default() -> Self {
        AccessLogSortField::AccessTime
    }
}

impl std::fmt::Display for AccessLogSortField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AccessLogSortField::Id => write!(f, "id"),
            AccessLogSortField::UserName => write!(f, "user_name"),
            AccessLogSortField::Url => write!(f, "url"),
            AccessLogSortField::Method => write!(f, "method"),
            AccessLogSortField::StatusCode => write!(f, "status_code"),
            AccessLogSortField::CostTime => write!(f, "cost_time"),
            AccessLogSortField::AccessTime => write!(f, "access_time"),
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

/// 访问日志分页查询响应
#[derive(Debug, Serialize)]
pub struct AccessLogPaginationResponse {
    /// 访问日志列表
    pub list: Vec<AccessLogListItem>,

    /// 总数量
    pub total: usize,

    /// 当前页码
    pub page: usize,

    /// 每页数量
    pub size: usize,

    /// 总页数
    pub pages: usize,
}
