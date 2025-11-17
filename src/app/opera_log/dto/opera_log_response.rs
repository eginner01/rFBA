/// 操作日志响应 DTO

use serde::{Deserialize, Serialize};

/// 操作日志详情响应
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OperaLogDetailResponse {
    /// 日志ID
    pub id: i64,
    /// 操作模块
    pub title: String,
    /// 业务类型
    pub business_type: i32,
    /// 业务类型名称
    pub business_type_name: String,
    /// 请求方法
    pub method: String,
    /// 请求方式
    pub request_method: String,
    /// 操作类别
    pub operator_type: i32,
    /// 操作员类型名称
    pub operator_type_name: String,
    /// 操作人ID
    pub user_id: Option<i64>,
    /// 操作人名称
    pub user_name: String,
    /// 操作人部门ID
    pub dept_id: Option<i64>,
    /// 操作人部门名称
    pub dept_name: Option<String>,
    /// 请求URL
    pub oper_url: String,
    /// 操作地址
    pub oper_ip: String,
    /// 操作地点
    pub oper_location: Option<String>,
    /// 请求参数
    pub oper_param: Option<String>,
    /// 返回参数
    pub json_result: Option<String>,
    /// 操作状态
    pub status: i32,
    /// 错误消息
    pub error_msg: Option<String>,
    /// 操作时间（毫秒）
    pub cost_time: Option<i64>,
    /// 创建时间
    pub created_time: chrono::DateTime<chrono::Utc>,
}

/// 操作日志列表项响应
/// 对齐前端 OperaLogResult 接口
#[derive(Debug, Serialize, Deserialize)]
pub struct OperaLogListItem {
    /// 日志ID
    pub id: i64,
    /// 追踪 ID
    pub trace_id: String,
    /// 用户名
    pub username: Option<String>,
    /// 请求方法
    pub method: String,
    /// 操作标题
    pub title: String,
    /// 请求路径
    pub path: String,
    /// IP 地址
    pub ip: String,
    /// 国家
    pub country: Option<String>,
    /// 地区
    pub region: Option<String>,
    /// 城市
    pub city: Option<String>,
    /// 用户代理
    pub user_agent: String,
    /// 操作系统
    pub os: Option<String>,
    /// 浏览器
    pub browser: Option<String>,
    /// 设备
    pub device: Option<String>,
    /// 状态（0: 异常, 1: 正常）
    pub status: i32,
    /// 状态码
    pub code: String,
    /// 消息
    pub msg: Option<String>,
    /// 耗时（毫秒）
    pub cost_time: f32,
    /// 操作时间
    pub opera_time: String,
}

/// 操作日志统计
#[derive(Debug, Serialize, Deserialize)]
pub struct OperaLogStatistics {
    /// 总操作次数
    pub total_count: usize,
    /// 成功操作次数
    pub success_count: usize,
    /// 失败操作次数
    pub failure_count: usize,
    /// 平均响应时间（毫秒）
    pub avg_response_time: Option<i64>,
    /// 今日操作次数
    pub today_count: usize,
    /// 本周操作次数
    pub week_count: usize,
    /// 本月操作次数
    pub month_count: usize,
    /// 业务类型统计
    pub business_type_stats: Vec<BusinessTypeStat>,
}

/// 业务类型统计
#[derive(Debug, Serialize, Deserialize)]
pub struct BusinessTypeStat {
    /// 业务类型
    pub business_type: i32,
    /// 业务类型名称
    pub business_type_name: String,
    /// 操作次数
    pub count: usize,
    /// 占比
    pub percentage: f64,
}

// 类型别名以保持API兼容性
pub type OperaLogResponse = OperaLogDetailResponse;
