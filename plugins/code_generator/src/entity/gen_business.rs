//! 代码生成业务实体

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// 代码生成业务实体模型
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "gen_business")]
pub struct Model {
    /// 主键ID
    #[sea_orm(primary_key)]
    pub id: i64,
    
    /// 应用名称（英文）
    #[sea_orm(column_type = "String(StringLen::N(64))")]
    pub app_name: String,
    
    /// 表名称（英文）
    #[sea_orm(column_type = "String(StringLen::N(256))", unique)]
    pub table_name: String,
    
    /// 文档注释（用于函数/参数文档）
    #[sea_orm(column_type = "String(StringLen::N(256))")]
    pub doc_comment: String,
    
    /// 表描述
    #[sea_orm(column_type = "String(StringLen::N(256))", nullable)]
    pub table_comment: Option<String>,
    
    /// 基础类名（默认为英文表名称）
    #[sea_orm(column_type = "String(StringLen::N(64))", nullable)]
    pub class_name: Option<String>,
    
    /// Schema名称（默认为英文表名称）
    #[sea_orm(column_type = "String(StringLen::N(64))", nullable)]
    pub schema_name: Option<String>,
    
    /// 基础文件名（默认为英文表名称）
    #[sea_orm(column_type = "String(StringLen::N(64))", nullable)]
    pub filename: Option<String>,
    
    /// 是否存在默认时间列
    pub default_datetime_column: bool,
    
    /// 代码生成API版本，默认为v1
    #[sea_orm(column_type = "String(StringLen::N(32))")]
    pub api_version: String,
    
    /// 代码生成路径（默认为app根路径）
    #[sea_orm(column_type = "String(StringLen::N(256))", nullable)]
    pub gen_path: Option<String>,
    
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
    #[sea_orm(has_many = "super::gen_column::Entity")]
    GenColumn,
}

impl Related<super::gen_column::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::GenColumn.def()
    }
}

/// ActiveModel行为实现
impl ActiveModelBehavior for ActiveModel {}

/// 查询辅助方法
impl Entity {
    /// 根据表名查找业务
    pub fn find_by_table_name(table_name: &str) -> Select<Self> {
        Self::find().filter(Column::TableName.eq(table_name))
    }
}
