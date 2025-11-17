//! 登录日志实体 - sys_login_log表

use sea_orm::entity::prelude::*;
use sea_orm::{EnumIter, DeriveRelation};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "sys_login_log")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub user_uuid: String,
    pub username: String,
    pub status: i32,
    pub ip: String,
    pub country: Option<String>,
    pub region: Option<String>,
    pub city: Option<String>,
    pub user_agent: String,
    pub browser: Option<String>,
    pub os: Option<String>,
    pub device: Option<String>,
    pub msg: String,
    pub login_time: DateTime,
    pub created_time: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

// impl Related<super::user::Entity> for Entity {
//     fn to() -> RelationDef {
//         Relation::User.def()
//     }
// }

impl ActiveModelBehavior for ActiveModel {
    // 可以在此处执行自定义逻辑
    // 在保存前设置默认值等操作
}

/// 登录状态
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoginStatus {
    /// 失败
    Failure = 0,
    /// 成功
    Success = 1,
}

impl LoginStatus {
    /// 从i32值创建LoginStatus
    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            0 => Some(LoginStatus::Failure),
            1 => Some(LoginStatus::Success),
            _ => None,
        }
    }

    /// 获取状态名称
    pub fn get_name(&self) -> &'static str {
        match self {
            LoginStatus::Failure => "失败",
            LoginStatus::Success => "成功",
        }
    }

    /// 获取状态描述
    pub fn get_description(&self) -> &'static str {
        match self {
            LoginStatus::Failure => "登录失败",
            LoginStatus::Success => "登录成功",
        }
    }
}
