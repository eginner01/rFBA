//! 字典数据实体
//! 对应数据库表：sys_dict_data

use sea_orm::entity::prelude::*;
use sea_orm::QueryOrder;
use serde::{Deserialize, Serialize};

/// 字典数据实体模型
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "sys_dict_data")]
pub struct Model {
    /// 主键ID
    #[sea_orm(primary_key)]
    pub id: i64,
    
    /// 显示标签
    #[sea_orm(column_type = "String(StringLen::N(64))")]
    pub label: String,
    
    /// 数据值
    #[sea_orm(column_type = "String(StringLen::N(64))")]
    pub value: String,
    
    /// 排序号
    pub sort: i32,
    
    /// 所属字典类型ID
    pub type_id: i64,
    
    /// 所属字典类型编码
    #[sea_orm(column_type = "String(StringLen::N(32))")]
    pub type_code: String,
    
    /// 是否默认（Y/N）
    #[sea_orm(column_type = "String(StringLen::N(1))")]
    pub is_default: String,
    
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
    /// 多对一关系：多个字典数据属于一个字典类型
    #[sea_orm(
        belongs_to = "super::dict_type::Entity",
        from = "Column::TypeId",
        to = "super::dict_type::Column::Id"
    )]
    DictType,
}

/// 实现与dict_type的关联
impl Related<super::dict_type::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DictType.def()
    }
}

/// ActiveModel行为实现
impl ActiveModelBehavior for ActiveModel {}

/// 查询辅助方法
impl Entity {
    /// 根据类型编码查找字典数据
    pub fn find_by_type_code(type_code: &str) -> Select<Self> {
        Self::find()
            .filter(Column::TypeCode.eq(type_code))
            .order_by_asc(Column::Sort)
    }
    
    /// 根据类型ID查找字典数据
    pub fn find_by_type_id(type_id: i64) -> Select<Self> {
        Self::find()
            .filter(Column::TypeId.eq(type_id))
            .order_by_asc(Column::Sort)
    }
    
    /// 查找启用的字典数据
    pub fn find_enabled() -> Select<Self> {
        Self::find()
            .filter(Column::Status.eq(1))
            .order_by_asc(Column::Sort)
    }
}

impl Model {
    /// 是否启用
    pub fn is_enabled(&self) -> bool {
        self.status == 1
    }
    
    /// 是否默认值
    pub fn is_default_value(&self) -> bool {
        self.is_default.to_uppercase() == "Y"
    }
}
