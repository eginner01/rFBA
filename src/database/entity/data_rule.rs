//! 数据规则实体 - sys_data_rule表

use sea_orm::prelude::*;
use sea_orm::EnumIter;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "sys_data_rule")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    /// 规则名称
    pub name: String,
    /// 模型名称
    pub model: String,
    /// 模型字段名
    pub column: String,
    /// 运算符（0：and、1：or）
    pub operator: i32,
    /// 表达式（0：==、1：!=、2：>、3：>=、4：<、5：<=、6：in、7：not_in）
    pub expression: i32,
    /// 规则值
    pub value: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataRuleType {
    Table = 1,
    Field = 2,
    Row = 3,
}

impl DataRuleType {
    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            1 => Some(DataRuleType::Table),
            2 => Some(DataRuleType::Field),
            3 => Some(DataRuleType::Row),
            _ => None,
        }
    }

    pub fn get_name(&self) -> &'static str {
        match self {
            DataRuleType::Table => "表级权限",
            DataRuleType::Field => "字段级权限",
            DataRuleType::Row => "行级权限",
        }
    }

    pub fn get_description(&self) -> &'static str {
        match self {
            DataRuleType::Table => "控制整个表的访问权限",
            DataRuleType::Field => "控制表中字段的读写权限",
            DataRuleType::Row => "控制表中行的访问权限",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PermissionType {
    Read = 1,
    ReadWrite = 2,
    CreateOnly = 3,
    UpdateOnly = 4,
    DeleteOnly = 5,
}

impl PermissionType {
    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            1 => Some(PermissionType::Read),
            2 => Some(PermissionType::ReadWrite),
            3 => Some(PermissionType::CreateOnly),
            4 => Some(PermissionType::UpdateOnly),
            5 => Some(PermissionType::DeleteOnly),
            _ => None,
        }
    }

    /// 获取权限名称
    pub fn get_name(&self) -> &'static str {
        match self {
            PermissionType::Read => "只读",
            PermissionType::ReadWrite => "读写",
            PermissionType::CreateOnly => "仅创建",
            PermissionType::UpdateOnly => "仅更新",
            PermissionType::DeleteOnly => "仅删除",
        }
    }
}
