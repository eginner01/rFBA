//! 系统用户实体 - sys_user表

use sea_orm::prelude::*;
use sea_orm::{EnumIter, DeriveRelation, ActiveModelBehavior};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "sys_user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub username: String,
    pub password_hash: String,
    pub nickname: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub avatar: Option<String>,
    pub dept_id: Option<i64>,
    pub status: i32,
    pub gender: i32,
    pub is_super: i32,
    pub last_login_time: Option<DateTime>,
    pub last_login_ip: Option<String>,
    pub password_reset_count: i32,
    pub del_flag: i32,
    pub created_time: DateTime,
    pub updated_time: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::user_role::Model")]
    UserRoles,
    #[sea_orm(has_one = "super::dept::Model")]
    Dept,
}

impl Related<super::user_role::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserRoles.def()
    }
}

impl Related<super::dept::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Dept.def()
    }
}

impl ActiveModelBehavior for ActiveModel {
    // 可以在此处执行自定义逻辑
    // 在保存前设置默认值等操作
}
