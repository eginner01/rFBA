//! 系统通知公告实体
//! 对应数据库表：sys_notice

use sea_orm::entity::prelude::*;
use sea_orm::QueryOrder;
use serde::{Deserialize, Serialize};

/// 系统通知公告实体模型
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "sys_notice")]
pub struct Model {
    /// 主键ID
    #[sea_orm(primary_key)]
    pub id: i64,
    
    /// 公告标题
    #[sea_orm(column_type = "String(StringLen::N(64))")]
    pub title: String,
    
    /// 类型（0：通知、1：公告）- 匹配Python版本
    #[sea_orm(column_name = "type")]
    pub type_: i32,
    
    /// 状态（0：隐藏、1：显示）- 匹配Python版本
    pub status: i32,
    
    /// 公告内容（富文本）
    #[sea_orm(column_type = "Text")]
    pub content: String,
}

/// 关系定义
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

/// ActiveModel行为实现
impl ActiveModelBehavior for ActiveModel {}

/// 查询辅助方法
impl Entity {
    /// 查找显示的通知公告
    pub fn find_visible() -> Select<Self> {
        Self::find()
            .filter(Column::Status.eq(1))
            .order_by_desc(Column::Id)
    }
    
    /// 查找隐藏的通知
    pub fn find_hidden() -> Select<Self> {
        Self::find().filter(Column::Status.eq(0))
    }
}

impl Model {
    /// 是否显示
    pub fn is_visible(&self) -> bool {
        self.status == 1
    }
    
    /// 是否隐藏
    pub fn is_hidden(&self) -> bool {
        self.status == 0
    }
    
    /// 获取状态文本
    pub fn status_text(&self) -> &str {
        match self.status {
            0 => "隐藏",
            1 => "显示",
            _ => "未知",
        }
    }
    
    /// 获取类型文本
    pub fn type_text(&self) -> &str {
        match self.type_ {
            0 => "通知",
            1 => "公告",
            _ => "未知",
        }
    }
}
