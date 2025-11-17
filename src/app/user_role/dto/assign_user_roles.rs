/// 分配用户角色 DTO

use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct AssignUserRolesRequest {
    /// 用户ID
    pub user_id: i64,
    /// 角色ID列表
    #[validate(length(min = 1))]
    pub role_ids: Vec<i64>,
}

/// 分配用户角色响应
#[derive(Debug, Serialize)]
pub struct AssignUserRolesResponse {
    /// 用户ID
    pub user_id: i64,
    /// 分配的角色ID列表
    pub role_ids: Vec<i64>,
    /// 分配时间
    pub assigned_time: chrono::DateTime<chrono::Utc>,
}
