//! 操作日志实体 - sys_opera_log表

use sea_orm::entity::prelude::*;
use sea_orm::{EnumIter, DeriveRelation};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "sys_opera_log")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub trace_id: String,
    pub username: Option<String>,
    pub method: String,
    pub title: String,
    pub path: String,
    pub ip: String,
    pub country: Option<String>,
    pub region: Option<String>,
    pub city: Option<String>,
    pub user_agent: String,
    pub os: Option<String>,
    pub browser: Option<String>,
    pub device: Option<String>,
    pub args: Option<Json>,
    pub status: i32,
    pub code: String,
    pub msg: Option<String>,
    pub cost_time: f32,
    pub opera_time: DateTime,
    pub created_time: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
