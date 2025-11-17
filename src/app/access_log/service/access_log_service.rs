use tracing::{info, warn, error, debug};

/// 访问日志服务实现
/// 提供访问日志记录、查询、管理等功能

use crate::app::access_log::dto::{
    CreateAccessLogRequest, CreateAccessLogResponse,
    AccessLogPaginationQuery, AccessLogPaginationResponse,
    AccessLogDetailResponse, AccessLogListItem,
    AccessLogStatistics, UrlStat, IpStat, MethodStat,
};
use crate::common::exception::{AppError, ErrorCode};
use crate::database::DatabaseManager;
use crate::database::entity::access_log;
use sea_orm::{DatabaseConnection, DbErr, EntityTrait, QueryFilter, ColumnTrait, Select};
use std::collections::HashMap;

/// 访问日志服务
pub struct AccessLogService {
    db: DatabaseConnection,
}

impl AccessLogService {
    /// 创建新的访问日志服务
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// 创建访问日志
    pub async fn create_access_log(
        &self,
        request: &CreateAccessLogRequest,
    ) -> Result<CreateAccessLogResponse, AppError> {
        let active_model = access_log::ActiveModel {
            id: Default::default(),
            user_id: sea_orm::Set(request.user_id),
            user_name: sea_orm::Set(request.user_name.clone()),
            dept_id: sea_orm::Set(request.dept_id),
            dept_name: sea_orm::Set(request.dept_name.clone()),
            trace_id: sea_orm::Set(request.trace_id.clone()),
            parent_trace_id: sea_orm::Set(request.parent_trace_id.clone()),
            method: sea_orm::Set(request.method.clone()),
            url: sea_orm::Set(request.url.clone()),
            query_params: sea_orm::Set(request.query_params.clone()),
            request_body: sea_orm::Set(request.request_body.clone()),
            status_code: sea_orm::Set(request.status_code),
            response_body: sea_orm::Set(request.response_body.clone()),
            client_ip: sea_orm::Set(request.client_ip.clone()),
            user_agent: sea_orm::Set(request.user_agent.clone()),
            os: sea_orm::Set(request.os.clone()),
            browser: sea_orm::Set(request.browser.clone()),
            device_type: sea_orm::Set(request.device_type.clone()),
            referer: sea_orm::Set(request.referer.clone()),
            cost_time: sea_orm::Set(request.cost_time),
            is_error: sea_orm::Set(request.is_error),
            error_msg: sea_orm::Set(request.error_msg.clone()),
            access_time: sea_orm::Set(chrono::Utc::now().naive_utc()),
        };

        let saved_model = active_model.insert(&self.db).await.map_err(|e| {
            error!("Failed to create access log: {:?}", e);
            AppError::new(ErrorCode::DatabaseError, "Failed to create access log")
        })?;

        Ok(CreateAccessLogResponse {
            id: saved_model.id,
            trace_id: saved_model.trace_id,
            access_time: saved_model.access_time,
        })
    }

