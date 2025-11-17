/// 系统权限实体
/// 对应 sys_permission 表

use sea_orm::prelude::*;
use sea_orm::{EnumIter, DeriveRelation, ActiveModelBehavior};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "sys_permission")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    /// 父权限ID
    pub parent_id: Option<i64>,
    /// 权限名称
    pub name: String,
    /// 权限编码
    pub code: String,
    /// 权限类型 (1: 目录 2: 菜单 3: 按钮)
    pub type_: i32,
    /// 路由路径
    pub path: Option<String>,
    /// 组件路径
    pub component: Option<String>,
    /// 图标
    pub icon: Option<String>,
    /// 排序
    pub sort: i32,
    /// 权限状态 (1: 正常 0: 禁用)
    pub status: i32,
    /// 是否外链 (1: 是 0: 否)
    pub is_external: i32,
    /// 是否缓存 (1: 是 0: 否)
    pub is_cache: i32,
    /// 是否删除 (1: 已删除 0: 未删除)
    pub del_flag: i32,
    /// 创建时间
    pub created_time: DateTime,
    /// 更新时间
    pub updated_time: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::role_permission::Model")]
    RolePermissions,
    #[sea_orm(has_many = "super::permission::Model")]
    Children,
    #[sea_orm(belongs_to = "super::permission::Entity", from = "super::parent_id", to = "super::id")]
    Parent,
}

impl Related<super::role_permission::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RolePermissions.def()
    }
}

impl Related<super::permission::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Children.def()
    }
}

impl ActiveModelBehavior for ActiveModel {
    // 可以在此处执行自定义逻辑
    // 在保存前设置默认值等操作
}
