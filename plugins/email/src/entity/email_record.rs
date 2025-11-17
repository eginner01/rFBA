//! 邮件发送记录实体

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// 邮件发送记录实体模型
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "sys_email_record")]
pub struct Model {
    /// 主键ID
    #[sea_orm(primary_key)]
    pub id: i64,
    
    /// 收件人邮箱
    #[sea_orm(column_type = "String(StringLen::N(255))")]
    pub to_email: String,
    
    /// 邮件主题
    #[sea_orm(column_type = "String(StringLen::N(255))")]
    pub subject: String,
    
    /// 邮件内容
    #[sea_orm(column_type = "Text")]
    pub content: String,
    
    /// 是否HTML格式（1是/0否）
    pub is_html: i16,
    
    /// 发送状态（0待发送/1发送成功/2发送失败）
    pub status: i16,
    
    /// 错误信息
    #[sea_orm(column_type = "Text", nullable)]
    pub error_msg: Option<String>,
    
    /// 发送时间
    #[sea_orm(nullable)]
    pub send_time: Option<DateTime>,
    
    /// 创建时间
    pub created_time: DateTime,
}

/// 关系定义
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

/// ActiveModel行为实现
impl ActiveModelBehavior for ActiveModel {}

/// 查询辅助方法
impl Entity {
    /// 查找待发送的邮件
    pub fn find_pending() -> Select<Self> {
        Self::find().filter(Column::Status.eq(0))
    }
    
    /// 查找发送成功的邮件
    pub fn find_success() -> Select<Self> {
        Self::find().filter(Column::Status.eq(1))
    }
    
    /// 查找发送失败的邮件
    pub fn find_failed() -> Select<Self> {
        Self::find().filter(Column::Status.eq(2))
    }
}
