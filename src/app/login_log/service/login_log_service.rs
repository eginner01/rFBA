use tracing::{info, warn, error};

/// 登录日志服务实现
/// 提供登录日志记录、查询、统计等功能

use crate::app::login_log::dto::{
    CreateLoginLogRequest, CreateLoginLogResponse, CreateLogoutLogRequest, CreateLogoutLogResponse,
    LoginLogPaginationQuery, LoginLogPaginationResponse, LoginLogListItem, LoginLogDetailResponse,
    LoginLogStatistics, LoginIpStat, FailureReasonStat,
};
use crate::common::exception::{AppError, ErrorCode};
use crate::database::entity::login_log;
use sea_orm::{
    DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait,
    Order, Condition, QueryOrder, QuerySelect, PaginatorTrait, ModelTrait,
    ActiveModelTrait,
};
use chrono::{Utc, Duration, NaiveTime, Datelike};
use std::collections::HashMap;

/// 登录日志服务
pub struct LoginLogService {
    db: DatabaseConnection,
}

impl LoginLogService {
    /// 创建新的登录日志服务
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// 创建登录日志
    pub async fn create_login_log(
        &self,
        request: &CreateLoginLogRequest,
    ) -> Result<CreateLoginLogResponse, AppError> {
        let now = Utc::now().naive_utc();

        let user_uuid = request
            .user_id
            .map(|id| id.to_string())
            .unwrap_or_else(|| request.username.clone());

        let user_agent = format!(
            "{} / {} / {}",
            request
                .os
                .clone()
                .unwrap_or_else(|| "Unknown".to_string()),
            request
                .browser
                .clone()
                .unwrap_or_else(|| "Unknown".to_string()),
            request
                .dev_type
                .clone()
                .unwrap_or_else(|| "Unknown".to_string()),
        );

        let active_model = login_log::ActiveModel {
            id: Default::default(),
            user_uuid: sea_orm::Set(user_uuid),
            username: sea_orm::Set(request.username.clone()),
            status: sea_orm::Set(request.status),
            ip: sea_orm::Set(request.ipaddr.clone()),
            country: sea_orm::Set(None),
            region: sea_orm::Set(None),
            city: sea_orm::Set(None),
            user_agent: sea_orm::Set(user_agent),
            browser: sea_orm::Set(request.browser.clone()),
            os: sea_orm::Set(request.os.clone()),
            device: sea_orm::Set(request.dev_type.clone()),
            msg: sea_orm::Set(request.msg.clone().unwrap_or_default()),
            login_time: sea_orm::Set(now),
            created_time: sea_orm::Set(now),
        };

        let saved_log = active_model.insert(&self.db).await.map_err(|e| {
            error!("Failed to create login log: {:?}", e);
            AppError::with_message(ErrorCode::DatabaseError, "Failed to create login log")
        })?;

        info!("Created login log for user: {} (id: {})", saved_log.username, saved_log.id);

        Ok(CreateLoginLogResponse {
            id: saved_log.id,
            username: saved_log.username,
            status: saved_log.status,
            // 实体中使用 NaiveDateTime，这里转换为 DateTime<Utc>
            access_time: saved_log.login_time.and_utc(),
        })
    }

    /// 创建注销日志
    pub async fn create_logout_log(
        &self,
        request: &CreateLogoutLogRequest,
    ) -> Result<CreateLogoutLogResponse, AppError> {
        let now = Utc::now().naive_utc();

        let username = request.username.clone();

        let user_uuid = request
            .user_id
            .map(|id| id.to_string())
            .unwrap_or_else(|| username.clone());

        let active_model = login_log::ActiveModel {
            id: Default::default(),
            user_uuid: sea_orm::Set(user_uuid),
            username: sea_orm::Set(username.clone()),
            status: sea_orm::Set(1),
            ip: sea_orm::Set("".to_string()),
            country: sea_orm::Set(None),
            region: sea_orm::Set(None),
            city: sea_orm::Set(None),
            user_agent: sea_orm::Set("".to_string()),
            browser: sea_orm::Set(None),
            os: sea_orm::Set(None),
            device: sea_orm::Set(None),
            msg: sea_orm::Set("用户注销".to_string()),
            login_time: sea_orm::Set(now),
            created_time: sea_orm::Set(now),
        };

        let saved_log = active_model.insert(&self.db).await.map_err(|e| {
            error!("Failed to create logout log: {:?}", e);
            AppError::with_message(ErrorCode::DatabaseError, "Failed to create logout log")
        })?;

        info!(
            "Created logout log for user: {} (id: {})",
            saved_log.username, saved_log.id
        );

        Ok(CreateLogoutLogResponse {
            id: saved_log.id,
            username: saved_log.username,
            logout_time: saved_log.login_time.and_utc(),
        })
    }

