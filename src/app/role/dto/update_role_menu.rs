/// 更新角色菜单 DTO
use serde::{Deserialize, Serialize};

/// 更新角色菜单请求
#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateRoleMenuRequest {
    /// 菜单 ID 列表
    pub menus: Vec<i64>,
}

/// 更新角色数据权限请求
#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateRoleScopeRequest {
    /// 数据范围 ID 列表
    pub scopes: Vec<i64>,
}
