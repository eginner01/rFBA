/// 系统配置创建和更新 DTO

use serde::{Deserialize, Serialize};
use validator::Validate;

/// 创建系统配置请求
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateSysConfigRequest {
    /// 配置名称
    #[validate(length(min = 1, max = 100, message = "配置名称长度必须在1-100个字符之间"))]
    pub config_name: String,

    /// 配置键名
    #[validate(length(min = 1, max = 100, message = "配置键名长度必须在1-100个字符之间"))]
    pub config_key: String,

    /// 配置值
    pub config_value: String,

    /// 配置类型（1:字符串 2:数字 3:布尔 4:JSON）
    #[validate(range(min = 1, max = 4, message = "配置类型必须是1-4之间的值"))]
    pub config_type: i32,

    /// 备注
    pub remark: Option<String>,
}

/// 更新系统配置请求
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateSysConfigRequest {
    /// 配置ID
    #[validate(range(min = 1, message = "配置ID必须大于0"))]
    pub id: i64,

    /// 配置名称
    #[validate(length(min = 1, max = 100, message = "配置名称长度必须在1-100个字符之间"))]
    pub config_name: String,

    /// 配置键名
    #[validate(length(min = 1, max = 100, message = "配置键名长度必须在1-100个字符之间"))]
    pub config_key: String,

    /// 配置值
    pub config_value: String,

    /// 配置类型（1:字符串 2:数字 3:布尔 4:JSON）
    #[validate(range(min = 1, max = 4, message = "配置类型必须是1-4之间的值"))]
    pub config_type: i32,

    /// 备注
    pub remark: Option<String>,
}

/// 删除系统配置请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteSysConfigRequest {
    /// 配置ID列表
    pub ids: Vec<i64>,
}

/// 创建系统配置响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSysConfigResponse {
    /// 配置ID
    pub id: i64,
    /// 配置名称
    pub config_name: String,
    /// 配置键名
    pub config_key: String,
    /// 创建时间
    pub created_time: chrono::DateTime<chrono::Utc>,
}

/// 更新系统配置响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSysConfigResponse {
    /// 配置ID
    pub id: i64,
    /// 配置名称
    pub config_name: String,
    /// 更新时间
    pub updated_time: chrono::DateTime<chrono::Utc>,
}
