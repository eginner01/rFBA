/// 用户分页查询 DTO

use serde::{Deserialize, Serialize};
use validator::Validate;

use super::user_response::UserListItem;

/// 用户分页查询请求
#[derive(Debug, Deserialize, Validate, Default)]
pub struct UserPaginationQuery {
    /// 页码（从1开始）
    #[validate(range(min = 1))]
    pub page: Option<usize>,

    /// 每页数量
    #[validate(range(min = 1, max = 100))]
    pub size: Option<usize>,

    /// 关键词搜索（用户名、昵称、邮箱、手机号）
    pub keyword: Option<String>,

    /// 用户名（模糊搜索）
    pub username: Option<String>,

    /// 昵称（模糊搜索）
    pub nickname: Option<String>,

    /// 邮箱（模糊搜索）
    pub email: Option<String>,

    /// 手机号（模糊搜索）
    pub phone: Option<String>,

    /// 状态（0: 禁用, 1: 启用）
    pub status: Option<i32>,

    /// 部门ID
    pub dept_id: Option<i64>,

    /// 是否超级管理员
    pub is_superuser: Option<bool>,

    /// 是否有后台管理权限
    pub is_staff: Option<bool>,

    /// 创建时间范围（开始）
    pub created_time_start: Option<chrono::DateTime<chrono::Utc>>,

    /// 创建时间范围（结束）
    pub created_time_end: Option<chrono::DateTime<chrono::Utc>>,

    /// 排序字段
    pub sort_by: Option<UserSortField>,

    /// 排序方向
    pub sort_order: Option<SortOrder>,
}

/// 排序字段
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[derive(Default)]
pub enum UserSortField {
    /// 按ID排序
    Id,
    /// 按用户名排序
    Username,
    /// 按昵称排序
    Nickname,
    /// 按状态排序
    Status,
    /// 按创建时间排序
    #[default]
    CreatedTime,
    /// 按更新时间排序
    UpdatedTime,
    /// 按最后登录时间排序
    LastLoginTime,
}


impl std::fmt::Display for UserSortField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserSortField::Id => write!(f, "id"),
            UserSortField::Username => write!(f, "username"),
            UserSortField::Nickname => write!(f, "nickname"),
            UserSortField::Status => write!(f, "status"),
            UserSortField::CreatedTime => write!(f, "created_time"),
            UserSortField::UpdatedTime => write!(f, "updated_time"),
            UserSortField::LastLoginTime => write!(f, "last_login_time"),
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

/// 用户分页查询响应
#[derive(Debug, Serialize)]
pub struct UserPaginationResponse {
    /// 用户列表
    pub list: Vec<UserListItem>,

    /// 总数量
    pub total: usize,

    /// 当前页码
    pub page: usize,

    /// 每页数量
    pub size: usize,

    /// 总页数
    pub pages: usize,
}
