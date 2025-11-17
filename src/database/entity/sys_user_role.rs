/// 用户角色关联实体
/// 对应 sys_user_role 表

use sea_orm::prelude::*;
use sea_orm::{EnumIter, DeriveRelation, ActiveModelBehavior};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "sys_user_role")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    /// 用户ID
    pub user_id: i64,
    /// 角色ID
    pub role_id: i64,
    /// 创建时间
    pub created_time: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = "super::user::Entity", from = "super::user_id", to = "super::id")]
    User,
    #[sea_orm(belongs_to = "super::role::Entity", from = "super::role_id", to = "super::id")]
    Role,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::role::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Role.def()
    }
}

impl ActiveModelBehavior for ActiveModel {
    // 可以在此处执行自定义逻辑
    // 在保存前设置默认值等操作
}
