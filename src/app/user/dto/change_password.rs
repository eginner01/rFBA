/// 修改密码请求 DTO

use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct ChangePasswordRequest {
    /// 用户ID
    pub user_id: i64,

    /// 旧密码
    #[validate(length(min = 6, max = 128))]
    pub old_password: String,

    /// 新密码
    #[validate(length(min = 6, max = 128))]
    pub new_password: String,
}

/// 重置密码请求 DTO
#[derive(Debug, Deserialize, Validate)]
pub struct ResetPasswordRequest {
    /// 用户ID
    pub user_id: i64,

    /// 新密码
    #[validate(length(min = 6, max = 128))]
    pub new_password: String,
}
