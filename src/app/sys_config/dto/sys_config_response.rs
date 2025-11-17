/// 系统配置响应 DTO

use serde::{Deserialize, Serialize};

/// 配置详情响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysConfigDetailResponse {
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
    /// 创建人
    pub create_by: Option<String>,
    /// 更新人
    pub update_by: Option<String>,
    /// 创建时间
    pub created_time: chrono::DateTime<chrono::Utc>,
    /// 更新时间
    pub updated_time: chrono::DateTime<chrono::Utc>,
}

/// 配置类型统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysConfigTypeStatistics {
    /// 配置类型
    pub config_type: i32,
    /// 配置类型名称
    pub config_type_name: String,
    /// 配置数量
    pub count: usize,
}

/// 配置分组统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysConfigGroupStatistics {
    /// 系统内置配置数量
    pub system_count: usize,
    /// 用户自定义配置数量
    pub custom_count: usize,
    /// 总配置数量
    pub total_count: usize,
}
