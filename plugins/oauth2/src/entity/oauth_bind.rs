//! OAuth用户绑定实体

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// OAuth用户绑定实体模型
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "sys_oauth_user_bind")]
pub struct Model {
    /// 主键ID
    #[sea_orm(primary_key)]
    pub id: i64,
    
    /// 用户ID
    pub user_id: i64,
    
    /// OAuth提供商（github/google等）
    #[sea_orm(column_type = "String(StringLen::N(50))")]
    pub provider: String,
    
    /// OAuth提供商用户ID
    #[sea_orm(column_type = "String(StringLen::N(255))")]
    pub provider_user_id: String,
    
    /// 访问令牌
    #[sea_orm(column_type = "String(StringLen::N(500))", nullable)]
    pub access_token: Option<String>,
    
    /// 刷新令牌
    #[sea_orm(column_type = "String(StringLen::N(500))", nullable)]
    pub refresh_token: Option<String>,
    
    /// 令牌过期时间
    #[sea_orm(nullable)]
    pub expires_at: Option<DateTime>,
    
    /// 用户信息JSON
    #[sea_orm(column_type = "Text", nullable)]
    pub user_info: Option<String>,
    
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
    /// 根据用户ID和提供商查找绑定
    pub fn find_by_user_and_provider(user_id: i64, provider: &str) -> Select<Self> {
        Self::find()
            .filter(Column::UserId.eq(user_id))
            .filter(Column::Provider.eq(provider))
    }
    
    /// 根据提供商用户ID查找
    pub fn find_by_provider_user_id(provider: &str, provider_user_id: &str) -> Select<Self> {
        Self::find()
            .filter(Column::Provider.eq(provider))
            .filter(Column::ProviderUserId.eq(provider_user_id))
    }
}
