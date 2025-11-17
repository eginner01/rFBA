/// 数据规则查询相关的 DTO
/// 包括分页查询参数等

use serde::{Deserialize, Serialize};
use validator::Validate;

/// 数据规则分页查询参数
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct DataRulePaginationQuery {
    /// 页码（从1开始）
    pub page: Option<u64>,

    /// 每页大小
    pub size: Option<u64>,

    /// 规则名称（模糊搜索）
    pub name: Option<String>,

    /// 规则编码（模糊搜索）
    pub code: Option<String>,

    /// 模型名称（精确搜索）
    pub model: Option<String>,

    /// 状态（0-禁用，1-启用）
    pub status: Option<i32>,

    /// 开始时间
    pub start_time: Option<chrono::DateTime<chrono::Utc>>,

    /// 结束时间
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
}

/// 数据规则查询参数（用于简单的列表查询）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataRuleQueryParams {
    /// 规则名称（模糊搜索）
    pub name: Option<String>,

    /// 规则编码（模糊搜索）
    pub code: Option<String>,

    /// 模型名称（精确搜索）
    pub model: Option<String>,

    /// 状态（0-禁用，1-启用）
    pub status: Option<i32>,
}

/// 获取模型列表查询参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataRuleModelQuery {
    /// 关键词（用于模糊搜索模型名称）
    pub keyword: Option<String>,
}

/// 获取模型列信息查询参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataRuleColumnQuery {
    /// 是否包含系统字段
    pub include_system: Option<bool>,
}
