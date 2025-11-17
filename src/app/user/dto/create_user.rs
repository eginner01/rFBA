/// 创建用户请求 DTO

use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserRequest {
    /// 用户名
    #[validate(length(min = 3, max = 50))]
    pub username: String,

    /// 昵称
    #[validate(length(min = 1, max = 50))]
    pub nickname: String,

    /// 密码
    #[validate(length(min = 6, max = 128))]
    pub password: String,

    /// 邮箱
    #[validate(email)]
    pub email: Option<String>,

    /// 手机号
    pub phone: Option<String>,

    /// 头像URL
    pub avatar: Option<String>,

    /// 部门ID
    pub dept_id: Option<i64>,

    /// 角色ID列表
    pub role_ids: Option<Vec<i64>>,

    /// 是否超级管理员
    pub is_superuser: Option<bool>,

    /// 是否有后台管理权限
    pub is_staff: Option<bool>,

    /// 是否允许多端登录
    pub is_multi_login: Option<bool>,

    /// 状态（0: 禁用, 1: 启用）
    pub status: Option<i32>,
}

/// 用户创建响应 DTO
#[derive(Debug, Serialize)]
pub struct CreateUserResponse {
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
    /// 部门ID
    pub dept_id: Option<i64>,
    /// 状态
    pub status: i32,
    /// 创建时间
    pub created_time: chrono::DateTime<chrono::Utc>,
}
