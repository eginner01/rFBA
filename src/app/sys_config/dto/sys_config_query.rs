/// 系统配置查询 DTO

use serde::{Deserialize, Serialize};
use validator::Validate;

/// 分页查询参数
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct SysConfigQuery {
    /// 页码
    #[validate(range(min = 1, message = "页码必须大于0"))]
    pub page_num: Option<usize>,

    /// 每页数量
    #[validate(range(min = 1, max = 100, message = "每页数量必须在1-100之间"))]
    pub page_size: Option<usize>,

    /// 配置名称
    pub config_name: Option<String>,

    /// 配置键名
    pub config_key: Option<String>,

    /// 配置类型
    pub config_type: Option<i32>,

    /// 是否系统内置
    pub is_system: Option<i32>,
}

/// 配置键名查询
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysConfigKeyQuery {
    /// 配置键名列表
    pub config_keys: Vec<String>,
}

/// 配置键值响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysConfigKeyValue {
    /// 配置键名
    pub config_key: String,
    /// 配置值
    pub config_value: String,
    /// 配置类型
    pub config_type: i32,
    /// 配置类型名称
    pub config_type_name: String,
}

/// 分页响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysConfigListResponse {
    /// 配置列表
    pub list: Vec<SysConfigListItem>,
    /// 总数
    pub total: usize,
    /// 页码
    pub page_num: usize,
    /// 每页数量
    pub page_size: usize,
}

/// 配置列表项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysConfigListItem {
    /// 配置ID
    pub id: i64,
    /// 配置名称
    pub config_name: String,
    /// 配置键名
    pub config_key: String,
    /// 配置值
    pub config_value: String,
    /// 配置类型
    pub config_type: i32,
    /// 配置类型名称
    pub config_type_name: String,
    /// 是否系统内置
    pub is_system: i32,
    /// 是否系统内置名称
    pub is_system_name: String,
    /// 备注
    pub remark: Option<String>,
    /// 创建时间
    pub created_time: chrono::DateTime<chrono::Utc>,
    /// 更新时间
    pub updated_time: chrono::DateTime<chrono::Utc>,
}
