use tracing::{info, warn, error, debug};

/// 日志服务实现
/// 提供统一日志查询、统计、管理等功能

use crate::common::exception::{AppError, ErrorCode};
use crate::database::DatabaseManager;
use crate::app::opera_log::service::OperaLogService;
use crate::app::access_log::service::AccessLogService;
use crate::app::login_log::service::LoginLogService;
use sea_orm::{DatabaseConnection, DbErr};

/// 统一日志服务
pub struct LogService {
    db: DatabaseConnection,
}

impl LogService {
    /// 创建新的日志服务
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// 获取操作日志服务
    pub fn opera_log_service(&self) -> OperaLogService {
        OperaLogService::new(self.db.clone())
    }

    /// 获取访问日志服务
    pub fn access_log_service(&self) -> AccessLogService {
        AccessLogService::new(self.db.clone())
    }

    /// 获取登录日志服务
    pub fn login_log_service(&self) -> LoginLogService {
        LoginLogService::new(self.db.clone())
    }

    /// 清空所有日志
    pub async fn clear_all_logs(&self) -> Result<LogClearResult, AppError> {
        let mut result = LogClearResult::default();

        // 清空操作日志
        match self.opera_log_service().clear_opera_logs().await {
            Ok(count) => {
                result.opera_log_count = count;
                info!("Cleared {} opera logs", count);
            }
            Err(e) => {
                error!("Failed to clear opera logs: {:?}", e);
                result.opera_log_error = Some(e.to_string());
            }
        }

        // 清空访问日志
        match self.access_log_service().clear_access_logs().await {
            Ok(count) => {
                result.access_log_count = count;
                info!("Cleared {} access logs", count);
            }
            Err(e) => {
                error!("Failed to clear access logs: {:?}", e);
                result.access_log_error = Some(e.to_string());
            }
        }

        // 清空登录日志
        match self.login_log_service().clear_login_logs().await {
            Ok(count) => {
                result.login_log_count = count;
                info!("Cleared {} login logs", count);
            }
            Err(e) => {
                error!("Failed to clear login logs: {:?}", e);
                result.login_log_error = Some(e.to_string());
            }
        }

        Ok(result)
    }
}

/// 日志清空结果
#[derive(Debug, Default)]
pub struct LogClearResult {
    /// 操作日志清空数量
    pub opera_log_count: usize,
    /// 访问日志清空数量
    pub access_log_count: usize,
    /// 登录日志清空数量
    pub login_log_count: usize,
    /// 操作日志清空错误
    pub opera_log_error: Option<String>,
    /// 访问日志清空错误
    pub access_log_error: Option<String>,
    /// 登录日志清空错误
    pub login_log_error: Option<String>,
}
