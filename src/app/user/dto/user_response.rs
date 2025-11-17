/// 用户响应 DTO

use serde::{Deserialize, Serialize};

/// 用户详情响应
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserDetailResponse {
    /// 用户ID
    pub id: i64,
    /// UUID
    pub uuid: String,
    /// 用户名
    pub username: String,
    /// 昵称
    pub nickname: String,
    /// 邮箱
    pub email: Option<String>,
    /// 手机号
    pub phone: Option<String>,
    /// 头像URL
    pub avatar: Option<String>,
    /// 状态
    pub status: i32,
    /// 是否超级管理员
    pub is_superuser: bool,
    /// 是否有后台管理权限
    pub is_staff: bool,
    /// 是否允许多端登录
    pub is_multi_login: bool,
    /// 注册时间
    pub join_time: chrono::DateTime<chrono::Utc>,
    /// 上次登录时间
    pub last_login_time: Option<chrono::DateTime<chrono::Utc>>,
    /// 部门ID
    pub dept_id: Option<i64>,
    /// 部门名称
    pub dept_name: Option<String>,
    /// 角色列表
    pub roles: Vec<String>,
    /// 创建时间
    pub created_time: chrono::DateTime<chrono::Utc>,
    /// 更新时间
    pub updated_time: chrono::DateTime<chrono::Utc>,
}

/// 用户列表项响应
#[derive(Debug, Serialize, Deserialize)]
pub struct UserListItem {
    /// 用户ID
    pub id: i64,
    /// 用户名
    pub username: String,
    /// 昵称
    pub nickname: String,
    /// 邮箱
    pub email: Option<String>,
    /// 手机号
    pub phone: Option<String>,
    /// 头像URL
    pub avatar: Option<String>,
    /// 状态
    pub status: i32,
    /// 部门名称
    pub dept_name: Option<String>,
    /// 角色列表
    pub roles: Vec<String>,
    /// 上次登录时间
    pub last_login_time: Option<chrono::DateTime<chrono::Utc>>,
    /// 创建时间
    pub created_time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CurrentUserResponse {
    pub id: i64,
    pub uuid: String,
    pub username: String,
    pub nickname: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub avatar: Option<String>,
    pub status: i32,
    pub is_superuser: bool,
    pub is_staff: bool,
    pub is_multi_login: bool,
    pub join_time: chrono::DateTime<chrono::Utc>,
    pub last_login_time: Option<chrono::DateTime<chrono::Utc>>,
    pub dept_id: Option<i64>,
    pub dept: Option<String>,
    pub roles: Vec<String>,
}
