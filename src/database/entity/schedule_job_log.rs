/// 任务执行日志实体模型
/// 对应数据库中的 sys_schedule_job_log 表

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "sys_schedule_job_log")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    /// 任务ID
    pub job_id: i64,
    /// 任务名称
    pub job_name: String,
    /// 任务组名
    pub job_group: String,
    /// 任务执行类
    pub bean_name: String,
    /// 任务执行方法
    pub method_name: String,
    /// 任务参数
    pub method_params: Option<String>,
    /// 执行状态（0: 成功, 1: 失败）
    pub status: i32,
    /// 异常信息
    pub exception: Option<String>,
    /// 异常信息详情
    pub exception_detail: Option<String>,
    /// 执行耗时（毫秒）
    pub cost_time: i64,
    /// 执行时间
    pub execute_time: DateTime,
    /// 结束时间
    pub end_time: DateTime,
    /// 开始时间
    pub start_time: DateTime,
    /// 任务参数
    pub job_params: Option<String>,
    /// 执行机器IP
    pub machine_ip: Option<String>,
    /// 执行机器名
    pub machine_name: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
#[sea_orm()]
pub enum Relation {
    /// 关联任务
    ScheduleJob,
}

impl Related<super::schedule_job::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ScheduleJob.def()
    }
}

impl ActiveModelBehavior for ActiveModel {
    fn before_save(c: &mut sea_orm::query::Iterable, insert: bool) {
        let now = chrono::Utc::now().naive_utc();
        if insert {
            if c.get("execute_time").is_none() {
                c.set("execute_time", now);
            }
        }
    }
}

/// 任务执行状态
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JobLogStatus {
    /// 成功
    Success = 0,
    /// 失败
    Failure = 1,
}

impl JobLogStatus {
    /// 从i32值创建JobLogStatus
    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            0 => Some(JobLogStatus::Success),
            1 => Some(JobLogStatus::Failure),
            _ => None,
        }
    }

    /// 获取状态名称
    pub fn get_name(&self) -> &'static str {
        match self {
            JobLogStatus::Success => "成功",
            JobLogStatus::Failure => "失败",
        }
    }

    /// 获取状态颜色
    pub fn get_color(&self) -> &'static str {
        match self {
            JobLogStatus::Success => "green",
            JobLogStatus::Failure => "red",
        }
    }
}
