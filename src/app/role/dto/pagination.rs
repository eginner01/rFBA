/// 角色分页查询 DTO

use serde::{Deserialize, Serialize};
use validator::Validate;

use super::role_response::RoleListItem;

#[derive(Debug, Deserialize, Validate, Default)]
pub struct RolePaginationQuery {
    /// 页码（从1开始）
    #[validate(range(min = 1))]
    pub page: Option<usize>,

    /// 每页数量
    #[validate(range(min = 1, max = 100))]
    pub size: Option<usize>,

    /// 关键词搜索（角色名称、角色编码）
    pub keyword: Option<String>,

    /// 角色名称（模糊搜索）
    pub name: Option<String>,

    /// 角色编码（模糊搜索）
    pub code: Option<String>,

    /// 状态（0: 禁用, 1: 启用）
    pub status: Option<i32>,

    /// 排序字段
    pub sort_by: Option<RoleSortField>,

    /// 排序方向
    pub sort_order: Option<SortOrder>,
}

/// 排序字段
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[derive(Default)]
pub enum RoleSortField {
    /// 按ID排序
    #[default]
    Id,
    /// 按角色名称排序
    Name,
    /// 按状态排序
    Status,
    /// 按创建时间排序
    CreatedTime,
    /// 按更新时间排序
    UpdatedTime,
}


impl std::fmt::Display for RoleSortField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RoleSortField::Id => write!(f, "id"),
            RoleSortField::Name => write!(f, "name"),
            RoleSortField::Status => write!(f, "status"),
            RoleSortField::CreatedTime => write!(f, "created_time"),
            RoleSortField::UpdatedTime => write!(f, "updated_time"),
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

/// 角色分页查询响应
#[derive(Debug, Serialize)]
pub struct RolePaginationResponse {
    /// 角色列表
    pub list: Vec<RoleListItem>,

    /// 总数量
    pub total: usize,

    /// 当前页码
    pub page: usize,

    /// 每页数量
    pub size: usize,

    /// 总页数
    pub pages: usize,
}
