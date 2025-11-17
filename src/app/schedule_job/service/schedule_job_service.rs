use tracing::{info, warn, error, debug};

/// 任务调度服务实现
/// 提供任务调度管理、查询、统计等功能

use crate::app::schedule_job::dto::{
    CreateScheduleJobRequest, CreateScheduleJobResponse,
    UpdateScheduleJobRequest, UpdateScheduleJobResponse,
    ExecuteScheduleJobRequest, ExecuteScheduleJobResponse,
    ScheduleJobPaginationQuery, ScheduleJobPaginationResponse,
    ScheduleJobDetailResponse, ScheduleJobListItem,
    ScheduleJobStatistics, ScheduleJobLogPaginationQuery,
    ScheduleJobLogPaginationResponse, ScheduleJobLogListItem,
    ScheduleJobLogDetailResponse,
};
use crate::common::exception::{AppError, ErrorCode};
use crate::database::DatabaseManager;
use crate::database::entity::schedule_job;
use crate::database::entity::schedule_job_log;
use sea_orm::{DatabaseConnection, DbErr, EntityTrait, QueryFilter, ColumnTrait, Select};
use std::collections::HashMap;

/// 任务调度服务
pub struct ScheduleJobService {
    db: DatabaseConnection,
}

impl ScheduleJobService {
    /// 创建新的任务调度服务
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// 创建任务调度
    pub async fn create_schedule_job(
        &self,
        request: &CreateScheduleJobRequest,
    ) -> Result<CreateScheduleJobResponse, AppError> {
        let active_model = schedule_job::ActiveModel {
            id: Default::default(),
            job_name: sea_orm::Set(request.job_name.clone()),
            job_group: sea_orm::Set(request.job_group.clone()),
            bean_name: sea_orm::Set(request.bean_name.clone()),
            method_name: sea_orm::Set(request.method_name.clone()),
            method_params: sea_orm::Set(request.method_params.clone()),
            cron_expression: sea_orm::Set(request.cron_expression.clone()),
            misfire_policy: sea_orm::Set(request.misfire_policy),
            concurrent: sea_orm::Set(request.concurrent),
            status: sea_orm::Set(request.status),
            priority: sea_orm::Set(request.priority),
            timeout: sea_orm::Set(request.timeout),
            retry_count: sea_orm::Set(request.retry_count),
            retry_interval: sea_orm::Set(request.retry_interval),
            description: sea_orm::Set(request.description.clone()),
            create_by: sea_orm::Set(None),
            update_by: sea_orm::Set(None),
            created_time: sea_orm::Set(chrono::Utc::now().naive_utc()),
            updated_time: sea_orm::Set(chrono::Utc::now().naive_utc()),
        };

        let saved_model = active_model.insert(&self.db).await.map_err(|e| {
            error!("Failed to create schedule job: {:?}", e);
            AppError::new(ErrorCode::DatabaseError, "Failed to create schedule job")
        })?;

        Ok(CreateScheduleJobResponse {
            id: saved_model.id,
            job_name: saved_model.job_name,
            job_group: saved_model.job_group,
            cron_expression: saved_model.cron_expression,
            created_time: saved_model.created_time,
        })
    }

    /// 更新任务调度
    pub async fn update_schedule_job(
        &self,
        request: &UpdateScheduleJobRequest,
    ) -> Result<UpdateScheduleJobResponse, AppError> {
        let existing_job = schedule_job::Entity::find_by_id(request.id)
            .one(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to find schedule job: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to find schedule job")
            })?
            .ok_or_else(|| {
                AppError::new(ErrorCode::NotFound, "Schedule job not found")
            })?;

        let mut active_model = existing_job.into_active_model();
        active_model.job_name = sea_orm::Set(request.job_name.clone());
        active_model.job_group = sea_orm::Set(request.job_group.clone());
        active_model.bean_name = sea_orm::Set(request.bean_name.clone());
        active_model.method_name = sea_orm::Set(request.method_name.clone());
        active_model.method_params = sea_orm::Set(request.method_params.clone());
        active_model.cron_expression = sea_orm::Set(request.cron_expression.clone());
        active_model.misfire_policy = sea_orm::Set(request.misfire_policy);
        active_model.concurrent = sea_orm::Set(request.concurrent);
        active_model.status = sea_orm::Set(request.status);
        active_model.priority = sea_orm::Set(request.priority);
        active_model.timeout = sea_orm::Set(request.timeout);
        active_model.retry_count = sea_orm::Set(request.retry_count);
        active_model.retry_interval = sea_orm::Set(request.retry_interval);
        active_model.description = sea_orm::Set(request.description.clone());
        active_model.updated_time = sea_orm::Set(chrono::Utc::now().naive_utc());

