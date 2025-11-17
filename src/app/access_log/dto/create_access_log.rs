/// 创建访问日志 DTO

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateAccessLogRequest {
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
}

/// 访问日志创建响应
#[derive(Debug, Serialize)]
pub struct CreateAccessLogResponse {
    /// 日志ID
    pub id: i64,
    /// 请求ID
    pub trace_id: String,
    /// 访问时间
    pub access_time: chrono::DateTime<chrono::Utc>,
}
