/// 创建角色请求 DTO

use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateRoleRequest {
    /// 角色名称
    #[validate(length(min = 1, max = 50))]
    pub name: String,

    /// 角色编码
    #[validate(length(min = 1, max = 50))]
    pub code: String,

    /// 排序
    pub sort: Option<i32>,

    /// 是否启用数据权限过滤
    pub is_filter_scopes: Option<bool>,

    /// 状态（0: 禁用, 1: 启用）
    pub status: Option<i32>,

    /// 角色描述
    pub remark: Option<String>,
}

/// 角色创建响应 DTO
#[derive(Debug, Serialize)]
pub struct CreateRoleResponse {
    /// 角色ID
    pub id: i64,
    /// 角色名称
    pub name: String,
    /// 状态
    pub status: i32,
    /// 创建时间
    pub created_time: chrono::DateTime<chrono::Utc>,
}
