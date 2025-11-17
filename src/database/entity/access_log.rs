//! 访问日志实体 - sys_access_log表

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "sys_access_log")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub user_id: Option<i64>,
    pub user_name: Option<String>,
    pub dept_id: Option<i64>,
    pub dept_name: Option<String>,
    pub trace_id: String,
    pub parent_trace_id: Option<String>,
    pub method: String,
    pub url: String,
    pub query_params: Option<String>,
    pub request_body: Option<String>,
    pub status_code: u16,
    pub response_body: Option<String>,
    pub client_ip: String,
    pub user_agent: Option<String>,
    pub os: Option<String>,
    pub browser: Option<String>,
    pub device_type: Option<String>,
    pub referer: Option<String>,
    pub cost_time: i64,
    pub is_error: bool,
    pub error_msg: Option<String>,
    pub access_time: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
#[sea_orm()]
pub enum Relation {
    User,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {
    fn before_save(c: &mut sea_orm::query::Iterable, insert: bool) {
        let now = chrono::Utc::now().naive_utc();
        if insert {
            if c.get("access_time").is_none() {
                c.set("access_time", now);
            }
        }
    }
}

/// 设备类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeviceType {
    /// 桌面
    Desktop,
    /// 移动设备
    Mobile,
    /// 平板
    Tablet,
    /// 电视
    Tv,
    /// 游戏机
    Console,
    /// 未知
    Unknown,
}

impl DeviceType {
    /// 从字符串创建设备类型
    pub fn from_str(device_type: &str) -> Self {
        match device_type.to_lowercase().as_str() {
            "desktop" => DeviceType::Desktop,
            "mobile" => DeviceType::Mobile,
            "tablet" => DeviceType::Tablet,
            "tv" => DeviceType::Tv,
            "console" => DeviceType::Console,
            _ => DeviceType::Unknown,
        }
    }

    /// 获取设备类型名称
    pub fn get_name(&self) -> &'static str {
        match self {
            DeviceType::Desktop => "桌面",
            DeviceType::Mobile => "移动设备",
            DeviceType::Tablet => "平板",
            DeviceType::Tv => "电视",
            DeviceType::Console => "游戏机",
            DeviceType::Unknown => "未知",
        }
    }
}
