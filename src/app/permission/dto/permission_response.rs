/// 权限响应 DTO

use serde::{Deserialize, Serialize};

/// 权限详情响应
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PermissionDetailResponse {
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
    /// 权限描述
    pub remark: Option<String>,
    /// 状态
    pub status: i32,
    /// 创建时间
    pub created_time: chrono::DateTime<chrono::Utc>,
    /// 更新时间
    pub updated_time: chrono::DateTime<chrono::Utc>,
}

/// 权限列表项响应
#[derive(Debug, Serialize, Deserialize)]
pub struct PermissionListItem {
    /// 权限ID
    pub id: i64,
    /// 权限名称
    pub name: String,
    /// 权限编码
    pub code: String,
    /// 权限类型
    pub permission_type: i32,
    /// 父权限ID
    pub parent_id: Option<i64>,
    /// 排序
    pub sort: i32,
    /// 权限描述
    pub remark: Option<String>,
    /// 状态
    pub status: i32,
    /// 创建时间
    pub created_time: chrono::DateTime<chrono::Utc>,
}

/// 权限树节点
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PermissionTreeNode {
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
    /// 状态
    pub status: i32,
    /// 子权限
    pub children: Vec<PermissionTreeNode>,
}
