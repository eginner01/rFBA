/// 系统状态响应 DTO

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemStatusResponse {
    /// 系统状态
    pub status: String,
    /// 系统健康度
    pub health: SystemHealth,
    /// 模块状态列表
    pub modules: Vec<ModuleStatus>,
    /// 数据库状态
    pub database: DatabaseStatus,
    /// 检查时间
    pub check_time: String,
    /// 系统版本
    pub version: String,
    /// 运行时长（秒）
    pub uptime: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemHealth {
    /// 整体健康度百分比
    pub health_percentage: u8,
    /// 正常模块数量
    pub healthy_modules: u32,
    /// 异常模块数量
    pub unhealthy_modules: u32,
    /// 数据库连接状态
    pub database_connection: bool,
    /// Redis连接状态
    pub redis_connection: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModuleStatus {
    /// 模块名称
    pub name: String,
    /// 模块状态
    pub status: String,
    /// 模块描述
    pub description: String,
    /// 模块版本
    pub version: Option<String>,
    /// 状态码
    pub code: i32,
    /// 错误信息
    pub error: Option<String>,
    /// 检查时间
    pub check_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseStatus {
    /// 数据库类型
    pub db_type: String,
    /// 数据库版本
    pub version: String,
    /// 连接状态
    pub connected: bool,
    /// 连接数量
    pub connections: Option<u32>,
    /// 表数量
    pub table_count: u32,
    /// 数据大小（MB）
    pub size_mb: Option<f64>,
    /// 状态码
    pub code: i32,
    /// 错误信息
    pub error: Option<String>,
}
