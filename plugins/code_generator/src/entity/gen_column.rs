//! 代码生成列实体

use sea_orm::entity::prelude::*;
use sea_orm::QueryOrder;
use serde::{Deserialize, Serialize};

/// 代码生成列实体模型
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "gen_column")]
pub struct Model {
    /// 主键ID
    #[sea_orm(primary_key)]
    pub id: i64,
    
    /// 业务ID
    pub business_id: i64,
    
    /// 列名称
    #[sea_orm(column_type = "String(StringLen::N(256))")]
    pub column_name: String,
    
    /// 列注释
    #[sea_orm(column_type = "String(StringLen::N(256))", nullable)]
    pub column_comment: Option<String>,
    
    /// 列类型
    #[sea_orm(column_type = "String(StringLen::N(64))")]
    pub column_type: String,
    
    /// Python类型
    #[sea_orm(column_type = "String(StringLen::N(64))", nullable)]
    pub python_type: Option<String>,
    
    /// TypeScript类型
    #[sea_orm(column_type = "String(StringLen::N(64))", nullable)]
    pub ts_type: Option<String>,
    
    /// 是否必填
    pub required: bool,
    
    /// 是否为主键
    pub is_pk: bool,
    
    /// 是否为外键
    pub is_fk: bool,
    
    /// 是否在查询中显示
    pub is_query: bool,
    
    /// 是否在列表中显示
    pub is_list: bool,
    
    /// 是否在表单中显示
    pub is_form: bool,
    
    /// 查询类型（eq/like/range等）
    #[sea_orm(column_type = "String(StringLen::N(32))", nullable)]
    pub query_type: Option<String>,
    
    /// 表单类型（input/select/date等）
    #[sea_orm(column_type = "String(StringLen::N(32))", nullable)]
    pub form_type: Option<String>,
    
    /// 排序
    pub sort: i32,
    
    /// 创建时间
    pub created_time: DateTime,
    
    /// 更新时间
    #[sea_orm(nullable)]
    pub updated_time: Option<DateTime>,
}

/// 关系定义
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::gen_business::Entity",
        from = "Column::BusinessId",
        to = "super::gen_business::Column::Id"
    )]
    GenBusiness,
}

impl Related<super::gen_business::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::GenBusiness.def()
    }
}

/// ActiveModel行为实现
impl ActiveModelBehavior for ActiveModel {}

/// 查询辅助方法
impl Entity {
    /// 根据业务ID查找列
    pub fn find_by_business_id(business_id: i64) -> Select<Self> {
        Self::find()
            .filter(Column::BusinessId.eq(business_id))
            .order_by_asc(Column::Sort)
    }
}
