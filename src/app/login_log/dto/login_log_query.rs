/// 登录日志查询 DTO

use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::app::login_log::dto::LoginLogListItem;

#[derive(Debug, Deserialize, Validate, Default)]
pub struct LoginLogPaginationQuery {
    /// 页码（从1开始）
    #[validate(range(min = 1))]
    pub page: Option<usize>,

    /// 每页数量
    #[validate(range(min = 1, max = 100))]
    pub size: Option<usize>,

    /// 关键词搜索（用户名、登录IP、登录地点）
    pub keyword: Option<String>,

    /// 用户名
    pub username: Option<String>,

    /// 登录IP
    /// 前端查询参数为 `ip`，通过 serde 重命名映射到此字段
    #[serde(rename = "ip")]
    pub ipaddr: Option<String>,

    /// 登录状态
    pub status: Option<i32>,

    /// 开始时间
    pub start_time: Option<chrono::DateTime<chrono::Utc>>,

    /// 结束时间
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,

    /// 排序字段
    pub sort_by: Option<LoginLogSortField>,

    /// 排序方向
    pub sort_order: Option<SortOrder>,
}

/// 排序字段
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[derive(Default)]
pub enum LoginLogSortField {
    /// 按ID排序
    Id,
    /// 按用户名排序
    Username,
    /// 按登录IP排序
    Ipaddr,
    /// 按登录状态排序
    Status,
    /// 按登录时间排序
    LoginTime,
    /// 按访问时间排序
    #[default]
    AccessTime,
}


impl std::fmt::Display for LoginLogSortField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoginLogSortField::Id => write!(f, "id"),
            LoginLogSortField::Username => write!(f, "username"),
            LoginLogSortField::Ipaddr => write!(f, "ipaddr"),
            LoginLogSortField::Status => write!(f, "status"),
            LoginLogSortField::LoginTime => write!(f, "login_time"),
            LoginLogSortField::AccessTime => write!(f, "access_time"),
        }
    }
}

/// 排序方向
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[derive(Default)]
pub enum SortOrder {
    /// 升序
    Asc,
    /// 降序
    #[default]
    Desc,
}


impl std::fmt::Display for SortOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SortOrder::Asc => write!(f, "asc"),
            SortOrder::Desc => write!(f, "desc"),
        }
    }
}

/// 登录日志分页查询响应
#[derive(Debug, Serialize)]
pub struct LoginLogPaginationResponse {
    /// 登录日志列表
    pub items: Vec<LoginLogListItem>,

    /// 总数量
    pub total: usize,

    /// 当前页码
    pub page: usize,

    /// 每页数量
    pub size: usize,

    /// 总页数
    pub total_pages: usize,
}
