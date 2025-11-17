/// 健康检查响应 DTO

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HealthCheckResponse {
    /// 健康状态
    pub status: String,
    /// 整体状态
    pub overall_status: String,
    /// 检查时间
    pub check_time: String,
    /// 响应时间（毫秒）
    pub response_time_ms: u64,
    /// 详细健康信息
    pub details: HealthStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HealthStatus {
    /// 数据库健康状态
    pub database: bool,
    /// Redis健康状态
    pub redis: bool,
    /// 系统负载状态
    pub system_load: bool,
    /// 内存使用状态
    pub memory_usage: bool,
    /// 磁盘空间状态
    pub disk_space: bool,
    /// 网络连接状态
    pub network: bool,
    /// 额外信息
    pub extra_info: Option<String>,
}
