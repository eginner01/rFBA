/// 创建操作日志 DTO

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateOperaLogRequest {
    /// 操作模块
    pub title: String,
    /// 业务类型
    pub business_type: i32,
    /// 请求方法
    pub method: String,
    /// 请求方式
    pub request_method: String,
    /// 操作类别
    pub operator_type: i32,
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
}

/// 操作日志创建响应
#[derive(Debug, Serialize)]
pub struct CreateOperaLogResponse {
    /// 日志ID
    pub id: i64,
    /// 操作模块
    pub title: String,
    /// 操作人
    pub user_name: String,
    /// 创建时间
    pub created_time: chrono::DateTime<chrono::Utc>,
}
