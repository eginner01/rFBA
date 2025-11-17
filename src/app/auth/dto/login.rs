/// 登录请求 DTO

use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    /// 用户名
    #[validate(length(min = 3, max = 50))]
    pub username: String,

    #[serde(alias = "selectAccount")]
    pub select_account: Option<String>,

    /// 密码
    #[validate(length(min = 6, max = 128))]
    pub password: String,

    /// 验证码（可选）
    pub captcha: Option<String>,

    /// 验证码key（可选）
    #[serde(alias = "uuid")]
    pub captcha_key: Option<String>,
}

/// 登录响应 DTO - 匹配Python后端的GetLoginToken
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    /// 访问令牌
    pub access_token: String,
    /// 令牌过期时间
    pub access_token_expire_time: chrono::NaiveDateTime,
    /// 会话 UUID
    pub session_uuid: String,
    /// 用户信息
    pub user: UserInfo,
}

/// 用户信息 DTO - 匹配Python后端的GetUserInfoDetail
#[derive(Debug, Serialize)]
pub struct UserInfo {
    /// 用户ID
    pub id: i64,
    /// 用户UUID
    pub uuid: String,
    /// 用户名
    pub username: String,
    /// 昵称
    pub nickname: String,
    /// 头像
    pub avatar: Option<String>,
    /// 邮箱
    pub email: Option<String>,
    /// 手机号
    pub phone: Option<String>,
    /// 部门ID
    pub dept_id: Option<i64>,
    /// 状态（1:正常 0:停用）
    pub status: i32,
    /// 是否超级管理员
    pub is_superuser: bool,
    /// 是否管理员
    pub is_staff: bool,
    /// 是否允许多端登录
    pub is_multi_login: bool,
    /// 加入时间
    pub join_time: chrono::NaiveDateTime,
    /// 最后登录时间
    pub last_login_time: Option<chrono::NaiveDateTime>,
    /// 部门名称（扁平化显示）
    pub dept: Option<String>,
    /// 角色名称列表（扁平化显示）
    pub roles: Vec<String>,
}
