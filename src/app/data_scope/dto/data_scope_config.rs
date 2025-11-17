/// 数据权限配置 DTO

use serde::{Deserialize, Serialize};
use validator::Validate;

/// 创建数据范围请求（匹配 Python 的 CreateDataScopeParam）
#[derive(Debug, Deserialize, Validate)]
pub struct CreateDataScopeRequest {
    /// 名称
    #[validate(length(min = 1, max = 64, message = "名称长度必须在1-64个字符之间"))]
    pub name: String,
    /// 状态（0停用 1正常）
    pub status: i32,
}

/// 更新数据范围请求（匹配 Python 的 UpdateDataScopeParam）
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateDataScopeRequest {
    /// 名称
    #[validate(length(min = 1, max = 64, message = "名称长度必须在1-64个字符之间"))]
    pub name: String,
    /// 状态（0停用 1正常）
    pub status: i32,
}

/// 更新数据范围规则请求（匹配 Python 的 UpdateDataScopeRuleParam）
#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateDataScopeRuleRequest {
    /// 数据规则 ID 列表
    pub rules: Vec<i64>,
}

/// 批量删除数据范围请求（匹配 Python 的 DeleteDataScopeParam）
#[derive(Debug, Deserialize, Serialize)]
pub struct DeleteDataScopeRequest {
    /// 数据范围 ID 列表
    pub pks: Vec<i64>,
}

// ===== 旧的 DTO（保留用于角色数据权限配置，后续实现） =====

/// 数据权限配置请求（角色维度）
#[derive(Debug, Deserialize, Validate)]
pub struct DataScopeConfigRequest {
    /// 角色ID
    pub role_id: i64,
    /// 数据权限类型（1: 全部数据, 2: 自定义数据, 3: 本部门数据, 4: 本部门及以下数据, 5: 仅本人数据）
    pub data_scope: i32,
    /// 自定义数据范围（部门ID列表，用逗号分隔）
    pub custom_data: Option<String>,
}

/// 数据权限配置响应
#[derive(Debug, Serialize)]
pub struct DataScopeConfigResponse {
    /// 配置ID
    pub id: i64,
    /// 角色ID
    pub role_id: i64,
    /// 角色名称
    pub role_name: String,
    /// 数据权限类型
    pub data_scope: i32,
    /// 数据权限类型名称
    pub data_scope_name: String,
    /// 自定义数据范围
    pub custom_data: Option<String>,
    /// 创建时间
    pub created_time: chrono::DateTime<chrono::Utc>,
    /// 更新时间
    pub updated_time: chrono::DateTime<chrono::Utc>,
}

/// 批量配置数据权限请求
#[derive(Debug, Deserialize, Validate)]
pub struct BatchDataScopeConfigRequest {
    /// 角色ID列表
    pub role_ids: Vec<i64>,
    /// 数据权限类型
    pub data_scope: i32,
    /// 自定义数据范围
    pub custom_data: Option<String>,
}

/// 批量配置数据权限响应
#[derive(Debug, Serialize)]
pub struct BatchDataScopeConfigResponse {
    /// 成功配置的角色ID列表
    pub success_role_ids: Vec<i64>,
    /// 失败配置的角色ID列表
    pub failed_role_ids: Vec<i64>,
    /// 配置时间
    pub configured_time: chrono::DateTime<chrono::Utc>,
}
