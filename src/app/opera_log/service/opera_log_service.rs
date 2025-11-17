use tracing::error;

/// 操作日志服务实现
/// 提供操作日志记录、查询、统计等功能

use crate::app::opera_log::dto::{
    CreateOperaLogRequest, CreateOperaLogResponse, OperaLogPaginationQuery,
    OperaLogPaginationResponse, OperaLogListItem, OperaLogDetailResponse,
    OperaLogStatistics,
};
use crate::common::exception::{AppError, ErrorCode};
use crate::database::entity::opera_log;
use sea_orm::{
    DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, 
    Order, Condition, QueryOrder, QuerySelect, PaginatorTrait
};

/// 操作日志服务
pub struct OperaLogService {
    db: DatabaseConnection,
}

impl OperaLogService {
    /// 创建新的操作日志服务
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// 创建操作日志
    /// TODO: 需要根据新的数据库结构重新实现
    pub async fn create_opera_log(
        &self,
        _request: &CreateOperaLogRequest,
    ) -> Result<CreateOperaLogResponse, AppError> {
        // 暂时返回错误，待实现
        Err(AppError::with_message(ErrorCode::InternalServerError, "创建操作日志功能暂未实现"))
    }

    /// 分页查询操作日志
    pub async fn get_opera_logs_paginated(
        &self,
        query: &OperaLogPaginationQuery,
    ) -> Result<OperaLogPaginationResponse, AppError> {
        let page = query.page.unwrap_or(1);
        let size = query.size.unwrap_or(10);
        let offset = (page - 1) * size;

        let mut select = opera_log::Entity::find();

        // 关键词搜索（搜索用户名和标题）
        if let Some(ref keyword) = query.keyword {
            select = select.filter(
                Condition::any()
                    .add(opera_log::Column::Username.like(format!("%{}%", keyword)))
                    .add(opera_log::Column::Title.like(format!("%{}%", keyword))),
            );
        }

        // 按用户名筛选
        if let Some(ref user_name) = query.user_name {
            select = select.filter(opera_log::Column::Username.like(format!("%{}%", user_name)));
        }

        // 按标题筛选
        if let Some(ref title) = query.title {
            select = select.filter(opera_log::Column::Title.like(format!("%{}%", title)));
        }

        // 按状态筛选
        if let Some(status) = query.status {
            select = select.filter(opera_log::Column::Status.eq(status));
        }

        // 按时间范围筛选
        if let Some(start_time) = query.start_time {
            select = select.filter(opera_log::Column::CreatedTime.gte(start_time));
        }

        if let Some(end_time) = query.end_time {
            select = select.filter(opera_log::Column::CreatedTime.lte(end_time));
        }

        // 排序（默认按创建时间降序）
        select = select.order_by(opera_log::Column::CreatedTime, Order::Desc);

        // 查询总数
        let total = select.clone().count(&self.db).await.map_err(|e| {
            error!("Failed to count opera logs: {:?}", e);
            AppError::with_message(ErrorCode::DatabaseError, "Failed to count opera logs")
        })?;

        // 分页查询
        let logs = select
            .offset(offset as u64)
            .limit(size as u64)
            .all(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to query opera logs: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, "Failed to query opera logs")
            })?;

        // 转换为 DTO
        let list = logs
            .into_iter()
            .map(|log| {
                OperaLogListItem {
                    id: log.id,
                    trace_id: log.trace_id,
                    username: log.username,
                    method: log.method,
                    title: log.title,
                    path: log.path,
                    ip: log.ip,
                    country: log.country,
                    region: log.region,
                    city: log.city,
                    user_agent: log.user_agent,
                    os: log.os,
                    browser: log.browser,
                    device: log.device,
                    status: log.status,
                    code: log.code,
                    msg: log.msg,
                    cost_time: log.cost_time,
                    opera_time: log.opera_time.format("%Y-%m-%d %H:%M:%S").to_string(),
                }
            })
            .collect();

        let total_pages = (total as usize).div_ceil(size);

        Ok(OperaLogPaginationResponse {
            items: list,
            total: total as usize,
            page,
            size,
            total_pages,
        })
    }

    /// 获取操作日志详情
    /// TODO: 需要根据新的数据库结构重新实现
    pub async fn get_opera_log_detail(&self, _log_id: i64) -> Result<OperaLogDetailResponse, AppError> {
        Err(AppError::with_message(ErrorCode::InternalServerError, "获取详情功能暂未实现"))
    }

    /// 删除操作日志
    pub async fn delete_opera_log(&self, _log_id: i64) -> Result<(), AppError> {
        Err(AppError::with_message(ErrorCode::InternalServerError, "删除功能暂未实现"))
    }

    /// 批量删除操作日志
    pub async fn delete_opera_logs_batch(&self, _log_ids: &[i64]) -> Result<usize, AppError> {
        Err(AppError::with_message(ErrorCode::InternalServerError, "批量删除功能暂未实现"))
    }

    /// 清空操作日志
    pub async fn clear_opera_logs(&self) -> Result<usize, AppError> {
        Err(AppError::with_message(ErrorCode::InternalServerError, "清空功能暂未实现"))
    }

    /// 获取操作日志统计
    /// TODO: 需要根据新的数据库结构重新实现
    pub async fn get_opera_log_statistics(
        &self,
        _start_time: Option<chrono::DateTime<chrono::Utc>>,
        _end_time: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<OperaLogStatistics, AppError> {
        Err(AppError::with_message(ErrorCode::InternalServerError, "统计功能暂未实现"))
    }

    /// 根据用户ID查询操作日志
    /// TODO: 需要根据新的数据库结构重新实现
    pub async fn get_opera_logs_by_user(
        &self,
        _user_id: i64,
        _query: &OperaLogPaginationQuery,
    ) -> Result<OperaLogPaginationResponse, AppError> {
        Err(AppError::with_message(ErrorCode::InternalServerError, "按用户查询功能暂未实现"))
    }
}

/// 获取业务类型名称（保留用于将来使用）
#[allow(dead_code)]
fn get_business_type_name(business_type: i32) -> String {
    match business_type {
        0 => "其它",
        1 => "新增",
        2 => "修改",
        3 => "删除",
        4 => "授权",
        5 => "导出",
        6 => "导入",
        7 => "强退",
        8 => "生成代码",
        9 => "清空数据",
        _ => "未知",
    }
    .to_string()
}

/// 获取操作员类型名称（保留用于将来使用）
#[allow(dead_code)]
fn get_operator_type_name(operator_type: i32) -> String {
    match operator_type {
        0 => "其它",
        1 => "后台用户",
        2 => "手机端用户",
        _ => "未知",
    }
    .to_string()
}
