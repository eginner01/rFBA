//! 定时任务实体 - sys_task表

use sea_orm::prelude::*;
use sea_orm::{EnumIter, ActiveValue};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "sys_task")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub name: String,
    pub type_name: String,
    pub cron_expression: String,
    pub job_class: String,
    pub job_data: Option<String>,
    pub status: i32,
    pub concurrent: i32,
    pub description: Option<String>,
    pub created_time: DateTime,
    pub updated_time: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
