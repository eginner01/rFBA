//! 插件实体 - sys_plugin表

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "sys_plugin")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub name: String,
    pub code: String,
    pub version: String,
    pub plugin_type: i32,
    pub description: Option<String>,
    pub author: Option<String>,
    pub homepage: Option<String>,
    pub file_path: String,
    pub class_name: String,
    pub config: Option<String>,
    pub status: i32,
    pub sort_order: i32,
    pub is_system: i32,
    pub dependencies: Option<String>,
    pub install_time: Option<DateTime>,
    pub uninstall_time: Option<DateTime>,
    pub created_time: DateTime,
    pub updated_time: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
#[sea_orm()]
pub enum Relation {
    PluginConfig,
}

impl Related<super::plugin_config::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PluginConfig.def()
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

/// 插件类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PluginType {
    /// 功能插件
    Feature = 0,
    /// 主题插件
    Theme = 1,
    /// 工具插件
    Tool = 2,
    /// 数据插件
    Data = 3,
    /// 安全插件
    Security = 4,
    /// 其他插件
    Other = 5,
}

impl PluginType {
    /// 从i32值创建PluginType
    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            0 => Some(PluginType::Feature),
            1 => Some(PluginType::Theme),
            2 => Some(PluginType::Tool),
            3 => Some(PluginType::Data),
            4 => Some(PluginType::Security),
            5 => Some(PluginType::Other),
            _ => None,
        }
    }

    /// 获取类型名称
    pub fn get_name(&self) -> &'static str {
        match self {
            PluginType::Feature => "功能插件",
            PluginType::Theme => "主题插件",
            PluginType::Tool => "工具插件",
            PluginType::Data => "数据插件",
            PluginType::Security => "安全插件",
            PluginType::Other => "其他插件",
        }
    }
}

/// 插件状态
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PluginStatus {
    /// 禁用
    Disabled = 0,
    /// 启用
    Enabled = 1,
    /// 已卸载
    Uninstalled = 2,
}

impl PluginStatus {
    /// 从i32值创建PluginStatus
    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            0 => Some(PluginStatus::Disabled),
            1 => Some(PluginStatus::Enabled),
            2 => Some(PluginStatus::Uninstalled),
            _ => None,
        }
    }

    /// 获取状态名称
    pub fn get_name(&self) -> &'static str {
        match self {
            PluginStatus::Disabled => "禁用",
            PluginStatus::Enabled => "启用",
            PluginStatus::Uninstalled => "已卸载",
        }
    }
}
