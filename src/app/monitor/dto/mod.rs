use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// 系统状态信息
#[derive(Debug, Serialize, Deserialize)]
pub struct SystemStatus {
    pub status: String,
    pub uptime_seconds: i64,
    pub timestamp: DateTime<Utc>,
}

/// 服务器性能指标
#[derive(Debug, Serialize, Deserialize)]
pub struct ServerMetrics {
    pub cpu: CpuInfo,
    pub mem: MemoryInfo,
    pub sys: SystemInfo,
    pub disk: Vec<DiskInfo>,
    pub service: ServiceInfo,
}

/// CPU 信息
#[derive(Debug, Serialize, Deserialize)]
pub struct CpuInfo {
    pub usage: f64,
    pub logical_num: usize,
    pub physical_num: usize,
    pub max_freq: f64,
    pub min_freq: f64,
    pub current_freq: f64,
}

/// 内存信息
#[derive(Debug, Serialize, Deserialize)]
pub struct MemoryInfo {
    pub total: f64,
    pub used: f64,
    pub free: f64,
    pub usage: f64,
}

/// 系统信息
#[derive(Debug, Serialize, Deserialize)]
pub struct SystemInfo {
    pub name: String,
    pub ip: String,
    pub os: String,
    pub arch: String,
}

/// 磁盘信息
#[derive(Debug, Serialize, Deserialize)]
pub struct DiskInfo {
    pub dir: String,
    #[serde(rename = "type")]
    pub disk_type: String,
    pub device: String,
    pub total: String,
    pub free: String,
    pub used: String,
    pub usage: String,
}

/// 服务信息
#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceInfo {
    pub name: String,
    pub version: String,
    pub home: String,
    pub cpu_usage: String,
    pub mem_vms: String,
    pub mem_rss: String,
    pub mem_free: String,
    pub startup: String,
    pub elapsed: String,
}

/// 系统性能指标
#[derive(Debug, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub disk_usage: f64,
    pub network_io: NetworkIO,
    pub timestamp: DateTime<Utc>,
}

/// 网络IO统计
#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkIO {
    pub bytes_sent: u64,
    pub bytes_recv: u64,
    pub packets_sent: u64,
    pub packets_recv: u64,
}

/// 数据库连接池状态
#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseStatus {
    pub active_connections: u32,
    pub idle_connections: u32,
    pub total_connections: u32,
    pub max_connections: u32,
}

/// API请求统计
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiMetrics {
    pub total_requests: u64,
    pub success_requests: u64,
    pub error_requests: u64,
    pub avg_response_time: f64,
    pub requests_per_minute: u64,
    pub timestamp: DateTime<Utc>,
}

/// 监控查询参数
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct MonitorQuery {
    pub metric_type: Option<String>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
}

/// 监控数据点
#[derive(Debug, Serialize, Deserialize)]
pub struct MetricDataPoint {
    pub timestamp: DateTime<Utc>,
    pub value: f64,
    pub tags: Option<serde_json::Value>,
}

/// 监控数据响应
#[derive(Debug, Serialize, Deserialize)]
pub struct MonitorResponse {
    pub metric_type: String,
    pub data_points: Vec<MetricDataPoint>,
}

/// 健康检查响应
#[derive(Debug, Serialize, Deserialize)]
pub struct HealthStatus {
    pub status: String,
    pub version: String,
    pub database: String,
    pub redis: String,
    pub timestamp: DateTime<Utc>,
}

/// 健康检查响应
#[derive(Debug, Serialize, Deserialize)]
pub struct HealthCheckResponse {
    pub status: String,
    pub timestamp: DateTime<Utc>,
    pub services: Vec<ServiceHealth>,
}

/// 服务健康状态
#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceHealth {
    pub service_name: String,
    pub status: String,
    pub message: Option<String>,
    pub response_time: Option<f64>,
}

/// Redis监控信息
#[derive(Debug, Serialize, Deserialize)]
pub struct RedisMetrics {
    /// Redis 服务器信息（格式化为字符串的字典）
    pub info: std::collections::HashMap<String, String>,
    /// Redis 命令统计
    pub stats: Vec<RedisCommandStat>,
}

/// Redis 命令统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisCommandStat {
    pub name: String,
    pub value: String,
}

/// Redis服务器信息（内部解析使用）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisServerInfo {
    pub version: String,
    pub connected_clients: u64,
    pub used_memory: u64,
    pub used_memory_peak: u64,
    pub uptime_in_seconds: u64,
    pub hit_rate: f64,
}

/// 在线会话信息（令牌详情）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OnlineSession {
    /// 用户 ID
    pub id: i64,
    /// 会话 UUID
    pub session_uuid: String,
    /// 用户名
    pub username: String,
    /// 昵称
    pub nickname: String,
    /// IP 地址
    pub ip: String,
    /// 操作系统
    pub os: String,
    /// 浏览器
    pub browser: String,
    /// 设备
    pub device: String,
    /// 状态（0-离线 1-在线）
    pub status: i32,
    /// 最后登录时间
    pub last_login_time: String,
    /// 过期时间
    pub expire_time: DateTime<Utc>,
}

/// Token 额外信息
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenExtraInfo {
    pub username: Option<String>,
    pub nickname: Option<String>,
    pub ip: Option<String>,
    pub os: Option<String>,
    pub browser: Option<String>,
    pub device: Option<String>,
    pub last_login_time: Option<String>,
    pub swagger: Option<bool>,
}

/// 任务信息
#[derive(Debug, Serialize, Deserialize)]
pub struct TaskInfo {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub schedule: String,
    pub status: String,
    pub next_run: Option<DateTime<Utc>>,
}
