/// 系统配置实体

use chrono::{DateTime, Utc};
use sea_orm::entity::{
    ActiveModelBehavior, ActiveModelTrait, ColumnTrait, DbErr, DeriveEntityModel,
    DerivePrimaryKey, DeriveRelationColumn, EntityFilter, EntityModel, EqTrait, ModelTrait,
    PrimaryKeyTrait, QueryFilter, QuerySelect, Related, RelationDef, RelationTrait, Set,
};
use sea_orm::DynIden;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize,
)]
#[sea_orm(table_name = "sys_config")]
pub struct Model {
    /// 配置ID
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    /// 配置名称
    pub config_name: String,
    /// 配置键名
    pub config_key: String,
    /// 配置值
    pub config_value: String,
    /// 配置类型
    pub config_type: i32,
    /// 是否系统内置
    pub is_system: i32,
    /// 备注
    pub remark: Option<String>,
    /// 创建时间
    pub created_time: DateTime<Utc>,
    /// 更新时间
    pub updated_time: DateTime<Utc>,
}

/// 配置类型枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SysConfigType {
    /// 字符串
    String = 1,
    /// 数字
    Number = 2,
    /// 布尔
    Boolean = 3,
    /// JSON
    Json = 4,
}

impl SysConfigType {
    /// 获取类型名称
    pub fn get_name(&self) -> &'static str {
        match self {
            SysConfigType::String => "字符串",
            SysConfigType::Number => "数字",
            SysConfigType::Boolean => "布尔",
            SysConfigType::Json => "JSON",
        }
    }
}

impl From<i32> for SysConfigType {
    fn from(value: i32) -> Self {
        match value {
            1 => SysConfigType::String,
            2 => SysConfigType::Number,
            3 => SysConfigType::Boolean,
            4 => SysConfigType::Json,
            _ => SysConfigType::String,
        }
    }
}

impl From<SysConfigType> for i32 {
    fn from(value: SysConfigType) -> Self {
        value as i32
    }
}

impl ActiveModelBehavior for ActiveModel {
    /// 在创建新记录前触发
    fn before_insert(model: sea_orm::ActiveModel<Self>) -> Result<Self, DbErr> {
        let mut model = model;
        if model.id.as_ref() == &0 {
            model.id = Set(chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0) as i64);
        }
        if model.created_time.as_ref() == &chrono::DateTime::default() {
            model.created_time = Set(Utc::now());
        }
        if model.updated_time.as_ref() == &chrono::DateTime::default() {
            model.updated_time = Set(Utc::now());
        }
        Ok(model)
    }

    /// 在更新记录前触发
    fn before_update(model: sea_orm::ActiveModel<Self>) -> Result<Self, DbErr> {
        let mut model = model;
        model.updated_time = Set(Utc::now());
        Ok(model)
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
#[sea_orm()]
pub enum Relation {
    /// 无关联
    #[sea_orm(name = "sys_config_id_fk")]
    SysConfigIdFk,
}

impl Related<sea_orm::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SysConfigIdFk.def()
    }
}

impl ActiveModel {
    /// 设置配置ID
    pub fn id(mut self, id: i64) -> Self {
        self.id = Set(id);
        self
    }

    /// 设置配置名称
    pub fn config_name(mut self, config_name: String) -> Self {
        self.config_name = Set(config_name);
        self
    }

    /// 设置配置键名
    pub fn config_key(mut self, config_key: String) -> Self {
        self.config_key = Set(config_key);
        self
    }

    /// 设置配置值
    pub fn config_value(mut self, config_value: String) -> Self {
        self.config_value = Set(config_value);
        self
    }

    /// 设置配置类型
    pub fn config_type(mut self, config_type: SysConfigType) -> Self {
        self.config_type = Set(config_type as i32);
        self
    }

    /// 设置是否系统内置
    pub fn is_system(mut self, is_system: i32) -> Self {
        self.is_system = Set(is_system);
        self
    }

    /// 设置备注
    pub fn remark(mut self, remark: Option<String>) -> Self {
        self.remark = Set(remark);
        self
    }

    /// 设置创建时间
    pub fn created_time(mut self, created_time: DateTime<Utc>) -> Self {
        self.created_time = Set(created_time);
        self
    }

    /// 设置更新时间
    pub fn updated_time(mut self, updated_time: DateTime<Utc>) -> Self {
        self.updated_time = Set(updated_time);
        self
    }
}

impl Entity {
    /// 根据配置键查找
    pub fn find_by_config_key(config_key: &str) -> Select<Self> {
        Self::find().filter(Column::ConfigKey.eq(config_key))
    }

    /// 根据配置类型查找
    pub fn find_by_config_type(config_type: SysConfigType) -> Select<Self> {
        Self::find().filter(Column::ConfigType.eq(config_type as i32))
    }

    /// 查找非系统内置配置
    pub fn find_non_system() -> Select<Self> {
        Self::find().filter(Column::IsSystem.eq(0))
    }

    /// 查找系统内置配置
    pub fn find_system() -> Select<Self> {
        Self::find().filter(Column::IsSystem.eq(1))
    }
}

impl Model {
    /// 获取配置类型
    pub fn get_config_type(&self) -> SysConfigType {
        SysConfigType::from(self.config_type)
    }

    /// 检查是否为系统内置
    pub fn is_system_config(&self) -> bool {
        self.is_system == 1
    }

    /// 检查是否为非系统内置
    pub fn is_non_system_config(&self) -> bool {
        self.is_system == 0
    }
}
