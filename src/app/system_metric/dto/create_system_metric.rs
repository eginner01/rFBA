/// 系统监控指标创建和更新 DTO

use serde::{Deserialize, Serialize};
use validator::Validate;

/// 创建系统指标请求
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateSystemMetricRequest {
    /// 指标类型（1:CPU 2:内存 3:磁盘 4:网络）
    #[validate(range(min = 1, max = 4, message = "指标类型必须是1-4之间的值"))]
    pub metric_type: i32,

    /// 指标名称
    #[validate(length(min = 1, max = 100, message = "指标名称长度必须在1-100个字符之间"))]
    pub metric_name: String,

    /// 指标值
    pub metric_value: f64,

    /// 指标单位
    #[validate(length(min = 1, max = 20, message = "指标单位长度必须在1-20个字符之间"))]
    pub unit: String,

    /// 主机名
    #[validate(length(min = 1, max = 100, message = "主机名长度必须在1-100个字符之间"))]
    pub host_name: String,

    /// IP地址
    #[validate(length(min = 1, max = 50, message = "IP地址长度必须在1-50个字符之间"))]
    pub ip_address: String,

    /// 备注
    pub remark: Option<String>,
}

/// 批量创建系统指标请求
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct BatchCreateSystemMetricRequest {
    /// 指标列表
    #[validate(length(min = 1, max = 100, message = "指标列表数量必须在1-100之间"))]
    pub metrics: Vec<CreateSystemMetricRequest>,
}

/// 批量创建系统指标响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchCreateSystemMetricResponse {
    /// 成功创建的数量
    pub success_count: usize,
    /// 失败的指标名称
    pub failed_metrics: Vec<String>,
}
