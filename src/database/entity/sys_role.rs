/// 系统角色实体
/// 对应 sys_role 表

use sea_orm::prelude::*;
use sea_orm::{EnumIter, DeriveRelation, ActiveModelBehavior};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "sys_role")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    /// 角色名称
    pub name: String,
    /// 角色编码
    pub code: String,
    /// 角色描述
    pub description: Option<String>,
    /// 排序
    pub sort: i32,
    /// 角色状态 (1: 正常 0: 禁用)
    pub status: i32,
    /// 是否系统角色 (1: 是 0: 否)
    pub is_system: i32,
    /// 是否删除 (1: 已删除 0: 未删除)
    pub del_flag: i32,
    /// 创建时间
    pub created_time: DateTime,
    /// 更新时间
    pub updated_time: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::user_role::Model")]
    UserRoles,
    #[sea_orm(has_many = "super::role_permission::Model")]
    RolePermissions,
}

impl Related<super::user_role::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserRoles.def()
    }
}

impl Related<super::role_permission::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RolePermissions.def()
    }
}

impl ActiveModelBehavior for ActiveModel {
    // 可以在此处执行自定义逻辑
    // 在保存前设置默认值等操作
}
