/// 插件配置实体模型
/// 对应数据库中的 sys_plugin_config 表

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "sys_plugin_config")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    /// 插件ID
    pub plugin_id: i64,
    /// 配置项名称
    pub config_key: String,
    /// 配置项值
    pub config_value: String,
    /// 配置项描述
    pub description: Option<String>,
    /// 配置类型（0: 文本, 1: 数字, 2: 布尔, 3: JSON）
    pub value_type: i32,
    /// 是否必填
    pub is_required: i32,
    /// 默认值
    pub default_value: Option<String>,
    /// 验证规则
    pub validation_rule: Option<String>,
    /// 创建时间
    pub created_time: DateTime,
    /// 更新时间
    pub updated_time: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
#[sea_orm()]
pub enum Relation {
    /// 关联插件
    Plugin,
}

impl Related<super::plugin::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Plugin.def()
    }
}

impl ActiveModelBehavior for ActiveModel {
    fn before_save(c: &mut sea_orm::query::Iterable, insert: bool) {
        let now = chrono::Utc::now().naive_utc();
        if insert {
            if c.get("created_time").is_none() {
                c.set("created_time", now);
            }
        }
        if c.get("updated_time").is_none() {
            c.set("updated_time", now);
        }
    }
}

/// 配置值类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigValueType {
    /// 文本
    Text = 0,
    /// 数字
    Number = 1,
    /// 布尔
    Boolean = 2,
    /// JSON
    Json = 3,
}

impl ConfigValueType {
    /// 从i32值创建ConfigValueType
    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            0 => Some(ConfigValueType::Text),
            1 => Some(ConfigValueType::Number),
            2 => Some(ConfigValueType::Boolean),
            3 => Some(ConfigValueType::Json),
            _ => None,
        }
    }

    /// 获取类型名称
    pub fn get_name(&self) -> &'static str {
        match self {
            ConfigValueType::Text => "文本",
            ConfigValueType::Number => "数字",
            ConfigValueType::Boolean => "布尔",
            ConfigValueType::Json => "JSON",
        }
    }
}
