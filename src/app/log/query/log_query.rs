/// 日志查询接口
/// 提供统一的日志查询接口

use serde::{Deserialize, Serialize};
use crate::app::opera_log::dto::{
    OperaLogPaginationQuery, OperaLogPaginationResponse,
};
use crate::app::access_log::dto::{
    AccessLogPaginationQuery, AccessLogPaginationResponse,
};
use crate::app::login_log::dto::{
    LoginLogPaginationQuery, LoginLogPaginationResponse,
};

/// 统一日志查询请求
#[derive(Debug, Deserialize)]
pub struct UnifiedLogQueryRequest {
    /// 日志类型
    pub log_type: LogType,
    /// 查询参数
    pub query: UnifiedLogQueryParams,
}

/// 日志类型
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogType {
    /// 操作日志
    OperaLog,
    /// 访问日志
    AccessLog,
    /// 登录日志
    LoginLog,
    /// 全部日志
    All,
}

/// 统一日志查询参数
#[derive(Debug, Deserialize)]
pub struct UnifiedLogQueryParams {
    /// 页码
    pub page: Option<usize>,
    /// 每页数量
    pub size: Option<usize>,
    /// 关键词
    pub keyword: Option<String>,
    /// 开始时间
    pub start_time: Option<chrono::DateTime<chrono::Utc>>,
    /// 结束时间
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
}

/// 统一日志查询响应
#[derive(Debug, Serialize)]
pub struct UnifiedLogQueryResponse {
    /// 日志类型
    pub log_type: LogType,
    /// 查询结果
    pub result: LogQueryResult,
}

/// 日志查询结果
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum LogQueryResult {
    /// 操作日志
    OperaLog {
        data: OperaLogPaginationResponse,
    },
    /// 访问日志
    AccessLog {
        data: AccessLogPaginationResponse,
    },
    /// 登录日志
    LoginLog {
        data: LoginLogPaginationResponse,
    },
    /// 全部日志
    All {
        opera_logs: OperaLogPaginationResponse,
        access_logs: AccessLogPaginationResponse,
        login_logs: LoginLogPaginationResponse,
    },
}

/// 批量日志查询请求
#[derive(Debug, Deserialize)]
pub struct BatchLogQueryRequest {
    /// 查询参数列表
    pub queries: Vec<UnifiedLogQueryRequest>,
}

/// 批量日志查询响应
#[derive(Debug, Serialize)]
pub struct BatchLogQueryResponse {
    /// 查询结果列表
    pub results: Vec<UnifiedLogQueryResponse>,
}

/// 日志导出请求
#[derive(Debug, Deserialize)]
pub struct LogExportRequest {
    /// 日志类型
    pub log_type: LogType,
    /// 查询参数
    pub query: UnifiedLogQueryParams,
    /// 导出格式
    pub format: ExportFormat,
}

/// 导出格式
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExportFormat {
    /// Excel
    Excel,
    /// CSV
    Csv,
    /// JSON
    Json,
}

/// 日志导出响应
#[derive(Debug, Serialize)]
pub struct LogExportResponse {
    /// 导出文件ID
    pub file_id: String,
    /// 导出文件URL
    pub file_url: String,
    /// 导出时间
    pub export_time: chrono::DateTime<chrono::Utc>,
    /// 导出数量
    pub export_count: usize,
}
