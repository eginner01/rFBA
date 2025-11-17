//! 系统配置DTO定义

use serde::{Deserialize, Serialize};
use validator::Validate;
use chrono::NaiveDateTime;

// 配置键正则验证（字母、数字、点、下划线）
lazy_static::lazy_static! {
    static ref KEY_REGEX: regex::Regex = regex::Regex::new(r"^[a-zA-Z0-9._]+$").unwrap();
}

/// 配置详情响应
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConfigDetail {
    pub id: i64,
    pub name: String,
    pub key: String,
    pub value: String,
    #[serde(rename = "type")]
    pub config_type: Option<String>,
    pub is_frontend: bool,
    pub remark: Option<String>,
    pub created_time: NaiveDateTime,
    pub updated_time: Option<NaiveDateTime>,
}

impl From<crate::entity::config::Model> for ConfigDetail {
    fn from(model: crate::entity::config::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            key: model.key,
            value: model.value,
            config_type: model.config_type,
            is_frontend: model.is_frontend,
            remark: model.remark,
            created_time: model.created_time,
            updated_time: model.updated_time,
        }
    }
}

/// 创建配置请求参数
#[derive(Debug, Deserialize, Validate)]
pub struct CreateConfigParam {
    /// 配置名称
    #[validate(length(min = 1, max = 64, message = "配置名称长度必须在1-64之间"))]
    pub name: String,
    
    /// 配置键
    #[validate(length(min = 1, max = 64, message = "配置键长度必须在1-64之间"))]
    #[validate(regex(path = *KEY_REGEX, message = "配置键只能包含字母、数字、点和下划线"))]
    pub key: String,
    
    /// 配置值
    #[validate(length(min = 0, max = 10000, message = "配置值长度不能超过10000"))]
    pub value: String,
    
    /// 配置类型
    #[serde(rename = "type")]
    #[serde(default)]
    pub config_type: Option<String>,
    
    /// 是否前端可见
    #[serde(default)]
    pub is_frontend: bool,
    
    /// 备注
    pub remark: Option<String>,
}

/// 更新配置请求参数
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateConfigParam {
    /// 配置名称
    #[validate(length(min = 1, max = 64, message = "配置名称长度必须在1-64之间"))]
    pub name: String,
    
    /// 配置值
    #[validate(length(min = 0, max = 10000, message = "配置值长度不能超过10000"))]
    pub value: String,
    
    /// 配置类型
    #[serde(rename = "type")]
    pub config_type: Option<String>,
    
    /// 是否前端可见
    pub is_frontend: bool,
    
    /// 备注
    pub remark: Option<String>,
}

/// 获取所有配置查询参数
#[derive(Debug, Deserialize, Clone)]
pub struct GetAllConfigQuery {
    /// 配置类型（可选，用于过滤）
    #[serde(rename = "type")]
    pub type_filter: Option<String>,
}

/// 配置查询参数
#[derive(Debug, Deserialize, Clone)]
pub struct ConfigQuery {
    /// 配置名称（模糊查询）
    pub name: Option<String>,
    
    /// 配置键（模糊查询）
    pub key: Option<String>,
    
    /// 是否前端可见
    pub is_frontend: Option<bool>,
}
