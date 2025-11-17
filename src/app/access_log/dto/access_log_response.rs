/// 访问日志响应 DTO

use serde::{Deserialize, Serialize};

/// 访问日志详情响应
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccessLogDetailResponse {
    /// 日志ID
    pub id: i64,
    /// 用户ID
    pub user_id: Option<i64>,
    /// 用户名
    pub user_name: Option<String>,
    /// 部门ID
    pub dept_id: Option<i64>,
    /// 部门名称
    pub dept_name: Option<String>,
    /// 请求ID
    pub trace_id: String,
    /// 父请求ID
    pub parent_trace_id: Option<String>,
    /// 请求方法
    pub method: String,
    /// 请求URL
    pub url: String,
    /// 请求参数
    pub query_params: Option<String>,
    /// 请求体
    pub request_body: Option<String>,
    /// 响应状态码
    pub status_code: u16,
    /// 响应体
    pub response_body: Option<String>,
    /// 客户端IP
    pub client_ip: String,
    /// 用户代理
    pub user_agent: Option<String>,
    /// 操作系统
    pub os: Option<String>,
    /// 浏览器
    pub browser: Option<String>,
    /// 设备类型
    pub device_type: Option<String>,
    /// 访问来源
    pub referer: Option<String>,
    /// 请求时间（毫秒）
    pub cost_time: i64,
    /// 是否异常
    pub is_error: bool,
    /// 错误信息
    pub error_msg: Option<String>,
    /// 访问时间
    pub access_time: chrono::DateTime<chrono::Utc>,
}

/// 访问日志列表项响应
#[derive(Debug, Serialize, Deserialize)]
pub struct AccessLogListItem {
    /// 日志ID
    pub id: i64,
    /// 用户名
    pub user_name: Option<String>,
    /// 请求方法
    pub method: String,
    /// 请求URL
    pub url: String,
    /// 客户端IP
    pub client_ip: String,
    /// 响应状态码
    pub status_code: u16,
    /// 请求时间（毫秒）
    pub cost_time: i64,
    /// 是否异常
    pub is_error: bool,
    /// 访问时间
    pub access_time: chrono::DateTime<chrono::Utc>,
}

/// 访问日志统计
#[derive(Debug, Serialize, Deserialize)]
pub struct AccessLogStatistics {
    /// 总访问次数
    pub total_count: usize,
    /// 成功访问次数
    pub success_count: usize,
    /// 失败访问次数
    pub failure_count: usize,
    /// 平均响应时间（毫秒）
    pub avg_response_time: Option<i64>,
    /// 最大响应时间（毫秒）
    pub max_response_time: Option<i64>,
    /// 今日访问次数
    pub today_count: usize,
    /// 本周访问次数
    pub week_count: usize,
    /// 本月访问次数
    pub month_count: usize,
    /// 热门URL TOP10
    pub top_urls: Vec<UrlStat>,
    /// 客户端IP TOP10
    pub top_ips: Vec<IpStat>,
    /// 方法统计
    pub method_stats: Vec<MethodStat>,
}

/// URL统计
#[derive(Debug, Serialize, Deserialize)]
pub struct UrlStat {
    /// URL
    pub url: String,
    /// 访问次数
    pub count: usize,
    /// 平均响应时间
    pub avg_response_time: Option<i64>,
}

/// IP统计
#[derive(Debug, Serialize, Deserialize)]
pub struct IpStat {
    /// IP地址
    pub ip: String,
    /// 访问次数
    pub count: usize,
}

/// 方法统计
#[derive(Debug, Serialize, Deserialize)]
pub struct MethodStat {
    /// HTTP方法
    pub method: String,
    /// 访问次数
    pub count: usize,
    /// 占比
    pub percentage: f64,
}
