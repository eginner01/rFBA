/// 登录日志响应 DTO

use serde::{Deserialize, Serialize};

/// 登录日志详情响应
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginLogDetailResponse {
    /// 日志ID
    pub id: i64,
    /// 用户ID
    pub user_id: Option<i64>,
    /// 用户名
    pub username: String,
    /// 部门ID
    pub dept_id: Option<i64>,
    /// 部门名称
    pub dept_name: Option<String>,
    /// 登录IP
    pub ipaddr: String,
    /// 登录地点
    pub login_location: Option<String>,
    /// 浏览器
    pub browser: Option<String>,
    /// 操作系统
    pub os: Option<String>,
    /// 设备类型
    pub dev_type: Option<String>,
    /// 登录状态
    pub status: i32,
    /// 状态名称
    pub status_name: String,
    /// 提示消息
    pub msg: Option<String>,
    /// 访问时间
    pub access_time: chrono::DateTime<chrono::Utc>,
    /// 注销时间
    pub logout_time: Option<chrono::DateTime<chrono::Utc>>,
    /// 登录时间（毫秒）
    pub login_time: Option<i64>,
    /// 注销时间（毫秒）
    pub logout_time_ms: Option<i64>,
}

/// 登录日志列表项响应
/// 与前端 LoginLogResult 对齐：id、username、status、ip、country、region、os、browser、device、msg、login_time、created_time
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginLogListItem {
    /// 日志ID
    pub id: i64,
    /// 用户名
    pub username: String,
    /// 登录状态（0 失败，1 成功）
    pub status: i32,
    /// 登录IP
    pub ip: String,
    /// 国家
    pub country: Option<String>,
    /// 地区/省份
    pub region: Option<String>,
    /// 操作系统
    pub os: Option<String>,
    /// 浏览器
    pub browser: Option<String>,
    /// 设备
    pub device: Option<String>,
    /// 提示消息
    pub msg: String,
    /// 登录时间
    pub login_time: chrono::DateTime<chrono::Utc>,
    /// 创建时间
    pub created_time: chrono::DateTime<chrono::Utc>,
}

/// 登录日志统计
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginLogStatistics {
    /// 总登录次数
    pub total_count: usize,
    /// 成功登录次数
    pub success_count: usize,
    /// 失败登录次数
    pub failure_count: usize,
    /// 今日登录次数
    pub today_count: usize,
    /// 本周登录次数
    pub week_count: usize,
    /// 本月登录次数
    pub month_count: usize,
    /// 活跃用户数
    pub active_users: usize,
    /// 登录IP TOP10
    pub top_ips: Vec<LoginIpStat>,
    /// 失败登录原因
    pub failure_reasons: Vec<FailureReasonStat>,
}

/// 登录IP统计
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginIpStat {
    /// IP地址
    pub ip: String,
    /// 登录次数
    pub count: usize,
}

/// 失败原因统计
#[derive(Debug, Serialize, Deserialize)]
pub struct FailureReasonStat {
    /// 失败原因
    pub reason: String,
    /// 失败次数
    pub count: usize,
}

// 类型别名以保持API兼容性
pub type LoginLogResponse = LoginLogDetailResponse;
