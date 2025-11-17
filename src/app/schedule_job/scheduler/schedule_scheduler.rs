/// 任务调度器
/// 负责任务的定时调度和执行

use crate::common::exception::{AppError, ErrorCode};
use crate::database::DatabaseManager;
use crate::database::entity::schedule_job;
use crate::app::schedule_job::executor::JobExecutor;
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait};
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;

/// 任务调度器
pub struct ScheduleScheduler {
    /// 数据库连接
    db: DatabaseConnection,
    /// 任务执行器
    executor: Arc<JobExecutor>,
    /// 定时任务列表
    scheduled_jobs: Arc<RwLock<HashMap<i64, ScheduledJob>>>,
    /// 是否运行中
    is_running: Arc<RwLock<bool>>,
}

/// 定时任务信息
#[derive(Debug, Clone)]
struct ScheduledJob {
    /// 任务ID
    job_id: i64,
    /// 任务名称
    job_name: String,
    /// 任务组名
    job_group: String,
    /// Cron表达式
    cron_expression: String,
    /// 下次执行时间
    next_execution: chrono::NaiveDateTime,
    /// 任务描述
    description: Option<String>,
}

impl ScheduleScheduler {
    /// 创建新的任务调度器
    pub fn new(db: DatabaseConnection) -> Self {
        let executor = Arc::new(JobExecutor::new(db.clone()));
        Self {
            db,
            executor,
            scheduled_jobs: Arc::new(RwLock::new(HashMap::new())),
            is_running: Arc::new(RwLock::new(false)),
        }
    }

    /// 启动调度器
    pub async fn start(&self) -> Result<(), AppError> {
        let mut is_running = self.is_running.write().await;
        if *is_running {
            warn!("Schedule scheduler is already running");
            return Ok(());
        }

        info!("Starting schedule scheduler...");

        // 加载所有正常状态的任务
        self.load_jobs_from_database().await?;

        // 启动调度循环
        *is_running = true;
        self.spawn_scheduler_loop().await;

        info!("Schedule scheduler started successfully");
        Ok(())
    }

    /// 停止调度器
    pub async fn stop(&self) -> Result<(), AppError> {
        let mut is_running = self.is_running.write().await;
        if !*is_running {
            warn!("Schedule scheduler is not running");
            return Ok(());
        }

        info!("Stopping schedule scheduler...");

        *is_running = false;
        self.scheduled_jobs.write().await.clear();

        info!("Schedule scheduler stopped");
        Ok(())
    }

    /// 添加任务
    pub async fn add_job(
        &self,
        job_id: i64,
        job_name: String,
        job_group: String,
        cron_expression: String,
        description: Option<String>,
    ) -> Result<(), AppError> {
        let next_execution = self.calculate_next_execution(&cron_expression)?;

        let scheduled_job = ScheduledJob {
            job_id,
            job_name: job_name.clone(),
            job_group,
            cron_expression,
            next_execution,
            description,
        };

        self.scheduled_jobs
            .write()
            .await
            .insert(job_id, scheduled_job);

        info!("Added scheduled job: {} (ID: {})", job_name, job_id);
        Ok(())
    }

    /// 移除任务
    pub async fn remove_job(&self, job_id: i64) -> Result<(), AppError> {
        let mut scheduled_jobs = self.scheduled_jobs.write().await;
        if let Some(job) = scheduled_jobs.remove(&job_id) {
            info!("Removed scheduled job: {} (ID: {})", job.job_name, job_id);
        }
        Ok(())
    }

    /// 立即执行任务
    pub async fn execute_job_immediately(
        &self,
        job_id: i64,
    ) -> Result<(), AppError> {
        let job = self.get_job_from_db(job_id).await?;
        if let Some(job) = job {
            self.executor
                .execute_job(
                    job.id,
                    job.job_name,
                    job.job_group,
                    job.bean_name,
                    job.method_name,
                    job.method_params,
                )
                .await?;
        }
        Ok(())
    }

    /// 从数据库加载所有任务
    async fn load_jobs_from_database(&self) -> Result<(), AppError> {
        let jobs = schedule_job::Entity::find()
            .filter(schedule_job::Column::Status.eq(0))
            .all(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to load jobs from database: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to load jobs from database")
            })?;

