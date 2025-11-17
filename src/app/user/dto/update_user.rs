/// 更新用户请求 DTO

use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, Default)]
pub struct UpdateUserRequest {
    /// 昵称
    #[validate(length(min = 1, max = 50))]
    pub nickname: Option<String>,

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
