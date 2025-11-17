/// 获取角色权限 DTO

use serde::{Deserialize, Serialize};

/// 获取角色权限响应
#[derive(Debug, Serialize, Deserialize)]
pub struct GetRolePermissionsResponse {
    /// 角色ID
    pub role_id: i64,
    /// 权限列表
    pub permissions: Vec<RolePermissionInfo>,
}

/// 角色权限信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RolePermissionInfo {
    /// 权限ID
    pub id: i64,
    /// 权限名称
    pub name: String,
    /// 权限编码
    pub code: String,
    /// 权限类型
    pub permission_type: i32,
    /// 状态
    pub status: i32,
    /// 分配时间
    pub assigned_time: chrono::DateTime<chrono::Utc>,
}
