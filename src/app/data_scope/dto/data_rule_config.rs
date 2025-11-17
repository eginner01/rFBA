/// 数据规则配置相关的 DTO
/// 包括创建、更新、删除等操作的数据结构

use serde::{Deserialize, Serialize};
use validator::Validate;

/// 创建数据规则请求
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateDataRuleRequest {
    /// 规则名称
    #[validate(length(min = 1, max = 512, message = "规则名称长度必须在1-512个字符之间"))]
    pub name: String,

    /// 模型名称
    #[validate(length(min = 1, max = 64, message = "模型名称长度必须在1-64个字符之间"))]
    pub model: String,

    /// 字段名称
    #[validate(length(min = 1, max = 32, message = "字段名称长度必须在1-32个字符之间"))]
    pub column: String,

    /// 运算符（0：and、1：or）
    pub operator: i32,

    /// 表达式（0：==、1：!=、2：>、3：>=、4：<、5：<=、6：in、7：not_in）
    pub expression: i32,

    /// 规则值
    #[validate(length(min = 1, max = 256, message = "规则值长度必须在1-256个字符之间"))]
    pub value: String,
}

/// 更新数据规则请求
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateDataRuleRequest {
    /// 规则名称
    #[validate(length(min = 1, max = 512, message = "规则名称长度必须在1-512个字符之间"))]
    pub name: String,

    /// 模型名称
    #[validate(length(min = 1, max = 64, message = "模型名称长度必须在1-64个字符之间"))]
    pub model: String,

    /// 字段名称
    #[validate(length(min = 1, max = 32, message = "字段名称长度必须在1-32个字符之间"))]
    pub column: String,

    /// 运算符（0：and、1：or）
    pub operator: i32,

    /// 表达式（0：==、1：!=、2：>、3：>=、4：<、5：<=、6：in、7：not_in）
    pub expression: i32,

    /// 规则值
    #[validate(length(min = 1, max = 256, message = "规则值长度必须在1-256个字符之间"))]
    pub value: String,
}

/// 数据规则模型信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataRuleModelInfo {
    pub model: String,
    pub name: String,
    pub description: String,
}

/// 数据规则字段信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataRuleColumnInfo {
    pub name: String,
    pub type_: String,
    pub nullable: bool,
    pub comment: Option<String>,
}

/// 数据规则树节点
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataRuleTreeNode {
    pub id: i64,
    pub name: String,
    pub code: String,
    pub model: String,
    pub columns: Option<String>,
    pub field_permissions: Option<String>,
    pub status: i32,
    pub sort: i32,
    pub remark: Option<String>,
    pub created_time: chrono::DateTime<chrono::Utc>,
    pub updated_time: chrono::DateTime<chrono::Utc>,
    pub create_by: Option<String>,
    pub update_by: Option<String>,
    pub children: Vec<DataRuleTreeNode>,
}
