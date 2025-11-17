/// 数据规则响应相关的 DTO
/// 包括API返回的数据结构

use serde::{Deserialize, Serialize};

// 注意：DataRuleTreeNode 已在 data_rule_config.rs 中定义，避免重复导出

/// 数据规则简单响应（用于列表查询，不包含详细信息）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataRuleSimpleResponse {
    pub id: i64,
    pub name: String,
    pub model: String,
    pub column: String,
}

/// 数据规则详情响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataRuleDetailResponse {
    pub id: i64,
    pub name: String,
    pub model: String,
    pub column: String,
    pub operator: i32,
    pub expression: i32,
    pub value: String,
}

/// 数据规则列表响应（用于分页查询）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataRuleListResponse {
    pub list: Vec<DataRuleDetailResponse>,
    pub total: u64,
    pub page: u64,
    pub size: u64,
}

/// 数据规则模型列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataRuleModelListResponse {
    pub models: Vec<DataRuleModelResponse>,
}

/// 数据规则模型响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataRuleModelResponse {
    pub name: String,
    pub display_name: String,
    pub description: String,
    pub table_name: String,
}

/// 数据规则字段列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataRuleColumnListResponse {
    pub columns: Vec<DataRuleColumnResponse>,
}

/// 数据规则字段响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataRuleColumnResponse {
    pub name: String,
    pub display_name: String,
    pub type_: String,
    pub nullable: bool,
    pub comment: Option<String>,
    pub primary_key: bool,
    pub unique: bool,
}

/// 数据规则批量操作响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataRuleBatchOperationResponse {
    pub success_ids: Vec<i64>,
    pub failed_ids: Vec<i64>,
    pub message: String,
}
