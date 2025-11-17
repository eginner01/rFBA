/// 创建权限请求 DTO

use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreatePermissionRequest {
    /// 权限名称
    #[validate(length(min = 1, max = 50))]
    pub name: String,

    /// 权限编码
    #[validate(length(min = 1, max = 100))]
    pub code: String,

    /// 权限类型（0: 目录, 1: 菜单, 2: 按钮）
    pub permission_type: i32,

    /// 父权限ID
    pub parent_id: Option<i64>,

    /// 排序
    pub sort: Option<i32>,

    /// 权限描述
    pub remark: Option<String>,

    /// 是否启用
    pub status: Option<i32>,
}

/// 权限创建响应 DTO
#[derive(Debug, Serialize)]
pub struct CreatePermissionResponse {
    /// 权限ID
    pub id: i64,
    /// 权限名称
    pub name: String,
    /// 权限编码
    pub code: String,
    /// 权限类型
    pub permission_type: i32,
    /// 创建时间
    pub created_time: chrono::DateTime<chrono::Utc>,
}
