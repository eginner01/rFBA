/// 获取用户角色 DTO

use serde::{Deserialize, Serialize};

/// 获取用户角色响应
#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserRolesResponse {
    /// 用户ID
    pub user_id: i64,
    /// 角色列表
    pub roles: Vec<UserRoleInfo>,
}

/// 用户角色信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserRoleInfo {
    /// 角色ID
    pub id: i64,
    /// 角色名称
    pub name: String,
    /// 角色编码
    pub code: String,
    /// 状态
    pub status: i32,
    /// 分配时间
    pub assigned_time: chrono::DateTime<chrono::Utc>,
}
