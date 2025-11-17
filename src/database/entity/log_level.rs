/// 日志级别实体

use chrono::{DateTime, Utc};
use sea_orm::entity::{
    ActiveModelBehavior, ActiveModelTrait, ColumnTrait, DbErr, DeriveEntityModel,
    DerivePrimaryKey, DeriveRelationColumn, EntityFilter, EntityModel, EqTrait, ModelTrait,
    PrimaryKeyTrait, QueryFilter, QuerySelect, Related, RelationDef, RelationTrait, Set,
};
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize,
)]
#[sea_orm(table_name = "sys_log_level")]
pub struct Model {
    /// 级别ID
    #[sea_orm(primary_key, auto_increment = false)]
    pub level_id: i64,
    /// 日志级别
    pub level_name: String,
    /// 级别值
    pub level_value: i32,
    /// 级别描述
    pub description: Option<String>,
    /// 是否系统内置（0:否 1:是）
    pub is_system: i32,
    /// 状态（0:启用 1:禁用）
    pub status: i32,
    /// 创建者
    pub create_by: String,
    /// 创建时间
    pub created_time: DateTime<Utc>,
    /// 更新者
    pub update_by: Option<String>,
    /// 更新时间
    pub updated_time: DateTime<Utc>,
}

/// 日志级别状态枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogLevelStatus {
    /// 启用
    Enabled = 0,
    /// 禁用
    Disabled = 1,
}

impl LogLevelStatus {
    /// 获取状态名称
    pub fn get_name(&self) -> &'static str {
        match self {
            LogLevelStatus::Enabled => "启用",
            LogLevelStatus::Disabled => "禁用",
        }
    }
}

impl From<i32> for LogLevelStatus {
    fn from(value: i32) -> Self {
        match value {
            1 => LogLevelStatus::Disabled,
            _ => LogLevelStatus::Enabled,
        }
    }
}

impl From<LogLevelStatus> for i32 {
    fn from(value: LogLevelStatus) -> Self {
        value as i32
    }
}

impl ActiveModelBehavior for ActiveModel {
    /// 在创建新记录前触发
    fn before_insert(model: sea_orm::ActiveModel<Self>) -> Result<Self, DbErr> {
        let mut model = model;
        if model.level_id.as_ref() == &0 {
            model.level_id = Set(chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0) as i64);
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
    #[sea_orm(name = "sys_log_level_level_id_fk")]
    SysLogLevelLevelIdFk,
}

impl Related<sea_orm::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SysLogLevelLevelIdFk.def()
    }
}

impl ActiveModel {
    /// 设置级别ID
    pub fn level_id(mut self, level_id: i64) -> Self {
        self.level_id = Set(level_id);
        self
    }

    /// 设置日志级别
    pub fn level_name(mut self, level_name: String) -> Self {
        self.level_name = Set(level_name);
        self
    }

    /// 设置级别值
    pub fn level_value(mut self, level_value: i32) -> Self {
        self.level_value = Set(level_value);
        self
    }

    /// 设置级别描述
    pub fn description(mut self, description: Option<String>) -> Self {
        self.description = Set(description);
        self
    }

    /// 设置是否系统内置
    pub fn is_system(mut self, is_system: i32) -> Self {
        self.is_system = Set(is_system);
        self
    }

    /// 设置状态
    pub fn status(mut self, status: LogLevelStatus) -> Self {
        self.status = Set(status as i32);
        self
    }

    /// 设置创建者
    pub fn create_by(mut self, create_by: String) -> Self {
        self.create_by = Set(create_by);
        self
    }

    /// 设置创建时间
    pub fn created_time(mut self, created_time: DateTime<Utc>) -> Self {
        self.created_time = Set(created_time);
        self
    }

    /// 设置更新者
    pub fn update_by(mut self, update_by: Option<String>) -> Self {
        self.update_by = Set(update_by);
        self
    }

    /// 设置更新时间
    pub fn updated_time(mut self, updated_time: DateTime<Utc>) -> Self {
        self.updated_time = Set(updated_time);
        self
    }
}

impl Entity {
    /// 根据级别值查找
    pub fn find_by_level_value(level_value: i32) -> Select<Self> {
        Self::find().filter(Column::LevelValue.eq(level_value))
    }

    /// 根据状态查找
    pub fn find_by_status(status: LogLevelStatus) -> Select<Self> {
        Self::find().filter(Column::Status.eq(status as i32))
    }

    /// 查找启用的级别
    pub fn find_enabled() -> Select<Self> {
        Self::find().filter(Column::Status.eq(0))
    }

    /// 查找系统内置级别
    pub fn find_system() -> Select<Self> {
        Self::find().filter(Column::IsSystem.eq(1))
    }

    /// 查找非系统内置级别
    pub fn find_non_system() -> Select<Self> {
        Self::find().filter(Column::IsSystem.eq(0))
    }

    /// 按级别值排序查找
    pub fn find_order_by_value() -> Select<Self> {
        Self::find().order_by(Column::LevelValue, sea_orm::Order::Asc)
    }
}

impl Model {
    /// 获取状态
    pub fn get_status(&self) -> LogLevelStatus {
        LogLevelStatus::from(self.status)
    }

    /// 检查是否为系统内置
    pub fn is_system(&self) -> bool {
        self.is_system == 1
    }

    /// 检查是否为启用状态
    pub fn is_enabled(&self) -> bool {
        self.status == 0
    }

    /// 检查是否为禁用状态
    pub fn is_disabled(&self) -> bool {
        self.status == 1
    }
}
