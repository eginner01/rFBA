/// 分配角色权限 DTO

use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct AssignRolePermissionsRequest {
    /// 角色ID
    pub role_id: i64,
    /// 权限ID列表
    #[validate(length(min = 1))]
    pub permission_ids: Vec<i64>,
}

/// 分配角色权限响应
#[derive(Debug, Serialize)]
pub struct AssignRolePermissionsResponse {
    /// 角色ID
    pub role_id: i64,
    /// 分配的权限ID列表
    pub permission_ids: Vec<i64>,
    /// 分配时间
    pub assigned_time: chrono::DateTime<chrono::Utc>,
}
