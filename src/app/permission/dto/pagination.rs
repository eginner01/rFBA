/// 权限分页查询 DTO

use super::permission_response::PermissionListItem;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate, Default)]
pub struct PermissionPaginationQuery {
    /// 页码（从1开始）
    #[validate(range(min = 1))]
    pub page: Option<usize>,

    /// 每页数量
    #[validate(range(min = 1, max = 100))]
    pub size: Option<usize>,

    /// 关键词搜索（权限名称、权限编码）
    pub keyword: Option<String>,

    /// 权限名称（模糊搜索）
    pub name: Option<String>,

    /// 权限编码（模糊搜索）
    pub code: Option<String>,

    /// 权限类型（0: 目录, 1: 菜单, 2: 按钮）
    pub permission_type: Option<i32>,

    /// 状态（0: 禁用, 1: 启用）
    pub status: Option<i32>,

    /// 父权限ID
    pub parent_id: Option<i64>,

    /// 排序字段
    pub sort_by: Option<PermissionSortField>,

    /// 排序方向
    pub sort_order: Option<SortOrder>,
}

/// 排序字段
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[derive(Default)]
pub enum PermissionSortField {
    /// 按ID排序
    Id,
    /// 按权限名称排序
    Name,
    /// 按权限编码排序
    Code,
    /// 按权限类型排序
    PermissionType,
    /// 按排序号排序
    #[default]
    Sort,
    /// 按创建时间排序
    CreatedTime,
    /// 按更新时间排序
    UpdatedTime,
}


impl std::fmt::Display for PermissionSortField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PermissionSortField::Id => write!(f, "id"),
            PermissionSortField::Name => write!(f, "name"),
            PermissionSortField::Code => write!(f, "code"),
            PermissionSortField::PermissionType => write!(f, "permission_type"),
            PermissionSortField::Sort => write!(f, "sort"),
            PermissionSortField::CreatedTime => write!(f, "created_time"),
            PermissionSortField::UpdatedTime => write!(f, "updated_time"),
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

/// 权限分页查询响应
#[derive(Debug, Serialize)]
pub struct PermissionPaginationResponse {
    /// 权限列表
    pub list: Vec<PermissionListItem>,

    /// 总数量
    pub total: usize,

    /// 当前页码
    pub page: usize,

    /// 每页数量
    pub size: usize,

    /// 总页数
    pub pages: usize,
}
