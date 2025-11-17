//! 菜单实体 - sys_menu表

use sea_orm::prelude::*;
use sea_orm::EnumIter;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "sys_menu")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub title: String,
    pub name: String,
    pub parent_id: Option<i64>,
    pub sort: i32,
    pub path: Option<String>,
    pub component: Option<String>,
    #[sea_orm(column_name = "type")]
    pub menu_type: i32,
    pub perms: Option<String>,
    pub icon: Option<String>,
    pub status: i32,
    pub display: bool,
    pub cache: bool,
    pub link: Option<String>,
    pub remark: Option<String>,
    pub created_time: DateTime,
    pub updated_time: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
