//! 角色权限关联实体 - sys_role_permission表

use sea_orm::prelude::*;
use sea_orm::{EnumIter, DeriveRelation, ActiveModelBehavior};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "sys_role_permission")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub role_id: i64,
    pub permission_id: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = "super::role::Entity", from = "super::role_id", to = "super::id")]
    Role,
    #[sea_orm(belongs_to = "super::permission::Entity", from = "super::permission_id", to = "super::id")]
    Permission,
}

impl Related<super::role::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Role.def()
    }
}

impl Related<super::permission::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Permission.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
