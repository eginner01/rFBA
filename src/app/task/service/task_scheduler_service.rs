/// 任务调度器服务实现
/// 提供任务调度器CRUD、状态管理、执行等功能

use tracing::{info, error};

use crate::app::task::dto::{
    CreateTaskSchedulerRequest, CreateTaskSchedulerResponse,
    UpdateTaskSchedulerRequest, UpdateTaskSchedulerResponse,
    TaskSchedulerDetailResponse, TaskSchedulerListItem,
    TaskSchedulerListQuery,
};
use crate::common::exception::{AppError, ErrorCode};
use crate::database::entity::task_scheduler;
use sea_orm::{
    DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, ColumnTrait,
    ActiveValue, ActiveModelTrait, PaginatorTrait, Order,
};
use chrono::Utc;

/// 任务调度器服务
pub struct TaskSchedulerService {
    db: DatabaseConnection,
}

impl TaskSchedulerService {
    /// 创建新的任务调度器服务
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// 获取所有任务调度器
    pub async fn get_all(&self) -> Result<Vec<TaskSchedulerDetailResponse>, AppError> {
        let schedulers = task_scheduler::Entity::find()
            .order_by(task_scheduler::Column::Id, Order::Desc)
            .all(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to query task schedulers: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, "Failed to query task schedulers")
            })?;

        let result = schedulers
            .iter()
            .map(TaskSchedulerDetailResponse::from_model)
            .collect();

        Ok(result)
    }

    /// 获取任务调度器详情
    pub async fn get_by_id(&self, id: i64) -> Result<TaskSchedulerDetailResponse, AppError> {
        let scheduler = task_scheduler::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to find task scheduler: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, "Failed to find task scheduler")
            })?
            .ok_or_else(|| AppError::with_message(ErrorCode::NotFound, "Task scheduler not found"))?;

        Ok(TaskSchedulerDetailResponse::from_model(&scheduler))
    }

    /// 获取任务调度器列表（分页）
    pub async fn get_list(&self, query: &TaskSchedulerListQuery) -> Result<(Vec<TaskSchedulerListItem>, u64), AppError> {
        let mut select = task_scheduler::Entity::find();

        // 按名称筛选
        if let Some(ref name) = query.name {
            select = select.filter(task_scheduler::Column::Name.like(format!("%{}%", name)));
        }

        // 按调度类型筛选
        if let Some(scheduler_type) = query.scheduler_type {
            select = select.filter(task_scheduler::Column::SchedulerType.eq(scheduler_type));
        }

        // 按启用状态筛选
        if let Some(enabled) = query.enabled {
            select = select.filter(task_scheduler::Column::Enabled.eq(enabled));
        }

        // 计算总数
        let total = select
            .clone()
            .count(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to count task schedulers: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, "Failed to count task schedulers")
            })?;

        // 获取数据
        let schedulers = select
            .order_by(task_scheduler::Column::Id, Order::Desc)
            .all(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to query task schedulers: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, "Failed to query task schedulers")
            })?;

        let result = schedulers
            .iter()
            .map(TaskSchedulerListItem::from_model)
            .collect();

        Ok((result, total))
    }

    /// 创建任务调度器
    pub async fn create(&self, request: &CreateTaskSchedulerRequest) -> Result<CreateTaskSchedulerResponse, AppError> {
        // 检查 expire_seconds 和 expire_time 是否冲突
        if request.expire_seconds.is_some() && request.expire_time.is_some() {
            return Err(AppError::with_message(
                ErrorCode::BadRequest,
                "expire_seconds and expire_time cannot both be set"
            ));
        }

        let active_model = task_scheduler::ActiveModel {
            id: ActiveValue::NotSet,
            name: ActiveValue::Set(request.name.clone()),
            task: ActiveValue::Set(request.task.clone()),
            args: ActiveValue::Set(request.args.clone()),
            kwargs: ActiveValue::Set(request.kwargs.clone()),
            queue: ActiveValue::Set(request.queue.clone()),
            exchange: ActiveValue::Set(request.exchange.clone()),
            routing_key: ActiveValue::Set(request.routing_key.clone()),
            start_time: ActiveValue::Set(request.start_time),
            expire_time: ActiveValue::Set(request.expire_time),
            expire_seconds: ActiveValue::Set(request.expire_seconds),
            scheduler_type: ActiveValue::Set(request.scheduler_type),
            interval_every: ActiveValue::Set(request.interval_every),
            interval_period: ActiveValue::Set(request.interval_period.clone()),
            crontab: ActiveValue::Set(request.crontab.clone()),
            one_off: ActiveValue::Set(request.one_off.unwrap_or(false)),
            enabled: ActiveValue::Set(true),
            total_run_count: ActiveValue::Set(0),
            last_run_time: ActiveValue::NotSet,
            remark: ActiveValue::Set(request.remark.clone()),
            created_time: ActiveValue::NotSet,
            updated_time: ActiveValue::NotSet,
        };

        let saved_scheduler = active_model.insert(&self.db).await.map_err(|e| {
            error!("Failed to create task scheduler: {:?}", e);
            AppError::with_message(ErrorCode::DatabaseError, "Failed to create task scheduler")
        })?;

        info!("Created task scheduler: {} (id: {})", saved_scheduler.name, saved_scheduler.id);

        Ok(CreateTaskSchedulerResponse {
            id: saved_scheduler.id,
            name: saved_scheduler.name,
            created_time: saved_scheduler.created_time,
        })
    }

    /// 更新任务调度器
    pub async fn update(&self, id: i64, request: &UpdateTaskSchedulerRequest) -> Result<UpdateTaskSchedulerResponse, AppError> {
        // 检查是否存在
        let existing = task_scheduler::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to find task scheduler: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, "Failed to find task scheduler")
            })?
            .ok_or_else(|| AppError::with_message(ErrorCode::NotFound, "Task scheduler not found"))?;

        // 检查 expire_seconds 和 expire_time 是否冲突
        if request.expire_seconds.is_some() && request.expire_time.is_some() {
            return Err(AppError::with_message(
                ErrorCode::BadRequest,
                "expire_seconds and expire_time cannot both be set"
            ));
        }

        let active_model = task_scheduler::ActiveModel {
            id: ActiveValue::Set(id),
            name: ActiveValue::Set(request.name.clone()),
            task: ActiveValue::Set(request.task.clone()),
            args: ActiveValue::Set(request.args.clone()),
            kwargs: ActiveValue::Set(request.kwargs.clone()),
            queue: ActiveValue::Set(request.queue.clone()),
            exchange: ActiveValue::Set(request.exchange.clone()),
            routing_key: ActiveValue::Set(request.routing_key.clone()),
            start_time: ActiveValue::Set(request.start_time),
            expire_time: ActiveValue::Set(request.expire_time),
            expire_seconds: ActiveValue::Set(request.expire_seconds),
            scheduler_type: ActiveValue::Set(request.scheduler_type),
            interval_every: ActiveValue::Set(request.interval_every),
            interval_period: ActiveValue::Set(request.interval_period.clone()),
            crontab: ActiveValue::Set(request.crontab.clone()),
            one_off: ActiveValue::Set(request.one_off.unwrap_or(false)),
            enabled: ActiveValue::Set(existing.enabled),
            total_run_count: ActiveValue::Set(existing.total_run_count),
            last_run_time: ActiveValue::Set(existing.last_run_time),
            remark: ActiveValue::Set(request.remark.clone()),
            created_time: ActiveValue::Set(existing.created_time),
            updated_time: ActiveValue::Set(Some(Utc::now())),
        };

        let updated_scheduler = active_model.update(&self.db).await.map_err(|e| {
            error!("Failed to update task scheduler: {:?}", e);
            AppError::with_message(ErrorCode::DatabaseError, "Failed to update task scheduler")
        })?;

        info!("Updated task scheduler: {} (id: {})", updated_scheduler.name, updated_scheduler.id);

        Ok(UpdateTaskSchedulerResponse {
            id: updated_scheduler.id,
            name: updated_scheduler.name,
            updated_time: updated_scheduler.updated_time.unwrap_or_else(Utc::now),
        })
    }

    /// 更新任务调度器状态
    pub async fn update_status(&self, id: i64) -> Result<(), AppError> {
        let scheduler = task_scheduler::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to find task scheduler: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, "Failed to find task scheduler")
            })?
            .ok_or_else(|| AppError::with_message(ErrorCode::NotFound, "Task scheduler not found"))?;

        let new_enabled = !scheduler.enabled;

        let active_model = task_scheduler::ActiveModel {
            id: ActiveValue::Set(scheduler.id),
            name: ActiveValue::Set(scheduler.name),
            task: ActiveValue::Set(scheduler.task),
            args: ActiveValue::Set(scheduler.args),
            kwargs: ActiveValue::Set(scheduler.kwargs),
            queue: ActiveValue::Set(scheduler.queue),
            exchange: ActiveValue::Set(scheduler.exchange),
            routing_key: ActiveValue::Set(scheduler.routing_key),
            start_time: ActiveValue::Set(scheduler.start_time),
            expire_time: ActiveValue::Set(scheduler.expire_time),
            expire_seconds: ActiveValue::Set(scheduler.expire_seconds),
            scheduler_type: ActiveValue::Set(scheduler.scheduler_type),
            interval_every: ActiveValue::Set(scheduler.interval_every),
            interval_period: ActiveValue::Set(scheduler.interval_period),
            crontab: ActiveValue::Set(scheduler.crontab),
            one_off: ActiveValue::Set(scheduler.one_off),
            enabled: ActiveValue::Set(new_enabled),
            total_run_count: ActiveValue::Set(scheduler.total_run_count),
            last_run_time: ActiveValue::Set(scheduler.last_run_time),
            remark: ActiveValue::Set(scheduler.remark),
            created_time: ActiveValue::Set(scheduler.created_time),
            updated_time: ActiveValue::Set(Some(Utc::now())),
        };

        active_model.update(&self.db).await.map_err(|e| {
            error!("Failed to update task scheduler status: {:?}", e);
            AppError::with_message(ErrorCode::DatabaseError, "Failed to update task scheduler status")
        })?;

        info!("Updated task scheduler status: {} to {}", id, new_enabled);

        Ok(())
    }

    /// 删除任务调度器
    pub async fn delete(&self, id: i64) -> Result<(), AppError> {
        let scheduler = task_scheduler::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to find task scheduler: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, "Failed to find task scheduler")
            })?
            .ok_or_else(|| AppError::with_message(ErrorCode::NotFound, "Task scheduler not found"))?;

        task_scheduler::Entity::delete_by_id(id)
            .exec(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to delete task scheduler: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, "Failed to delete task scheduler")
            })?;

        info!("Deleted task scheduler: {}", id);

        Ok(())
    }

    /// 手动执行任务
    pub async fn execute(&self, id: i64) -> Result<(), AppError> {
        let scheduler = task_scheduler::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to find task scheduler: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, "Failed to find task scheduler")
            })?
            .ok_or_else(|| AppError::with_message(ErrorCode::NotFound, "Task scheduler not found"))?;

        if !scheduler.enabled {
            return Err(AppError::with_message(
                ErrorCode::BadRequest,
                "Task scheduler is disabled"
            ));
        }

        // TODO: 这里应该调用实际的 Celery 任务执行
        // 由于当前没有集成 Celery，这里只是模拟执行
        info!("Executing task scheduler: {} - {}", scheduler.name, scheduler.task);

        // 更新运行计数和最后运行时间
        let active_model = task_scheduler::ActiveModel {
            id: ActiveValue::Set(scheduler.id),
            name: ActiveValue::Set(scheduler.name),
            task: ActiveValue::Set(scheduler.task),
            args: ActiveValue::Set(scheduler.args),
            kwargs: ActiveValue::Set(scheduler.kwargs),
            queue: ActiveValue::Set(scheduler.queue),
            exchange: ActiveValue::Set(scheduler.exchange),
            routing_key: ActiveValue::Set(scheduler.routing_key),
            start_time: ActiveValue::Set(scheduler.start_time),
            expire_time: ActiveValue::Set(scheduler.expire_time),
            expire_seconds: ActiveValue::Set(scheduler.expire_seconds),
            scheduler_type: ActiveValue::Set(scheduler.scheduler_type),
            interval_every: ActiveValue::Set(scheduler.interval_every),
            interval_period: ActiveValue::Set(scheduler.interval_period),
            crontab: ActiveValue::Set(scheduler.crontab),
            one_off: ActiveValue::Set(scheduler.one_off),
            enabled: ActiveValue::Set(scheduler.enabled),
            total_run_count: ActiveValue::Set(scheduler.total_run_count + 1),
            last_run_time: ActiveValue::Set(Some(Utc::now())),
            remark: ActiveValue::Set(scheduler.remark),
            created_time: ActiveValue::Set(scheduler.created_time),
            updated_time: ActiveValue::Set(Some(Utc::now())),
        };

        active_model.update(&self.db).await.map_err(|e| {
            error!("Failed to update task scheduler execution info: {:?}", e);
            AppError::with_message(ErrorCode::DatabaseError, "Failed to update task scheduler execution info")
        })?;

        Ok(())
    }
}
