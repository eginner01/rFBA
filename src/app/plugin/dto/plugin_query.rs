/// 插件查询 DTO

use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate, Default)]
pub struct PluginPaginationQuery {
    /// 页码（从1开始）
    #[validate(range(min = 1))]
    pub page: Option<usize>,

    /// 每页数量
    #[validate(range(min = 1, max = 100))]
    pub size: Option<usize>,

    /// 关键词搜索（插件名称、插件编码、插件描述）
    pub keyword: Option<String>,

    /// 插件名称
    pub name: Option<String>,

    /// 插件编码
    pub code: Option<String>,

    /// 插件类型
    pub plugin_type: Option<i32>,

    /// 插件状态（0: 禁用, 1: 启用, 2: 已卸载）
    pub status: Option<i32>,

    /// 是否系统插件（0: 否, 1: 是）
    pub is_system: Option<i32>,

    /// 插件作者
    pub author: Option<String>,

    /// 开始时间
    pub start_time: Option<chrono::DateTime<chrono::Utc>>,

    /// 结束时间
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,

    /// 排序字段
    pub sort_by: Option<PluginSortField>,

    /// 排序方向
    pub sort_order: Option<SortOrder>,
}

/// 排序字段
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[derive(Default)]
pub enum PluginSortField {
    /// 按ID排序
    Id,
    /// 按插件名称排序
    Name,
    /// 按插件编码排序
    Code,
    /// 按插件类型排序
    PluginType,
    /// 按状态排序
    Status,
    /// 按排序排序
    #[default]
    SortOrder,
    /// 按创建时间排序
    CreatedTime,
    /// 按更新时间排序
    UpdatedTime,
}


impl std::fmt::Display for PluginSortField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PluginSortField::Id => write!(f, "id"),
            PluginSortField::Name => write!(f, "name"),
            PluginSortField::Code => write!(f, "code"),
            PluginSortField::PluginType => write!(f, "plugin_type"),
            PluginSortField::Status => write!(f, "status"),
            PluginSortField::SortOrder => write!(f, "sort_order"),
            PluginSortField::CreatedTime => write!(f, "created_time"),
            PluginSortField::UpdatedTime => write!(f, "updated_time"),
        }
    }
}

/// 排序方向
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[derive(Default)]
pub enum SortOrder {
    /// 升序
    #[default]
    Asc,
    /// 降序
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

/// 插件分页查询响应
#[derive(Debug, Serialize)]
pub struct PluginPaginationResponse {
    /// 插件列表
    pub list: Vec<PluginListItem>,

    /// 总数量
    pub total: usize,

    /// 当前页码
    pub page: usize,

    /// 每页数量
    pub size: usize,

    /// 总页数
    pub pages: usize,
}

/// 插件列表项
#[derive(Debug, Serialize, Deserialize)]
pub struct PluginListItem {
    /// 插件ID
    pub id: i64,
    /// 插件名称
    pub name: String,
    /// 插件编码
    pub code: String,
    /// 插件版本
    pub version: String,
    /// 插件类型
    pub plugin_type: i32,
    /// 插件类型名称
    pub plugin_type_name: String,
    /// 插件描述
    pub description: Option<String>,
    /// 插件作者
    pub author: Option<String>,
    /// 插件状态
    pub status: i32,
    /// 插件状态名称
    pub status_name: String,
    /// 插件排序
    pub sort_order: i32,
    /// 是否系统插件
    pub is_system: i32,
    /// 安装时间
    pub install_time: Option<chrono::DateTime<chrono::Utc>>,
    /// 创建时间
    pub created_time: chrono::DateTime<chrono::Utc>,
    /// 更新时间
    pub updated_time: chrono::DateTime<chrono::Utc>,
}
