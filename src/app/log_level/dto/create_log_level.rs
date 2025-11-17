/// 日志级别创建和更新 DTO

use serde::{Deserialize, Serialize};
use validator::Validate;

/// 创建日志级别请求
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateLogLevelRequest {
    /// 日志级别
    #[validate(length(min = 1, max = 50, message = "日志级别长度必须在1-50个字符之间"))]
    pub level_name: String,

    /// 级别值
    #[validate(range(min = 0, message = "级别值必须大于等于0"))]
    pub level_value: i32,

    /// 级别描述
    pub description: Option<String>,

    /// 是否系统内置（0:否 1:是）
    pub is_system: Option<i32>,

    /// 状态（0:启用 1:禁用）
    #[validate(range(min = 0, max = 1, message = "状态必须是0或1"))]
    pub status: i32,
}

/// 更新日志级别请求
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateLogLevelRequest {
    /// 级别ID
    #[validate(range(min = 1, message = "级别ID必须大于0"))]
    pub level_id: i64,

    /// 日志级别
    #[validate(length(min = 1, max = 50, message = "日志级别长度必须在1-50个字符之间"))]
    pub level_name: String,

    /// 级别值
    #[validate(range(min = 0, message = "级别值必须大于等于0"))]
    pub level_value: i32,

    /// 级别描述
    pub description: Option<String>,

    /// 状态（0:启用 1:禁用）
    #[validate(range(min = 0, max = 1, message = "状态必须是0或1"))]
    pub status: i32,
}

/// 删除日志级别请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteLogLevelRequest {
    /// 级别ID列表
    pub level_ids: Vec<i64>,
}

/// 创建日志级别响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLogLevelResponse {
    /// 级别ID
    pub level_id: i64,
    /// 日志级别
    pub level_name: String,
    /// 级别值
    pub level_value: i32,
    /// 创建时间
    pub created_time: chrono::DateTime<chrono::Utc>,
}

/// 更新日志级别响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateLogLevelResponse {
    /// 级别ID
    pub level_id: i64,
    /// 日志级别
    pub level_name: String,
    /// 更新时间
    pub updated_time: chrono::DateTime<chrono::Utc>,
}
