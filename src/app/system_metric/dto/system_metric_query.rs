/// 系统监控指标查询 DTO

use serde::{Deserialize, Serialize};
use validator::Validate;

/// 分页查询参数
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct SystemMetricQuery {
    /// 页码
    #[validate(range(min = 1, message = "页码必须大于0"))]
    pub page_num: Option<usize>,

    /// 每页数量
    #[validate(range(min = 1, max = 100, message = "每页数量必须在1-100之间"))]
    pub page_size: Option<usize>,

    /// 指标类型
    pub metric_type: Option<i32>,

    /// 指标名称
    pub metric_name: Option<String>,

    /// 主机名
    pub host_name: Option<String>,

    /// IP地址
    pub ip_address: Option<String>,

    /// 开始时间
    pub start_time: Option<chrono::DateTime<chrono::Utc>>,

    /// 结束时间
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
}

/// 实时指标查询
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealTimeMetricQuery {
    /// 主机名
    pub host_name: Option<String>,
    /// 指标类型列表
    pub metric_types: Option<Vec<i32>>,
}

/// 单个指标历史查询
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct MetricHistoryQuery {
    /// 主机名
    #[validate(length(min = 1, message = "主机名不能为空"))]
    pub host_name: String,

    /// 指标名称
    #[validate(length(min = 1, message = "指标名称不能为空"))]
    pub metric_name: String,

    /// 开始时间
    pub start_time: Option<chrono::DateTime<chrono::Utc>>,

    /// 结束时间
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,

    /// 时间间隔（分钟）
    pub interval_minutes: Option<u32>,
}

/// 指标统计查询
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct MetricStatisticsQuery {
    /// 主机名
    pub host_name: Option<String>,

    /// 指标类型
    #[validate(range(min = 1, max = 4, message = "指标类型必须是1-4之间的值"))]
    pub metric_type: i32,

    /// 时间范围（小时）
    #[validate(range(min = 1, max = 168, message = "时间范围必须在1-168小时之间"))]
    pub time_range_hours: i32,
}

/// 分页响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetricListResponse {
    /// 指标列表
    pub list: Vec<SystemMetricListItem>,
    /// 总数
    pub total: usize,
    /// 页码
    pub page_num: usize,
    /// 每页数量
    pub page_size: usize,
}

/// 指标列表项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetricListItem {
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