        let updated_model = active_model.update(&self.db).await.map_err(|e| {
            error!("Failed to update schedule job: {:?}", e);
            AppError::new(ErrorCode::DatabaseError, "Failed to update schedule job")
        })?;

        Ok(UpdateScheduleJobResponse {
            id: updated_model.id,
            job_name: updated_model.job_name,
            job_group: updated_model.job_group,
            updated_time: updated_model.updated_time,
        })
    }

    /// 分页查询任务调度
    pub async fn get_schedule_jobs_paginated(
        &self,
        query: &ScheduleJobPaginationQuery,
    ) -> Result<ScheduleJobPaginationResponse, AppError> {
        let page = query.page.unwrap_or(1);
        let size = query.size.unwrap_or(10);
        let offset = (page - 1) * size;

        let mut select = schedule_job::Entity::find();

        if let Some(ref keyword) = query.keyword {
            select = select.filter(
                sea_orm::Condition::any()
                    .add(schedule_job::Column::JobName.like(format!("%{}%", keyword)))
                    .add(schedule_job::Column::JobGroup.like(format!("%{}%", keyword)))
                    .add(schedule_job::Column::BeanName.like(format!("%{}%", keyword))),
            );
        }

        if let Some(ref job_name) = query.job_name {
            select = select.filter(schedule_job::Column::JobName.like(format!("%{}%", job_name)));
        }

        if let Some(ref job_group) = query.job_group {
            select = select.filter(schedule_job::Column::JobGroup.like(format!("%{}%", job_group)));
        }

        if let Some(ref bean_name) = query.bean_name {
            select = select.filter(schedule_job::Column::BeanName.like(format!("%{}%", bean_name)));
        }

        if let Some(status) = query.status {
            select = select.filter(schedule_job::Column::Status.eq(status));
        }

        if let Some(start_time) = query.start_time {
            select = select.filter(schedule_job::Column::CreatedTime.gte(start_time));
        }

        if let Some(end_time) = query.end_time {
            select = select.filter(schedule_job::Column::CreatedTime.lte(end_time));
        }

        let sort_field = query.sort_by.as_ref().unwrap_or(&ScheduleJobSortField::CreatedTime);
        let sort_order = query.sort_order.as_ref().unwrap_or(&SortOrder::Desc);

        let order_by = match sort_order {
            SortOrder::Asc => sea_orm::Order::Asc,
            SortOrder::Desc => sea_orm::Order::Desc,
        };

        select = select.order_by(
            match sort_field {
                ScheduleJobSortField::Id => schedule_job::Column::Id,
                ScheduleJobSortField::JobName => schedule_job::Column::JobName,
                ScheduleJobSortField::JobGroup => schedule_job::Column::JobGroup,
                ScheduleJobSortField::BeanName => schedule_job::Column::BeanName,
                ScheduleJobSortField::Status => schedule_job::Column::Status,
                ScheduleJobSortField::CreatedTime => schedule_job::Column::CreatedTime,
                ScheduleJobSortField::UpdatedTime => schedule_job::Column::UpdatedTime,
            },
            order_by,
        );

        let total = select.clone().count(&self.db).await.map_err(|e| {
            error!("Failed to count schedule jobs: {:?}", e);
            AppError::new(ErrorCode::DatabaseError, "Failed to count schedule jobs")
        })?;

        let jobs = select
            .offset(offset as u64)
            .limit(size as u64)
            .all(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to query schedule jobs: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to query schedule jobs")
            })?;

        let list = jobs
            .into_iter()
            .map(|job| {
                let misfire_policy_name =
                    crate::database::entity::schedule_job::MisfirePolicy::from_i32(job.misfire_policy)
                        .map(|mp| mp.get_name().to_string())
                        .unwrap_or_else(|| "未知".to_string());

                let status_name =
                    crate::database::entity::schedule_job::JobStatus::from_i32(job.status)
                        .map(|js| js.get_name().to_string())
                        .unwrap_or_else(|| "未知".to_string());

                ScheduleJobListItem {
                    id: job.id,
                    job_name: job.job_name,
                    job_group: job.job_group,
                    bean_name: job.bean_name,
                    method_name: job.method_name,
                    cron_expression: job.cron_expression,
                    misfire_policy_name,
                    concurrent: job.concurrent,
                    status: job.status,
                    status_name,
                    priority: job.priority,
                    description: job.description,
                    created_time: job.created_time,
                    updated_time: job.updated_time,
                }
            })
            .collect();

        let pages = (total + size - 1) / size;

        Ok(ScheduleJobPaginationResponse {
            list,
            total,
            page,
            size,
            pages,
        })
    }

    /// 获取任务调度详情
    pub async fn get_schedule_job_detail(
        &self,
        job_id: i64,
    ) -> Result<ScheduleJobDetailResponse, AppError> {
        let job = schedule_job::Entity::find_by_id(job_id)
            .one(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to find schedule job: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to find schedule job")
            })?
            .ok_or_else(|| {
                AppError::new(ErrorCode::NotFound, "Schedule job not found")
            })?;

        let misfire_policy_name =
            crate::database::entity::schedule_job::MisfirePolicy::from_i32(job.misfire_policy)
                .map(|mp| mp.get_name().to_string())
                .unwrap_or_else(|| "未知".to_string());

        let status_name =
            crate::database::entity::schedule_job::JobStatus::from_i32(job.status)
                .map(|js| js.get_name().to_string())
                .unwrap_or_else(|| "未知".to_string());

        Ok(ScheduleJobDetailResponse {
            id: job.id,
            job_name: job.job_name,
            job_group: job.job_group,
            bean_name: job.bean_name,
            method_name: job.method_name,
            method_params: job.method_params,
            cron_expression: job.cron_expression,
            misfire_policy: job.misfire_policy,
            misfire_policy_name,
            concurrent: job.concurrent,
            status: job.status,
            status_name,
            priority: job.priority,
            timeout: job.timeout,
            retry_count: job.retry_count,
            retry_interval: job.retry_interval,
            description: job.description,
            create_by: job.create_by,
            update_by: job.update_by,
            created_time: job.created_time,
            updated_time: job.updated_time,
        })
    }

    /// 删除任务调度
    pub async fn delete_schedule_job(&self, job_id: i64) -> Result<(), AppError> {
        let job = schedule_job::Entity::find_by_id(job_id)
            .one(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to find schedule job for deletion: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to find schedule job")
            })?
            .ok_or_else(|| {
                AppError::new(ErrorCode::NotFound, "Schedule job not found")
            })?;

        let active_model = job.into_active_model();
        active_model.delete(&self.db).await.map_err(|e| {
            error!("Failed to delete schedule job: {:?}", e);
            AppError::new(ErrorCode::DatabaseError, "Failed to delete schedule job")
        })?;

        Ok(())
    }

    /// 批量删除任务调度
    pub async fn delete_schedule_jobs_batch(&self, job_ids: &[i64]) -> Result<usize, AppError> {
        if job_ids.is_empty() {
            return Ok(0);
        }

        let mut deleted_count = 0;
        for job_id in job_ids {
            match schedule_job::Entity::find_by_id(*job_id)
                .one(&self.db)
                .await
            {
                Ok(Some(job)) => {
                    let active_model = job.into_active_model();
                    if let Err(e) = active_model.delete(&self.db).await {
                        error!("Failed to delete schedule job {}: {:?}", job_id, e);
                    } else {
                        deleted_count += 1;
                    }
                }
                Ok(None) => {
                    warn!("Schedule job {} not found for deletion", job_id);
                }
                Err(e) => {
                    error!("Failed to find schedule job {} for deletion: {:?}", job_id, e);
                }
            }
        }

        Ok(deleted_count)
    }

    /// 更新任务状态
    pub async fn update_job_status(
        &self,
        job_id: i64,
        status: i32,
    ) -> Result<(), AppError> {
        let job = schedule_job::Entity::find_by_id(job_id)
            .one(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to find schedule job: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to find schedule job")
            })?
            .ok_or_else(|| {
                AppError::new(ErrorCode::NotFound, "Schedule job not found")
            })?;

        let mut active_model = job.into_active_model();
        active_model.status = sea_orm::Set(status);
        active_model.updated_time = sea_orm::Set(chrono::Utc::now().naive_utc());

        active_model.update(&self.db).await.map_err(|e| {
            error!("Failed to update schedule job status: {:?}", e);
            AppError::new(ErrorCode::DatabaseError, "Failed to update schedule job status")
        })?;

        Ok(())
    }

    /// 立即执行任务
    pub async fn execute_job_immediately(
        &self,
        request: &ExecuteScheduleJobRequest,
    ) -> Result<ExecuteScheduleJobResponse, AppError> {
        let job = schedule_job::Entity::find_by_id(request.id)
            .one(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to find schedule job: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to find schedule job")
            })?
            .ok_or_else(|| {
                AppError::new(ErrorCode::NotFound, "Schedule job not found")
            })?;

        let execute_id = format!("EXEC-{}-{}", job.id, chrono::Utc::now().timestamp());

        Ok(ExecuteScheduleJobResponse {
            execute_id,
            job_id: job.id,
            job_name: job.job_name,
            execute_time: chrono::Utc::now(),
        })
    }

    /// 获取任务调度统计
    pub async fn get_schedule_job_statistics(
        &self,
    ) -> Result<ScheduleJobStatistics, AppError> {
        let now = chrono::Utc::now();
        let today_start = now.date_naive().and_time(chrono::NaTime::MIN);
        let week_start = today_start - chrono::Duration::days(now.weekday().num_days_from_monday() as i64);
        let month_start = chrono::NaiveDate::from_ymd_opt(now.year(), now.month(), 1)
            .unwrap()
            .and_time(chrono::NaTime::MIN);

        let total_count = schedule_job::Entity::find()
            .count(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to count schedule jobs: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to count schedule jobs")
            })?;

        let normal_count = schedule_job::Entity::find()
            .filter(schedule_job::Column::Status.eq(0))
            .count(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to count normal schedule jobs: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to count normal schedule jobs")
            })?;

        let paused_count = schedule_job::Entity::find()
            .filter(schedule_job::Column::Status.eq(1))
            .count(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to count paused schedule jobs: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to count paused schedule jobs")
            })?;

        let today_execute_count = schedule_job_log::Entity::find()
            .filter(schedule_job_log::Column::ExecuteTime.gte(today_start))
            .count(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to count today execute: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to count today execute")
            })?;

        let today_success_count = schedule_job_log::Entity::find()
            .filter(
                sea_orm::Condition::all()
                    .add(schedule_job_log::Column::ExecuteTime.gte(today_start))
                    .add(schedule_job_log::Column::Status.eq(0))
            )
            .count(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to count today success: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to count today success")
            })?;

        let today_failure_count = today_execute_count - today_success_count;

        let week_execute_count = schedule_job_log::Entity::find()
            .filter(schedule_job_log::Column::ExecuteTime.gte(week_start))
            .count(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to count week execute: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to count week execute")
            })?;

        let month_execute_count = schedule_job_log::Entity::find()
            .filter(schedule_job_log::Column::ExecuteTime.gte(month_start))
            .count(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to count month execute: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to count month execute")
            })?;

        Ok(ScheduleJobStatistics {
            total_count,
            normal_count,
            paused_count,
            today_execute_count,
            today_success_count,
            today_failure_count,
            week_execute_count,
            month_execute_count,
        })
    }

    /// 分页查询任务执行日志
    pub async fn get_schedule_job_logs_paginated(
        &self,
        query: &ScheduleJobLogPaginationQuery,
    ) -> Result<ScheduleJobLogPaginationResponse, AppError> {
        let page = query.page.unwrap_or(1);
        let size = query.size.unwrap_or(10);
        let offset = (page - 1) * size;

        let mut select = schedule_job_log::Entity::find();

        if let Some(job_id) = query.job_id {
            select = select.filter(schedule_job_log::Column::JobId.eq(job_id));
        }

        if let Some(ref job_name) = query.job_name {
            select = select.filter(schedule_job_log::Column::JobName.like(format!("%{}%", job_name)));
        }

        if let Some(status) = query.status {
            select = select.filter(schedule_job_log::Column::Status.eq(status));
        }

        if let Some(start_time) = query.start_time {
            select = select.filter(schedule_job_log::Column::ExecuteTime.gte(start_time));
        }

        if let Some(end_time) = query.end_time {
            select = select.filter(schedule_job_log::Column::ExecuteTime.lte(end_time));
        }

        let sort_field = query.sort_by.as_ref().unwrap_or(&ScheduleJobLogSortField::ExecuteTime);
        let sort_order = query.sort_order.as_ref().unwrap_or(&SortOrder::Desc);

        let order_by = match sort_order {
            SortOrder::Asc => sea_orm::Order::Asc,
            SortOrder::Desc => sea_orm::Order::Desc,
        };

        select = select.order_by(
            match sort_field {
                ScheduleJobLogSortField::Id => schedule_job_log::Column::Id,
                ScheduleJobLogSortField::JobId => schedule_job_log::Column::JobId,
                ScheduleJobLogSortField::ExecuteTime => schedule_job_log::Column::ExecuteTime,
                ScheduleJobLogSortField::CostTime => schedule_job_log::Column::CostTime,
                ScheduleJobLogSortField::Status => schedule_job_log::Column::Status,
            },
            order_by,
        );

        let total = select.clone().count(&self.db).await.map_err(|e| {
            error!("Failed to count schedule job logs: {:?}", e);
            AppError::new(ErrorCode::DatabaseError, "Failed to count schedule job logs")
        })?;

        let logs = select
            .offset(offset as u64)
            .limit(size as u64)
            .all(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to query schedule job logs: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to query schedule job logs")
            })?;

        let list = logs
            .into_iter()
            .map(|log| {
                let status_name =
                    crate::database::entity::schedule_job_log::JobLogStatus::from_i32(log.status)
                        .map(|js| js.get_name().to_string())
                        .unwrap_or_else(|| "未知".to_string());

                ScheduleJobLogListItem {
                    id: log.id,
                    job_id: log.job_id,
                    job_name: log.job_name,
                    job_group: log.job_group,
                    bean_name: log.bean_name,
                    method_name: log.method_name,
                    status: log.status,
                    status_name,
                    cost_time: log.cost_time,
                    execute_time: log.execute_time,
                    start_time: log.start_time,
                    end_time: log.end_time,
                    machine_ip: log.machine_ip,
                    method_params: log.method_params,
                }
            })
            .collect();

        let pages = (total + size - 1) / size;

        Ok(ScheduleJobLogPaginationResponse {
            list,
            total,
            page,
            size,
            pages,
        })
    }

    /// 获取任务执行日志详情
    pub async fn get_schedule_job_log_detail(
        &self,
        log_id: i64,
    ) -> Result<ScheduleJobLogDetailResponse, AppError> {
        let log = schedule_job_log::Entity::find_by_id(log_id)
            .one(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to find schedule job log: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to find schedule job log")
            })?
            .ok_or_else(|| {
                AppError::new(ErrorCode::NotFound, "Schedule job log not found")
            })?;

        let status_name =
            crate::database::entity::schedule_job_log::JobLogStatus::from_i32(log.status)
                .map(|js| js.get_name().to_string())
                .unwrap_or_else(|| "未知".to_string());

        Ok(ScheduleJobLogDetailResponse {
            id: log.id,
            job_id: log.job_id,
            job_name: log.job_name,
            job_group: log.job_group,
            bean_name: log.bean_name,
            method_name: log.method_name,
            method_params: log.method_params,
            status: log.status,
            status_name,
            exception: log.exception,
            exception_detail: log.exception_detail,
            cost_time: log.cost_time,
            execute_time: log.execute_time,
            start_time: log.start_time,
            end_time: log.end_time,
            job_params: log.job_params,
            machine_ip: log.machine_ip,
            machine_name: log.machine_name,
        })
    }

    /// 删除任务执行日志
    pub async fn delete_schedule_job_log(&self, log_id: i64) -> Result<(), AppError> {
        let log = schedule_job_log::Entity::find_by_id(log_id)
            .one(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to find schedule job log for deletion: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to find schedule job log")
            })?
            .ok_or_else(|| {
                AppError::new(ErrorCode::NotFound, "Schedule job log not found")
            })?;

        let active_model = log.into_active_model();
        active_model.delete(&self.db).await.map_err(|e| {
            error!("Failed to delete schedule job log: {:?}", e);
            AppError::new(ErrorCode::DatabaseError, "Failed to delete schedule job log")
        })?;

        Ok(())
    }

    /// 批量删除任务执行日志
    pub async fn delete_schedule_job_logs_batch(&self, log_ids: &[i64]) -> Result<usize, AppError> {
        if log_ids.is_empty() {
            return Ok(0);
        }

        let mut deleted_count = 0;
        for log_id in log_ids {
            match schedule_job_log::Entity::find_by_id(*log_id)
                .one(&self.db)
                .await
            {
                Ok(Some(log)) => {
                    let active_model = log.into_active_model();
                    if let Err(e) = active_model.delete(&self.db).await {
                        error!("Failed to delete schedule job log {}: {:?}", log_id, e);
                    } else {
                        deleted_count += 1;
                    }
                }
                Ok(None) => {
                    warn!("Schedule job log {} not found for deletion", log_id);
                }
                Err(e) => {
                    error!("Failed to find schedule job log {} for deletion: {:?}", log_id, e);
                }
            }
        }

        Ok(deleted_count)
    }

    /// 清空任务执行日志
    pub async fn clear_schedule_job_logs(&self) -> Result<usize, AppError> {
        let logs = schedule_job_log::Entity::find()
            .all(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to find schedule job logs for clearing: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to find schedule job logs")
            })?;

        let mut deleted_count = 0;
        for log in logs {
            let active_model = log.into_active_model();
            if let Err(e) = active_model.delete(&self.db).await {
                error!("Failed to delete schedule job log during clear: {:?}", e);
            } else {
                deleted_count += 1;
            }
        }

        Ok(deleted_count)
    }
}