    /// 分页查询访问日志
    pub async fn get_access_logs_paginated(
        &self,
        query: &AccessLogPaginationQuery,
    ) -> Result<AccessLogPaginationResponse, AppError> {
        let page = query.page.unwrap_or(1);
        let size = query.size.unwrap_or(10);
        let offset = (page - 1) * size;

        let mut select = access_log::Entity::find();

        if let Some(ref keyword) = query.keyword {
            select = select.filter(
                sea_orm::Condition::any()
                    .add(access_log::Column::UserName.like(format!("%{}%", keyword)))
                    .add(access_log::Column::Url.like(format!("%{}%", keyword)))
                    .add(access_log::Column::ClientIp.like(format!("%{}%", keyword))),
            );
        }

        if let Some(ref user_name) = query.user_name {
            select = select.filter(access_log::Column::UserName.like(format!("%{}%", user_name)));
        }

        if let Some(ref url) = query.url {
            select = select.filter(access_log::Column::Url.like(format!("%{}%", url)));
        }

        if let Some(ref method) = query.method {
            select = select.filter(access_log::Column::Method.like(format!("%{}%", method)));
        }

        if let Some(ref client_ip) = query.client_ip {
            select = select.filter(access_log::Column::ClientIp.like(format!("%{}%", client_ip)));
        }

        if let Some(status_code) = query.status_code {
            select = select.filter(access_log::Column::StatusCode.eq(status_code));
        }

        if let Some(is_error) = query.is_error {
            select = select.filter(access_log::Column::IsError.eq(is_error));
        }

        if let Some(start_time) = query.start_time {
            select = select.filter(access_log::Column::AccessTime.gte(start_time));
        }

        if let Some(end_time) = query.end_time {
            select = select.filter(access_log::Column::AccessTime.lte(end_time));
        }

        let sort_field = query.sort_by.as_ref().unwrap_or(&crate::app::access_log::dto::AccessLogSortField::AccessTime);
        let sort_order = query.sort_order.as_ref().unwrap_or(&crate::app::access_log::dto::SortOrder::Desc);

        let order_by = match sort_order {
            crate::app::access_log::dto::SortOrder::Asc => sea_orm::Order::Asc,
            crate::app::access_log::dto::SortOrder::Desc => sea_orm::Order::Desc,
        };

        select = select.order_by(
            match sort_field {
                crate::app::access_log::dto::AccessLogSortField::Id => access_log::Column::Id,
                crate::app::access_log::dto::AccessLogSortField::Method => access_log::Column::Method,
                crate::app::access_log::dto::AccessLogSortField::Url => access_log::Column::Url,
                crate::app::access_log::dto::AccessLogSortField::ClientIp => access_log::Column::ClientIp,
                crate::app::access_log::dto::AccessLogSortField::StatusCode => access_log::Column::StatusCode,
                crate::app::access_log::dto::AccessLogSortField::CostTime => access_log::Column::CostTime,
                crate::app::access_log::dto::AccessLogSortField::AccessTime => access_log::Column::AccessTime,
            },
            order_by,
        );

        let total = select.clone().count(&self.db).await.map_err(|e| {
            error!("Failed to count access logs: {:?}", e);
            AppError::new(ErrorCode::DatabaseError, "Failed to count access logs")
        })?;

        let logs = select
            .offset(offset as u64)
            .limit(size as u64)
            .all(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to query access logs: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to query access logs")
            })?;

        let list = logs
            .into_iter()
            .map(|log| {
                AccessLogListItem {
                    id: log.id,
                    user_name: log.user_name,
                    method: log.method,
                    url: log.url,
                    client_ip: log.client_ip,
                    status_code: log.status_code,
                    cost_time: log.cost_time,
                    is_error: log.is_error,
                    access_time: log.access_time,
                }
            })
            .collect();

        let pages = (total + size - 1) / size;

