/// 日志统计接口
/// 提供统一的日志统计接口

use serde::{Deserialize, Serialize};
use crate::app::opera_log::dto::OperaLogStatistics;
use crate::app::access_log::dto::AccessLogStatistics;
use crate::app::login_log::dto::LoginLogStatistics;

/// 统一日志统计请求
#[derive(Debug, Deserialize)]
pub struct UnifiedLogStatisticsRequest {
    /// 统计类型
    pub statistics_type: LogStatisticsType,
    /// 统计参数
    pub params: LogStatisticsParams,
}

/// 统计类型
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogStatisticsType {
    /// 操作日志统计
    OperaLog,
    /// 访问日志统计
    AccessLog,
    /// 登录日志统计
    LoginLog,
    /// 全部日志统计
    All,
}

/// 统计参数
#[derive(Debug, Deserialize)]
pub struct LogStatisticsParams {
    /// 开始时间
    pub start_time: Option<chrono::DateTime<chrono::Utc>>,
    /// 结束时间
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    /// 用户ID
    pub user_id: Option<i64>,
    /// 部门ID
    pub dept_id: Option<i64>,
}

/// 统一日志统计响应
#[derive(Debug, Serialize)]
pub struct UnifiedLogStatisticsResponse {
    /// 统计类型
    pub statistics_type: LogStatisticsType,
    /// 统计结果
    pub result: LogStatisticsResult,
}

/// 日志统计结果
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum LogStatisticsResult {
    /// 操作日志统计
    OperaLog {
        data: OperaLogStatistics,
    },
    /// 访问日志统计
    AccessLog {
        data: AccessLogStatistics,
    },
    /// 登录日志统计
    LoginLog {
        data: LoginLogStatistics,
    },
    /// 全部日志统计
    All {
        opera_logs: OperaLogStatistics,
        access_logs: AccessLogStatistics,
        login_logs: LoginLogStatistics,
    },
}

/// 日志统计趋势
#[derive(Debug, Serialize, Deserialize)]
pub struct LogStatisticsTrend {
    /// 时间点
    pub time_point: chrono::DateTime<chrono::Utc>,
    /// 操作日志数量
    pub opera_log_count: usize,
    /// 访问日志数量
    pub access_log_count: usize,
    /// 登录日志数量
    pub login_log_count: usize,
}

/// 统一日志趋势统计请求
#[derive(Debug, Deserialize)]
pub struct UnifiedLogTrendRequest {
    /// 统计类型
    pub trend_type: LogTrendType,
    /// 统计时间范围
    pub time_range: LogTrendTimeRange,
}

/// 趋势类型
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogTrendType {
    /// 按天统计
    Daily,
    /// 按周统计
    Weekly,
    /// 按月统计
    Monthly,
}

/// 趋势时间范围
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogTrendTimeRange {
    /// 最近7天
    Last7Days,
    /// 最近30天
    Last30Days,
    /// 最近90天
    Last90Days,
    /// 自定义时间范围
    Custom {
        start_time: chrono::DateTime<chrono::Utc>,
        end_time: chrono::DateTime<chrono::Utc>,
    },
}

/// 统一日志趋势统计响应
#[derive(Debug, Serialize)]
pub struct UnifiedLogTrendResponse {
    /// 趋势类型
    pub trend_type: LogTrendType,
    /// 时间范围
    pub time_range: LogTrendTimeRange,
    /// 统计趋势
    pub trends: Vec<LogStatisticsTrend>,
}
