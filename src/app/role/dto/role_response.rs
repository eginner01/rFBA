/// 角色响应 DTO

use serde::{Deserialize, Serialize};

/// 角色详情响应
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RoleDetailResponse {
    /// 角色ID
    pub id: i64,
    /// 角色名称
    pub name: String,
    /// 状态
    pub status: i32,
    /// 是否启用数据权限过滤
    pub is_filter_scopes: bool,
    /// 角色描述
    pub remark: Option<String>,
    /// 创建时间
    pub created_time: chrono::DateTime<chrono::Utc>,
    /// 更新时间
    pub updated_time: Option<chrono::DateTime<chrono::Utc>>,
}

/// 角色列表项响应
#[derive(Debug, Serialize, Deserialize)]
pub struct RoleListItem {
    /// 角色ID
    pub id: i64,
    /// 角色名称
    pub name: String,
    /// 状态
    pub status: i32,
    /// 是否启用数据权限过滤
    pub is_filter_scopes: bool,
    /// 角色描述
    pub remark: Option<String>,
    /// 创建时间
    pub created_time: chrono::DateTime<chrono::Utc>,
}

/// 角色权限树
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RolePermissionTree {
    /// 权限ID
    pub id: i64,
    /// 权限名称
    pub name: String,
    /// 权限编码
    pub code: String,
    /// 权限类型（0: 目录, 1: 菜单, 2: 按钮）
    pub permission_type: i32,
    /// 父权限ID
    pub parent_id: Option<i64>,
    /// 排序
    pub sort: i32,
    /// 是否已分配
    pub is_assigned: bool,
    /// 子权限
    pub children: Vec<RolePermissionTree>,
}
