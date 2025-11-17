/// 日志级别响应 DTO

use serde::{Deserialize, Serialize};

/// 级别详情响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogLevelDetailResponse {
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
    /// 更新者
    pub update_by: Option<String>,
    /// 更新时间
    pub updated_time: chrono::DateTime<chrono::Utc>,
}

/// 系统内置级别统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemLevelStatistics {
    /// 系统内置数量
    pub system_count: usize,
    /// 用户自定义数量
    pub custom_count: usize,
    /// 总级别数量
    pub total_count: usize,
}

/// 状态级别统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusLevelStatistics {
    /// 启用状态数量
    pub enabled_count: usize,
    /// 禁用状态数量
    pub disabled_count: usize,
    /// 总级别数量
    pub total_count: usize,
}