        Ok(AccessLogPaginationResponse {
            list,
            total,
            page,
            size,
            pages,
        })
    }

    /// 获取访问日志详情
    pub async fn get_access_log_detail(
        &self,
        log_id: i64,
    ) -> Result<AccessLogDetailResponse, AppError> {
        let log = access_log::Entity::find_by_id(log_id)
            .one(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to find access log: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to find access log")
            })?
            .ok_or_else(|| {
                AppError::new(ErrorCode::NotFound, "Access log not found")
            })?;

        Ok(AccessLogDetailResponse {
            id: log.id,
            user_id: log.user_id,
            user_name: log.user_name,
            dept_id: log.dept_id,
            dept_name: log.dept_name,
            trace_id: log.trace_id,
            parent_trace_id: log.parent_trace_id,
            method: log.method,
            url: log.url,
            query_params: log.query_params,
            request_body: log.request_body,
            status_code: log.status_code,
            response_body: log.response_body,
            client_ip: log.client_ip,
            user_agent: log.user_agent,
            os: log.os,
            browser: log.browser,
            device_type: log.device_type,
            referer: log.referer,
            cost_time: log.cost_time,
            is_error: log.is_error,
            error_msg: log.error_msg,
            access_time: log.access_time,
        })
    }

    /// 删除访问日志
    pub async fn delete_access_log(&self, log_id: i64) -> Result<(), AppError> {
        let log = access_log::Entity::find_by_id(log_id)
            .one(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to find access log for deletion: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to find access log")
            })?
            .ok_or_else(|| {
                AppError::new(ErrorCode::NotFound, "Access log not found")
            })?;

        let active_model = log.into_active_model();
        active_model.delete(&self.db).await.map_err(|e| {
            error!("Failed to delete access log: {:?}", e);
            AppError::new(ErrorCode::DatabaseError, "Failed to delete access log")
        })?;

        Ok(())
    }

    /// 批量删除访问日志
    pub async fn delete_access_logs_batch(&self, log_ids: &[i64]) -> Result<usize, AppError> {
        if log_ids.is_empty() {
            return Ok(0);
        }

        let mut deleted_count = 0;
        for log_id in log_ids {
            match access_log::Entity::find_by_id(*log_id)
                .one(&self.db)
                .await
            {
                Ok(Some(log)) => {
                    let active_model = log.into_active_model();
                    if let Err(e) = active_model.delete(&self.db).await {
                        error!("Failed to delete access log {}: {:?}", log_id, e);
                    } else {
                        deleted_count += 1;
                    }
                }
                Ok(None) => {
                    warn!("Access log {} not found for deletion", log_id);
                }
                Err(e) => {
                    error!("Failed to find access log {} for deletion: {:?}", log_id, e);
                }
            }
        }

        Ok(deleted_count)
    }

    /// 清空访问日志
    pub async fn clear_access_logs(&self) -> Result<usize, AppError> {
        let logs = access_log::Entity::find()
            .all(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to find access logs for clearing: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to find access logs")
            })?;

        let mut deleted_count = 0;
        for log in logs {
            let active_model = log.into_active_model();
            if let Err(e) = active_model.delete(&self.db).await {
                error!("Failed to delete access log during clear: {:?}", e);
            } else {
                deleted_count += 1;
            }
        }

        Ok(deleted_count)
    }

    /// 获取访问日志统计
    pub async fn get_access_log_statistics(
        &self,
        start_time: Option<chrono::DateTime<chrono::Utc>>,
        end_time: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<AccessLogStatistics, AppError> {
        let now = chrono::Utc::now();
        let today_start = now.date_naive().and_time(chrono::NaTime::MIN);
        let week_start = today_start - chrono::Duration::days(now.weekday().num_days_from_monday() as i64);
        let month_start = chrono::NaiveDate::from_ymd_opt(now.year(), now.month(), 1)
            .unwrap()
            .and_time(chrono::NaTime::MIN);

        let mut base_query = access_log::Entity::find();

        if let Some(start) = start_time {
            base_query = base_query.filter(access_log::Column::AccessTime.gte(start));
        }

        if let Some(end) = end_time {
            base_query = base_query.filter(access_log::Column::AccessTime.lte(end));
        }

        let total_count = base_query.clone().count(&self.db).await.map_err(|e| {
            error!("Failed to count total access logs: {:?}", e);
            AppError::new(ErrorCode::DatabaseError, "Failed to count total access logs")
        })?;

        let success_count = base_query
            .clone()
            .filter(access_log::Column::IsError.eq(false))
            .count(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to count success access logs: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to count success access logs")
            })?;

        let failure_count = total_count - success_count;

        let all_logs = base_query
            .clone()
            .all(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to query all access logs: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to query all access logs")
            })?;

        let avg_response_time = if !all_logs.is_empty() {
            let total_time: i64 = all_logs.iter().map(|log| log.cost_time).sum();
            Some(total_time / total_count as i64)
        } else {
            None
        };

        let max_response_time = all_logs.iter().map(|log| log.cost_time).max();

        let today_count = access_log::Entity::find()
            .filter(access_log::Column::AccessTime.gte(today_start))
            .count(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to count today access logs: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to count today access logs")
            })?;

        let week_count = access_log::Entity::find()
            .filter(access_log::Column::AccessTime.gte(week_start))
            .count(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to count week access logs: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to count week access logs")
            })?;

        let month_count = access_log::Entity::find()
            .filter(access_log::Column::AccessTime.gte(month_start))
            .count(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to count month access logs: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to count month access logs")
            })?;

        let mut url_map: HashMap<String, (usize, i64)> = HashMap::new();
        let mut ip_map: HashMap<String, usize> = HashMap::new();
        let mut method_map: HashMap<String, usize> = HashMap::new();

        for log in &all_logs {
            let entry = url_map.entry(log.url.clone()).or_insert((0, 0));
            entry.0 += 1;
            entry.1 += log.cost_time;

            *ip_map.entry(log.client_ip.clone()).or_insert(0) += 1;
            *method_map.entry(log.method.clone()).or_insert(0) += 1;
        }

        let mut top_urls: Vec<UrlStat> = url_map
            .into_iter()
            .map(|(url, (count, total_time))| {
                let avg_response_time = if count > 0 {
                    Some(total_time / count as i64)
                } else {
                    None
                };
                UrlStat { url, count, avg_response_time }
            })
            .collect();

        top_urls.sort_by(|a, b| b.count.cmp(&a.count));
        top_urls.truncate(10);

        let top_ips: Vec<IpStat> = ip_map
            .into_iter()
            .map(|(ip, count)| IpStat { ip, count })
            .collect();

        top_ips.sort_by(|a, b| b.count.cmp(&a.count));
        top_ips.truncate(10);

        let method_stats: Vec<MethodStat> = method_map
            .into_iter()
            .map(|(method, count)| {
                let percentage = if total_count > 0 {
                    (count as f64 / total_count as f64) * 100.0
                } else {
                    0.0
                };
                MethodStat { method, count, percentage }
            })
            .collect();

        Ok(AccessLogStatistics {
            total_count,
            success_count,
            failure_count,
            avg_response_time,
            max_response_time,
            today_count,
            week_count,
            month_count,
            top_urls,
            top_ips,
            method_stats,
        })
    }

    /// 根据用户ID查询访问日志
    pub async fn get_access_logs_by_user(
        &self,
        user_id: i64,
        query: &AccessLogPaginationQuery,
    ) -> Result<AccessLogPaginationResponse, AppError> {
        let page = query.page.unwrap_or(1);
        let size = query.size.unwrap_or(10);
        let offset = (page - 1) * size;

        let mut select = access_log::Entity::find().filter(access_log::Column::UserId.eq(user_id));

        if let Some(ref keyword) = query.keyword {
            select = select.filter(
                sea_orm::Condition::any()
                    .add(access_log::Column::UserName.like(format!("%{}%", keyword)))
                    .add(access_log::Column::Url.like(format!("%{}%", keyword))),
            );
        }

        if let Some(start_time) = query.start_time {
            select = select.filter(access_log::Column::AccessTime.gte(start_time));
        }

        if let Some(end_time) = query.end_time {
            select = select.filter(access_log::Column::AccessTime.lte(end_time));
        }

        select = select.order_by(access_log::Column::AccessTime, sea_orm::Order::Desc);

        let total = select.clone().count(&self.db).await.map_err(|e| {
            error!("Failed to count access logs by user: {:?}", e);
            AppError::new(ErrorCode::DatabaseError, "Failed to count access logs")
        })?;

        let logs = select
            .offset(offset as u64)
            .limit(size as u64)
            .all(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to query access logs by user: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to query access logs")
            })?;

        let list = logs
            .into_iter()
            .map(|log| {
                AccessLogListItem {
                    id: log.id,
                    user_name: log.user_name,
                    method: log.method,
                    url: log.url,
                    client_ip: log.client_ip,
                    status_code: log.status_code,
                    cost_time: log.cost_time,
                    is_error: log.is_error,
                    access_time: log.access_time,
                }
            })
            .collect();

        let pages = (total + size - 1) / size;

        Ok(AccessLogPaginationResponse {
            list,
            total,
            page,
            size,
            pages,
        })
    }

    /// 根据URL查询访问日志
    pub async fn get_access_logs_by_url(
        &self,
        url: &str,
        query: &AccessLogPaginationQuery,
    ) -> Result<AccessLogPaginationResponse, AppError> {
        let page = query.page.unwrap_or(1);
        let size = query.size.unwrap_or(10);
        let offset = (page - 1) * size;

        let mut select = access_log::Entity::find().filter(access_log::Column::Url.like(format!("%{}%", url)));

        if let Some(ref keyword) = query.keyword {
            select = select.filter(
                sea_orm::Condition::any()
                    .add(access_log::Column::UserName.like(format!("%{}%", keyword)))
                    .add(access_log::Column::Url.like(format!("%{}%", keyword))),
            );
        }

        if let Some(start_time) = query.start_time {
            select = select.filter(access_log::Column::AccessTime.gte(start_time));
        }

        if let Some(end_time) = query.end_time {
            select = select.filter(access_log::Column::AccessTime.lte(end_time));
        }

        select = select.order_by(access_log::Column::AccessTime, sea_orm::Order::Desc);

        let total = select.clone().count(&self.db).await.map_err(|e| {
            error!("Failed to count access logs by url: {:?}", e);
            AppError::new(ErrorCode::DatabaseError, "Failed to count access logs")
        })?;

        let logs = select
            .offset(offset as u64)
            .limit(size as u64)
            .all(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to query access logs by url: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to query access logs")
            })?;

        let list = logs
            .into_iter()
            .map(|log| {
                AccessLogListItem {
                    id: log.id,
                    user_name: log.user_name,
                    method: log.method,
                    url: log.url,
                    client_ip: log.client_ip,
                    status_code: log.status_code,
                    cost_time: log.cost_time,
                    is_error: log.is_error,
                    access_time: log.access_time,
                }
            })
            .collect();

        let pages = (total + size - 1) / size;

        Ok(AccessLogPaginationResponse {
            list,
            total,
            page,
            size,
            pages,
        })
    }
}