        let mut scheduled_jobs = self.scheduled_jobs.write().await;
        scheduled_jobs.clear();

        for job in jobs {
            let next_execution = self.calculate_next_execution(&job.cron_expression)?;
            let scheduled_job = ScheduledJob {
                job_id: job.id,
                job_name: job.job_name,
                job_group: job.job_group,
                cron_expression: job.cron_expression,
                next_execution,
                description: job.description,
            };
            scheduled_jobs.insert(job.id, scheduled_job);
        }

        info!("Loaded {} jobs from database", scheduled_jobs.len());
        Ok(())
    }

    /// 获取数据库中的任务
    async fn get_job_from_db(&self, job_id: i64) -> Result<Option<schedule_job::Model>, AppError> {
        let job = schedule_job::Entity::find_by_id(job_id)
            .one(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to find job: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to find job")
            })?;

        Ok(job)
    }

    /// 计算下次执行时间
    fn calculate_next_execution(
        &self,
        cron_expression: &str,
    ) -> Result<chrono::NaiveDateTime, AppError> {
        // TODO: 实现 Cron 表达式解析
        // 目前使用简单的逻辑：每分钟执行一次
        Ok(chrono::Utc::now().naive_utc() + chrono::Duration::minutes(1))
    }

    /// 启动调度循环
    async fn spawn_scheduler_loop(&self) {
        let db = self.db.clone();
        let executor = self.executor.clone();
        let scheduled_jobs = self.scheduled_jobs.clone();
        let is_running = self.is_running.clone();

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(1));

            loop {
                interval.tick().await;

                let running = *is_running.read().await;
                if !running {
                    break;
                }

                let now = chrono::Utc::now().naive_utc();

                // 查找需要执行的任务
                let jobs_to_execute = {
                    let scheduled_jobs = scheduled_jobs.read().await;
                    scheduled_jobs
                        .values()
                        .filter(|job| job.next_execution <= now)
                        .cloned()
                        .collect::<Vec<_>>()
                };

                // 执行任务
                for job in jobs_to_execute {
                    let job_db = schedule_job::Entity::find_by_id(job.job_id)
                        .one(&db)
                        .await;

                    if let Ok(Some(job_model)) = job_db {
                        if job_model.status == 0 {
                            // 异步执行任务
                            let executor_clone = executor.clone();
                            let job_name = job.job_name.clone();
                            let job_group = job.job_group.clone();
                            let bean_name = job_model.bean_name.clone();
                            let method_name = job_model.method_name.clone();
                            let method_params = job_model.method_params.clone();

                            tokio::spawn(async move {
                                if let Err(e) = executor_clone
                                    .execute_job(
                                        job_model.id,
                                        job_name,
                                        job_group,
                                        bean_name,
                                        method_name,
                                        method_params,
                                    )
                                    .await
                                {
                                    error!("Failed to execute job {}: {:?}", job_model.id, e);
                                }
                            });
                        }
                    }

                    // 计算下次执行时间
                    let next_execution =
                        Self::calculate_next_execution_static(&job.cron_expression);

                    // 更新下次执行时间
                    {
                        let mut scheduled_jobs = scheduled_jobs.write().await;
                        if let Some(entry) = scheduled_jobs.get_mut(&job.job_id) {
                            entry.next_execution = next_execution;
                        }
                    }
                }
            }
        });
    }

    /// 静态方法：计算下次执行时间
    fn calculate_next_execution_static(
        cron_expression: &str,
    ) -> chrono::NaiveDateTime {
        // TODO: 实现 Cron 表达式解析
        chrono::Utc::now().naive_utc() + chrono::Duration::minutes(1)
    }

    /// 获取所有调度的任务
    pub async fn get_scheduled_jobs(&self) -> HashMap<i64, ScheduledJob> {
        self.scheduled_jobs.read().await.clone()
    }

    /// 检查调度器是否运行中
    pub async fn is_running(&self) -> bool {
        *self.is_running.read().await
    }
}
