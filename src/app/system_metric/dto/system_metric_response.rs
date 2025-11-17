/// 系统监控指标响应 DTO

use serde::{Deserialize, Serialize};

/// 指标详情响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetricDetailResponse {
    /// 指标ID
    pub metric_id: i64,
    /// 指标类型
    pub metric_type: i32,
    /// 指标类型名称
    pub metric_type_name: String,
    /// 指标名称
    pub metric_name: String,
    /// 指标值
    pub metric_value: f64,
    /// 指标单位
    pub unit: String,
    /// 指标值描述
    pub value_description: String,
    /// 主机名
    pub host_name: String,
    /// IP地址
    pub ip_address: String,
    /// 采集时间
    pub collection_time: chrono::DateTime<chrono::Utc>,
    /// 备注
    pub remark: Option<String>,
}

/// 实时指标响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealTimeMetricResponse {
    /// 主机名
    pub host_name: String,
    /// IP地址
    pub ip_address: String,
    /// 指标列表
    pub metrics: Vec<SystemMetricListItem>,
}

/// 指标历史数据点
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricHistoryPoint {
    /// 时间
    pub time: chrono::DateTime<chrono::Utc>,
    /// 指标值
    pub value: f64,
}

/// 指标历史响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricHistoryResponse {
    /// 主机名
    pub host_name: String,
    /// 指标名称
    pub metric_name: String,
    /// 指标类型
    pub metric_type: i32,
    /// 指标单位
    pub unit: String,
    /// 历史数据点
    pub history: Vec<MetricHistoryPoint>,
}

/// 指标统计数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricStatisticsData {
    /// 最小值
    pub min_value: f64,
    /// 最大值
    pub max_value: f64,
    /// 平均值
    pub avg_value: f64,
    /// 当前值
    pub current_value: f64,
    /// 数据点数量
    pub data_count: usize,
}

/// 指标统计响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricStatisticsResponse {
    /// 主机名
    pub host_name: String,
    /// 指标类型
    pub metric_type: i32,
    /// 指标类型名称
    pub metric_type_name: String,
    /// 时间范围
    pub time_range_hours: i32,
    /// 统计数据
    pub statistics: MetricStatisticsData,
}

/// 指标类型统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricTypeStatistics {
    /// 指标类型
    pub metric_type: i32,
    /// 指标类型名称
    pub metric_type_name: String,
    /// 指标数量
    pub count: usize,
    /// 主机数量
    pub host_count: usize,
}

/// 主机概览
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostOverview {
    /// 主机名
    pub host_name: String,
    /// IP地址
    pub ip_address: String,
    /// 最后采集时间
    pub last_collection_time: chrono::DateTime<chrono::Utc>,
    /// CPU使用率
    pub cpu_usage: Option<f64>,
    /// 内存使用率
    pub memory_usage: Option<f64>,
    /// 磁盘使用率
    pub disk_usage: Option<f64>,
}