    /// 分页查询登录日志
    pub async fn get_login_logs_paginated(
        &self,
        query: &LoginLogPaginationQuery,
    ) -> Result<LoginLogPaginationResponse, AppError> {
        let page = query.page.unwrap_or(1);
        let size = query.size.unwrap_or(10);
        let offset = (page - 1) * size;

        let mut select = login_log::Entity::find();

        if let Some(ref keyword) = query.keyword {
            select = select.filter(
                Condition::any()
                    .add(login_log::Column::Username.like(format!("%{}%", keyword)))
                    .add(login_log::Column::Ip.like(format!("%{}%", keyword))),
            );
        }

        if let Some(ref username) = query.username {
            select = select.filter(login_log::Column::Username.like(format!("%{}%", username)));
        }

        if let Some(ref ipaddr) = query.ipaddr {
            select = select.filter(login_log::Column::Ip.like(format!("%{}%", ipaddr)));
        }

        if let Some(status) = query.status {
            select = select.filter(login_log::Column::Status.eq(status));
        }

        if let Some(start_time) = query.start_time {
            select = select.filter(login_log::Column::LoginTime.gte(start_time));
        }

        if let Some(end_time) = query.end_time {
            select = select.filter(login_log::Column::LoginTime.lte(end_time));
        }

        let sort_field = query.sort_by.as_ref().unwrap_or(&crate::app::login_log::dto::LoginLogSortField::AccessTime);
        let sort_order = query.sort_order.as_ref().unwrap_or(&crate::app::login_log::dto::SortOrder::Desc);

        let order_by = match sort_order {
            crate::app::login_log::dto::SortOrder::Asc => Order::Asc,
            crate::app::login_log::dto::SortOrder::Desc => Order::Desc,
        };

        select = select.order_by(
            match sort_field {
                crate::app::login_log::dto::LoginLogSortField::Id => login_log::Column::Id,
                crate::app::login_log::dto::LoginLogSortField::Username => login_log::Column::Username,
                crate::app::login_log::dto::LoginLogSortField::Ipaddr => login_log::Column::Ip,
                crate::app::login_log::dto::LoginLogSortField::Status => login_log::Column::Status,
                crate::app::login_log::dto::LoginLogSortField::LoginTime => login_log::Column::LoginTime,
                crate::app::login_log::dto::LoginLogSortField::AccessTime => login_log::Column::LoginTime,
            },
            order_by,
        );

        let total = select.clone().count(&self.db).await.map_err(|e| {
            error!("Failed to count login logs: {:?}", e);
            AppError::with_message(ErrorCode::DatabaseError, "Failed to count login logs")
        })?;

        let logs = select
            .offset(offset as u64)
            .limit(size as u64)
            .all(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to query login logs: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, "Failed to query login logs")
            })?;

        let items: Vec<LoginLogListItem> = logs
            .into_iter()
            .map(|log| LoginLogListItem {
                id: log.id,
                username: log.username,
                status: log.status,
                ip: log.ip,
                country: log.country,
                region: log.region,
                os: log.os,
                browser: log.browser,
                device: log.device,
                msg: log.msg,
                login_time: log.login_time.and_utc(),
                created_time: log.created_time.and_utc(),
            })
            .collect();

        let total_pages = total.div_ceil(size as u64);

        Ok(LoginLogPaginationResponse {
            items,
            total: total as usize,
            page,
            size,
            total_pages: total_pages as usize,
        })
    }

    /// 获取登录日志详情
    pub async fn get_login_log_detail(&self, log_id: i64) -> Result<LoginLogDetailResponse, AppError> {
        let log = login_log::Entity::find_by_id(log_id)
            .one(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to find login log: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, "Failed to find login log")
            })?
            .ok_or_else(|| AppError::with_message(ErrorCode::NotFound, "Login log not found"))?;

        let status_name = if log.status == 1 { "成功".to_string() } else { "失败".to_string() };

        // 将 NaiveDateTime 转为 DateTime<Utc>
        let access_time = log.login_time.and_utc();

        let login_location = match (&log.country, &log.region, &log.city) {
            (Some(country), Some(region), Some(city)) => {
                Some(format!("{} {} {}", country, region, city))
            }
            (Some(country), Some(region), None) => Some(format!("{} {}", country, region)),
            (Some(country), None, None) => Some(country.clone()),
            _ => None,
        };

        Ok(LoginLogDetailResponse {
            id: log.id,
            user_id: None,
            username: log.username,
            dept_id: None,
            dept_name: None,
            ipaddr: log.ip,
            login_location,
            browser: log.browser,
            os: log.os,
            dev_type: log.device,
            status: log.status,
            status_name,
            msg: Some(log.msg),
            access_time,
            logout_time: None,
            login_time: None,
            logout_time_ms: None,
        })
    }

    /// 删除登录日志
    pub async fn delete_login_log(&self, log_id: i64) -> Result<(), AppError> {
        let log = login_log::Entity::find_by_id(log_id)
            .one(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to find login log: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, "Failed to find login log")
            })?
            .ok_or_else(|| AppError::with_message(ErrorCode::NotFound, "Login log not found"))?;

        log.delete(&self.db).await.map_err(|e| {
            error!("Failed to delete login log: {:?}", e);
            AppError::with_message(ErrorCode::DatabaseError, "Failed to delete login log")
        })?;

        info!("Deleted login log: {}", log_id);

        Ok(())
    }

    /// 批量删除登录日志
    pub async fn delete_login_logs_batch(&self, log_ids: &[i64]) -> Result<usize, AppError> {
        if log_ids.is_empty() {
            return Ok(0);
        }

        let mut deleted_count = 0;
        for log_id in log_ids {
            match login_log::Entity::find_by_id(*log_id)
                .one(&self.db)
                .await
            {
                Ok(Some(log)) => {
                    if let Err(e) = log.delete(&self.db).await {
                        error!("Failed to delete login log {}: {:?}", log_id, e);
                    } else {
                        deleted_count += 1;
                    }
                }
                Ok(None) => {
                    warn!("Login log {} not found for deletion", log_id);
                }
                Err(e) => {
                    error!("Failed to find login log {} for deletion: {:?}", log_id, e);
                }
            }
        }

        Ok(deleted_count)
    }

    /// 清空登录日志
    pub async fn clear_login_logs(&self) -> Result<usize, AppError> {
        let logs = login_log::Entity::find()
            .all(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to find login logs for clearing: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, "Failed to find login logs")
            })?;

        let mut deleted_count = 0;
        for log in logs {
            if let Err(e) = log.delete(&self.db).await {
                error!("Failed to delete login log during clear: {:?}", e);
            } else {
                deleted_count += 1;
            }
        }

        info!("Cleared {} login logs", deleted_count);

        Ok(deleted_count)
    }

    /// 获取登录日志统计
    pub async fn get_login_log_statistics(
        &self,
        start_time: Option<chrono::DateTime<chrono::Utc>>,
        end_time: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<LoginLogStatistics, AppError> {
        let now = Utc::now();
        let today_start = now.date_naive().and_time(NaiveTime::MIN);
        let week_start = today_start - Duration::days(now.weekday().num_days_from_monday() as i64);
        let month_start = chrono::NaiveDate::from_ymd_opt(now.year(), now.month(), 1)
            .unwrap()
            .and_time(NaiveTime::MIN);

        let mut base_query = login_log::Entity::find();

        if let Some(start) = start_time {
            base_query = base_query.filter(login_log::Column::LoginTime.gte(start));
        }

        if let Some(end) = end_time {
            base_query = base_query.filter(login_log::Column::LoginTime.lte(end));
        }

        let total_count = base_query.clone().count(&self.db).await.map_err(|e| {
            error!("Failed to count total login logs: {:?}", e);
            AppError::with_message(ErrorCode::DatabaseError, "Failed to count total login logs")
        })?;

        let success_count = base_query
            .clone()
            .filter(login_log::Column::Status.eq(1))
            .count(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to count success login logs: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, "Failed to count success login logs")
            })?;

        let failure_count = total_count - success_count;

        let today_count = login_log::Entity::find()
            .filter(login_log::Column::LoginTime.gte(today_start))
            .count(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to count today login logs: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, "Failed to count today login logs")
            })?;

        let week_count = login_log::Entity::find()
            .filter(login_log::Column::LoginTime.gte(week_start))
            .count(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to count week login logs: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, "Failed to count week login logs")
            })?;

        let month_count = login_log::Entity::find()
            .filter(login_log::Column::LoginTime.gte(month_start))
            .count(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to count month login logs: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, "Failed to count month login logs")
            })?;

        // 获取活跃用户数（最近30天有登录的用户）
        let active_users = login_log::Entity::find()
            .filter(login_log::Column::LoginTime.gte(now - Duration::days(30)))
            .count(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to count active users: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, "Failed to count active users")
            })?;

        // 获取登录IP TOP10
        let all_logs = base_query
            .clone()
            .all(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to query all login logs: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, "Failed to query all login logs")
            })?;

        let mut ip_map: HashMap<String, usize> = HashMap::new();
        let mut failure_reason_map: HashMap<String, usize> = HashMap::new();

        for log in &all_logs {
            *ip_map.entry(log.ip.clone()).or_insert(0) += 1;

            if log.status == 0 && !log.msg.is_empty() {
                let reason = log.msg.clone();
                *failure_reason_map.entry(reason).or_insert(0) += 1;
            }
        }

        let mut top_ips: Vec<LoginIpStat> = ip_map
            .into_iter()
            .map(|(ip, count)| LoginIpStat { ip, count })
            .collect();

        top_ips.sort_by(|a, b| b.count.cmp(&a.count));
        top_ips.truncate(10);

        let mut failure_reasons: Vec<FailureReasonStat> = failure_reason_map
            .into_iter()
            .map(|(reason, count)| FailureReasonStat { reason, count })
            .collect();

        failure_reasons.sort_by(|a, b| b.count.cmp(&a.count));
        failure_reasons.truncate(10);

        Ok(LoginLogStatistics {
            total_count: total_count as usize,
            success_count: success_count as usize,
            failure_count: failure_count as usize,
            today_count: today_count as usize,
            week_count: week_count as usize,
            month_count: month_count as usize,
            active_users: active_users as usize,
            top_ips,
            failure_reasons,
        })
    }

    /// 根据用户ID查询登录日志
    pub async fn get_login_logs_by_user(
        &self,
        user_id: i64,
        query: &LoginLogPaginationQuery,
    ) -> Result<LoginLogPaginationResponse, AppError> {
        let page = query.page.unwrap_or(1);
        let size = query.size.unwrap_or(10);
        let offset = (page - 1) * size;

        let user_uuid = user_id.to_string();

        let mut select = login_log::Entity::find().filter(login_log::Column::UserUuid.eq(user_uuid));

        if let Some(ref keyword) = query.keyword {
            select = select.filter(
                Condition::any()
                    .add(login_log::Column::Username.like(format!("%{}%", keyword)))
                    .add(login_log::Column::Ip.like(format!("%{}%", keyword))),
            );
        }

        if let Some(start_time) = query.start_time {
            select = select.filter(login_log::Column::LoginTime.gte(start_time));
        }

        if let Some(end_time) = query.end_time {
            select = select.filter(login_log::Column::LoginTime.lte(end_time));
        }

        select = select.order_by(login_log::Column::LoginTime, Order::Desc);

        let total = select.clone().count(&self.db).await.map_err(|e| {
            error!("Failed to count login logs by user: {:?}", e);
            AppError::with_message(ErrorCode::DatabaseError, "Failed to count login logs")
        })?;

        let logs = select
            .offset(offset as u64)
            .limit(size as u64)
            .all(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to query login logs by user: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, "Failed to query login logs")
            })?;

        let items: Vec<LoginLogListItem> = logs
            .into_iter()
            .map(|log| LoginLogListItem {
                id: log.id,
                username: log.username,
                status: log.status,
                ip: log.ip,
                country: log.country,
                region: log.region,
                os: log.os,
                browser: log.browser,
                device: log.device,
                msg: log.msg,
                login_time: log.login_time.and_utc(),
                created_time: log.created_time.and_utc(),
            })
            .collect();

        let total_pages = total.div_ceil(size as u64);

        Ok(LoginLogPaginationResponse {
            items,
            total: total as usize,
            page,
            size,
            total_pages: total_pages as usize,
        })
    }
}
