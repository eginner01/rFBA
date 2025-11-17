/// 任务调度器实体
/// 对应 task_scheduler 表

use sea_orm::prelude::*;
use sea_orm::{EnumIter, DeriveRelation, ActiveModelBehavior, Set};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "task_scheduler")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    /// 任务名称
    pub name: String,
    /// 要运行的任务名称
    pub task: String,
    /// 任务可接收的位置参数
    pub args: Option<serde_json::Value>,
    /// 任务可接收的关键字参数
    pub kwargs: Option<serde_json::Value>,
    /// 队列名称
    pub queue: Option<String>,
    /// 低级别 AMQP 路由的交换机
    pub exchange: Option<String>,
    /// 低级别 AMQP 路由的路由密钥
    pub routing_key: Option<String>,
    /// 任务开始触发的时间
    pub start_time: Option<DateTime<Utc>>,
    /// 任务不再触发的截止时间
    pub expire_time: Option<DateTime<Utc>>,
    /// 任务不再触发的秒数时间差
    pub expire_seconds: Option<i32>,
    /// 任务调度类型（0间隔 1定时）- 对应数据库中的 type 字段
    #[sea_orm(column_name = "type")]
    pub scheduler_type: i32,
    /// 任务再次运行前的间隔周期数
    pub interval_every: Option<i32>,
    /// 任务运行之间的周期类型
    pub interval_period: Option<String>,
    /// 运行的 Crontab 表达式
    pub crontab: Option<String>,
    /// 是否仅运行一次
    pub one_off: bool,
    /// 是否启用任务
    pub enabled: bool,
    /// 已运行总次数
    pub total_run_count: i32,
    /// 最后运行时间
    pub last_run_time: Option<DateTime<Utc>>,
    /// 备注
    pub remark: Option<String>,
    /// 创建时间
    pub created_time: DateTime<Utc>,
    /// 更新时间
    pub updated_time: Option<DateTime<Utc>>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    /// 在插入或更新前自动设置时间戳
    async fn before_save<C>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        let now = Utc::now();
        
        // 插入时设置创建时间
        if insert {
            self.created_time = Set(now);
        }
        
        // 插入和更新时都设置更新时间
        self.updated_time = Set(Some(now));
        
        Ok(self)
    }
}
