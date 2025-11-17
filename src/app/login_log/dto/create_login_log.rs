/// 创建登录日志 DTO

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateLoginLogRequest {
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
    /// 提示消息
    pub msg: Option<String>,
    /// 登录时间（毫秒）
    pub login_time: Option<i64>,
}

/// 登录日志创建响应
#[derive(Debug, Serialize)]
pub struct CreateLoginLogResponse {
    /// 日志ID
    pub id: i64,
    /// 用户名
    pub username: String,
    /// 登录状态
    pub status: i32,
    /// 访问时间
    pub access_time: chrono::DateTime<chrono::Utc>,
}

/// 创建注销日志请求
#[derive(Debug, Deserialize)]
pub struct CreateLogoutLogRequest {
    /// 用户ID
    pub user_id: Option<i64>,
    /// 用户名
    pub username: String,
    /// 登录ID
    pub login_id: Option<i64>,
    /// 注销时间（毫秒）
    pub logout_time: Option<i64>,
}

/// 注销日志创建响应
#[derive(Debug, Serialize)]
pub struct CreateLogoutLogResponse {
    /// 日志ID
    pub id: i64,
    /// 用户名
    pub username: String,
    /// 注销时间
    pub logout_time: chrono::DateTime<chrono::Utc>,
}
