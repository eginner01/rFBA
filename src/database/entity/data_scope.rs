/// 数据权限实体模型
/// 对应数据库中的 sys_data_scope 表

use sea_orm::prelude::*;
use sea_orm::EnumIter;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "sys_data_scope")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    /// 名称
    pub name: String,
    /// 状态（0停用 1正常）
    pub status: i32,
}

/// Empty Relation enum (we'll implement relations manually later)
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

// 使用默认实现
impl ActiveModelBehavior for ActiveModel {}
// impl ActiveModelBehavior for ActiveModel {}

/// 数据权限类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataScopeType {
    /// 全部数据（所有数据）
    AllData = 1,
    /// 自定义数据（按custom_data字段指定）
    CustomData = 2,
    /// 本部门数据
    DeptData = 3,
    /// 本部门及以下数据
    DeptAndSubData = 4,
    /// 仅本人数据
    SelfData = 5,
}

impl DataScopeType {
    /// 从i32值创建DataScopeType
    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            1 => Some(DataScopeType::AllData),
            2 => Some(DataScopeType::CustomData),
            3 => Some(DataScopeType::DeptData),
            4 => Some(DataScopeType::DeptAndSubData),
            5 => Some(DataScopeType::SelfData),
            _ => None,
        }
    }

    /// 获取类型名称
    pub fn get_name(&self) -> &'static str {
        match self {
            DataScopeType::AllData => "全部数据",
            DataScopeType::CustomData => "自定义数据",
            DataScopeType::DeptData => "本部门数据",
            DataScopeType::DeptAndSubData => "本部门及以下数据",
            DataScopeType::SelfData => "仅本人数据",
        }
    }

    /// 获取类型描述
    pub fn get_description(&self) -> &'static str {
        match self {
            DataScopeType::AllData => "拥有所有数据的访问权限",
            DataScopeType::CustomData => "拥有指定部门的数据访问权限",
            DataScopeType::DeptData => "拥有本部门的数据访问权限",
            DataScopeType::DeptAndSubData => "拥有本部门及下属部门的数据访问权限",
            DataScopeType::SelfData => "仅拥有本人的数据访问权限",
        }
    }
}
