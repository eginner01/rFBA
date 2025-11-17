//! 任务调度实体 - sys_schedule_job表

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "sys_schedule_job")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub job_name: String,
    pub job_group: String,
    pub bean_name: String,
    pub method_name: String,
    pub method_params: Option<String>,
    pub cron_expression: String,
    pub misfire_policy: i32,
    pub concurrent: i32,
    pub status: i32,
    pub priority: i32,
    pub timeout: Option<i32>,
    pub retry_count: i32,
    pub retry_interval: i32,
    pub description: Option<String>,
    pub create_by: Option<String>,
    pub update_by: Option<String>,
    pub created_time: DateTime,
    pub updated_time: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
#[sea_orm()]
pub enum Relation {
    ScheduleJobLog,
}

impl Related<super::schedule_job_log::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ScheduleJobLog.def()
    }
}

impl ActiveModelBehavior for ActiveModel {
    fn before_save(c: &mut sea_orm::query::Iterable, insert: bool) {
        let now = chrono::Utc::now().naive_utc();
        if insert {
            if c.get("created_time").is_none() {
                c.set("created_time", now);
            }
        }
        if c.get("updated_time").is_none() {
            c.set("updated_time", now);
        }
    }
}

/// 任务状态
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JobStatus {
    /// 正常
    Normal = 0,
    /// 暂停
    Paused = 1,
}

impl JobStatus {
    /// 从i32值创建JobStatus
    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            0 => Some(JobStatus::Normal),
            1 => Some(JobStatus::Paused),
            _ => None,
        }
    }

    /// 获取状态名称
    pub fn get_name(&self) -> &'static str {
        match self {
            JobStatus::Normal => "正常",
            JobStatus::Paused => "暂停",
        }
    }
}

/// 任务执行策略
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MisfirePolicy {
    /// 默认
    Default = 0,
    /// 立即触发执行
    FireNow = 1,
    /// 触发一次
    DoNothing = 2,
    /// 不触发立即执行
    NextNow = 3,
}

impl MisfirePolicy {
    /// 从i32值创建MisfirePolicy
    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            0 => Some(MisfirePolicy::Default),
            1 => Some(MisfirePolicy::FireNow),
            2 => Some(MisfirePolicy::DoNothing),
            3 => Some(MisfirePolicy::NextNow),
            _ => None,
        }
    }

    /// 获取策略名称
    pub fn get_name(&self) -> &'static str {
        match self {
            MisfirePolicy::Default => "默认",
            MisfirePolicy::FireNow => "立即触发执行",
            MisfirePolicy::DoNothing => "触发一次",
            MisfirePolicy::NextNow => "不触发立即执行",
        }
    }
}
