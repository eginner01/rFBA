/// 任务结果实体
/// 对应 task_result 表

use sea_orm::prelude::*;
use sea_orm::{EnumIter, DeriveRelation, ActiveModelBehavior};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "task_result")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    /// 任务 ID
    pub task_id: String,
    /// 执行状态
    pub status: String,
    /// 执行结果
    pub result: Option<serde_json::Value>,
    /// 结束时间
    pub date_done: Option<DateTime<Utc>>,
    /// 错误回溯
    pub traceback: Option<String>,
    /// 任务名称
    pub name: Option<String>,
    /// 任务位置参数
    pub args: Option<Vec<u8>>,
    /// 任务关键字参数
    pub kwargs: Option<Vec<u8>>,
    /// 运行 Worker
    pub worker: Option<String>,
    /// 重试次数
    pub retries: Option<i32>,
    /// 运行队列
    pub queue: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {
    // 可以在此处执行自定义逻辑
    // 在保存前设置默认值等操作
}
