/// 日志级别查询 DTO

use serde::{Deserialize, Serialize};
use validator::Validate;

/// 分页查询参数
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct LogLevelQuery {
    /// 页码
    #[validate(range(min = 1, message = "页码必须大于0"))]
    pub page_num: Option<usize>,

    /// 每页数量
    #[validate(range(min = 1, max = 100, message = "每页数量必须在1-100之间"))]
    pub page_size: Option<usize>,

    /// 日志级别
    pub level_name: Option<String>,

    /// 级别值
    pub level_value: Option<i32>,

    /// 是否系统内置
    pub is_system: Option<i32>,

    /// 状态
    pub status: Option<i32>,

    /// 创建者
    pub create_by: Option<String>,
}

/// 启用级别查询
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnabledLogLevelQuery {
    /// 是否只查询启用状态
    pub enabled_only: bool,
}

/// 日志级别项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogLevelItem {
    /// 级别ID
    pub level_id: i64,
    /// 日志级别
    pub level_name: String,
    /// 级别值
    pub level_value: i32,
    /// 级别描述
    pub description: Option<String>,
}

/// 启用的日志级别列表
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnabledLogLevelList {
    /// 级别列表
    pub levels: Vec<LogLevelItem>,
}

/// 分页响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogLevelListResponse {
    /// 级别列表
    pub list: Vec<LogLevelListItem>,
    /// 总数
    pub total: usize,
    /// 页码
    pub page_num: usize,
    /// 每页数量
    pub page_size: usize,
}

/// 级别列表项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogLevelListItem {
    /// 级别ID
    pub level_id: i64,
    /// 日志级别
    pub level_name: String,
    /// 级别值
    pub level_value: i32,
    /// 级别描述
    pub description: Option<String>,
    /// 是否系统内置
    pub is_system: i32,
    /// 是否系统内置名称
    pub is_system_name: String,
    /// 状态
    pub status: i32,
    /// 状态名称
    pub status_name: String,
    /// 创建者
    pub create_by: String,
    /// 创建时间
    pub created_time: chrono::DateTime<chrono::Utc>,
    /// 更新时间
    pub updated_time: chrono::DateTime<chrono::Utc>,
}
