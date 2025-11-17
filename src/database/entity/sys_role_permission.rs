/// 角色权限关联实体
/// 对应 sys_role_permission 表

use sea_orm::prelude::*;
use sea_orm::{EnumIter, DeriveRelation, ActiveModelBehavior};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "sys_role_permission")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    /// 角色ID
    pub role_id: i64,
    /// 权限ID
    pub permission_id: i64,
    /// 创建时间
    pub created_time: DateTime,
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

impl ActiveModelBehavior for ActiveModel {
    // 可以在此处执行自定义逻辑
    // 在保存前设置默认值等操作
}
