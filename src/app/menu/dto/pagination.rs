/// 菜单分页查询 DTO

use super::menu_response::MenuListItem;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate, Default)]
pub struct MenuPaginationQuery {
    /// 页码（从1开始）
    #[validate(range(min = 1))]
    pub page: Option<usize>,

    /// 每页数量
    #[validate(range(min = 1, max = 100))]
    pub size: Option<usize>,

    /// 关键词搜索（菜单名称、路由路径）
    pub keyword: Option<String>,

    /// 菜单名称（模糊搜索）
    pub name: Option<String>,

    /// 路由路径（模糊搜索）
    pub path: Option<String>,

    /// 菜单类型（0: 目录, 1: 菜单, 2: 按钮）
    pub menu_type: Option<i32>,

    /// 状态（0: 禁用, 1: 启用）
    pub status: Option<i32>,

    /// 父菜单ID
    pub parent_id: Option<i64>,

    /// 排序字段
    pub sort_by: Option<MenuSortField>,

    /// 排序方向
    pub sort_order: Option<SortOrder>,
}

/// 排序字段
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[derive(Default)]
pub enum MenuSortField {
    /// 按ID排序
    Id,
    /// 按菜单名称排序
    Name,
    /// 按路由路径排序
    Path,
    /// 按菜单类型排序
    MenuType,
    /// 按排序号排序
    #[default]
    Sort,
    /// 按创建时间排序
    CreatedTime,
    /// 按更新时间排序
    UpdatedTime,
}


impl std::fmt::Display for MenuSortField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MenuSortField::Id => write!(f, "id"),
            MenuSortField::Name => write!(f, "name"),
            MenuSortField::Path => write!(f, "path"),
            MenuSortField::MenuType => write!(f, "menu_type"),
            MenuSortField::Sort => write!(f, "sort"),
            MenuSortField::CreatedTime => write!(f, "created_time"),
            MenuSortField::UpdatedTime => write!(f, "updated_time"),
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

/// 菜单分页查询响应
#[derive(Debug, Serialize)]
pub struct MenuPaginationResponse {
    /// 菜单列表
    pub list: Vec<MenuListItem>,

    /// 总数量
    pub total: usize,

    /// 当前页码
    pub page: usize,

    /// 每页数量
    pub size: usize,

    /// 总页数
    pub pages: usize,
}
