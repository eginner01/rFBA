//! 字典类型实体
//! 对应数据库表：sys_dict_type

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// 字典类型实体模型
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "sys_dict_type")]
pub struct Model {
    /// 主键ID
    #[sea_orm(primary_key)]
    pub id: i64,
    
    /// 字典名称
    #[sea_orm(column_type = "String(StringLen::N(32))")]
    pub name: String,
    
    /// 字典编码（唯一）
    #[sea_orm(column_type = "String(StringLen::N(32))", unique)]
    pub code: String,
    
    /// 状态（1启用/0禁用）
    pub status: i16,
    
    /// 备注
    #[sea_orm(column_type = "Text", nullable)]
    pub remark: Option<String>,
    
    /// 创建时间
    pub created_time: DateTime,
    
    /// 更新时间
    #[sea_orm(nullable)]
    pub updated_time: Option<DateTime>,
}

/// 关系定义
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    /// 一对多关系：一个字典类型包含多个字典数据
    #[sea_orm(has_many = "super::dict_data::Entity")]
    DictData,
}

/// 实现与dict_data的关联
impl Related<super::dict_data::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DictData.def()
    }
}

/// ActiveModel行为实现
impl ActiveModelBehavior for ActiveModel {}

/// 查询辅助方法
impl Entity {
    /// 根据编码查找字典类型
    pub fn find_by_code(code: &str) -> Select<Self> {
        Self::find().filter(Column::Code.eq(code))
    }
    
    /// 查找启用的字典类型
    pub fn find_enabled() -> Select<Self> {
        Self::find().filter(Column::Status.eq(1))
    }
}

impl Model {
    /// 是否启用
    pub fn is_enabled(&self) -> bool {
        self.status == 1
    }
}
