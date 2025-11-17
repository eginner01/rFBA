/// 操作日志查询 DTO

use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::app::opera_log::dto::OperaLogListItem;

#[derive(Debug, Deserialize, Validate, Default)]
pub struct OperaLogPaginationQuery {
    /// 页码（从1开始）
    #[validate(range(min = 1))]
    pub page: Option<usize>,

    /// 每页数量
    #[validate(range(min = 1, max = 100))]
    pub size: Option<usize>,

    /// 关键词搜索（操作人名称、操作模块）
    pub keyword: Option<String>,

    /// 操作人名称
    pub user_name: Option<String>,

    /// 操作模块
    pub title: Option<String>,

    /// 业务类型
    pub business_type: Option<i32>,

    /// 操作状态（0: 正常, 1: 异常）
    pub status: Option<i32>,

    /// 操作员类型
    pub operator_type: Option<i32>,

    /// 开始时间
    pub start_time: Option<chrono::DateTime<chrono::Utc>>,

    /// 结束时间
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,

    /// 排序字段
    pub sort_by: Option<OperaLogSortField>,

    /// 排序方向
    pub sort_order: Option<SortOrder>,
}

/// 排序字段
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[derive(Default)]
pub enum OperaLogSortField {
    /// 按ID排序
    Id,
    /// 按操作模块排序
    Title,
    /// 按操作人排序
    UserName,
    /// 按业务类型排序
    BusinessType,
    /// 按操作时间排序
    CostTime,
    /// 按创建时间排序
    #[default]
    CreatedTime,
}


impl std::fmt::Display for OperaLogSortField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperaLogSortField::Id => write!(f, "id"),
            OperaLogSortField::Title => write!(f, "title"),
            OperaLogSortField::UserName => write!(f, "user_name"),
            OperaLogSortField::BusinessType => write!(f, "business_type"),
            OperaLogSortField::CostTime => write!(f, "cost_time"),
            OperaLogSortField::CreatedTime => write!(f, "created_time"),
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

/// 操作日志分页查询响应
#[derive(Debug, Serialize)]
pub struct OperaLogPaginationResponse {
    /// 操作日志列表
    pub items: Vec<OperaLogListItem>,

    /// 总数量
    pub total: usize,

    /// 当前页码
    pub page: usize,

    /// 每页数量
    pub size: usize,

    /// 总页数
    pub total_pages: usize,
}
