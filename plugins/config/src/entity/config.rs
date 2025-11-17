//! 系统配置实体
//! 对应数据库表：sys_config

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// 系统配置实体模型
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "sys_config")]
pub struct Model {
    /// 主键ID
    #[sea_orm(primary_key)]
    pub id: i64,
    
    /// 配置名称
    #[sea_orm(column_type = "String(StringLen::N(64))")]
    pub name: String,
    
    /// 配置键（唯一）
    #[sea_orm(column_type = "String(StringLen::N(64))", unique)]
    pub key: String,
    
    /// 配置值
    #[sea_orm(column_type = "Text")]
    pub value: String,
    
    /// 配置类型（text/number/boolean等）
    #[sea_orm(column_name = "type", column_type = "String(StringLen::N(32))", nullable)]
    pub config_type: Option<String>,
    
    /// 是否前端可见
    pub is_frontend: bool,
    
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
pub enum Relation {}

/// ActiveModel行为实现
impl ActiveModelBehavior for ActiveModel {}

/// 查询辅助方法
impl Entity {
    /// 根据键查找配置
    pub fn find_by_key(key: &str) -> Select<Self> {
        Self::find().filter(Column::Key.eq(key))
    }
    
    /// 查找前端可见的配置
    pub fn find_frontend_visible() -> Select<Self> {
        Self::find().filter(Column::IsFrontend.eq(true))
    }
}

impl Model {
    /// 是否前端可见
    pub fn is_frontend_visible(&self) -> bool {
        self.is_frontend
    }
}
