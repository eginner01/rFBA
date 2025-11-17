//! 代码生成器DTO定义

use serde::{Deserialize, Serialize};
use validator::Validate;
use std::collections::HashMap;

/// 表信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TableInfo {
    pub table_name: String,
    pub table_schema: String,
    pub table_comment: Option<String>,
}

/// 业务详情
#[derive(Debug, Serialize, Deserialize)]
pub struct GenBusinessDetail {
    pub id: i64,
    pub app_name: String,
    pub table_name: String,
    pub doc_comment: String,
    pub table_comment: Option<String>,
    pub class_name: Option<String>,
    pub schema_name: Option<String>,
    pub filename: Option<String>,
    pub default_datetime_column: bool,
    pub api_version: String,
    pub gen_path: Option<String>,
    pub remark: Option<String>,
    pub created_time: String,
}

/// 创建业务请求
#[derive(Debug, Deserialize, Validate)]
pub struct CreateGenBusinessParam {
    #[validate(length(min = 1, max = 64))]
    pub app_name: String,
    
    #[validate(length(min = 1, max = 256))]
    pub table_name: String,
    
    #[validate(length(min = 1, max = 256))]
    pub doc_comment: String,
    
    pub table_comment: Option<String>,
    pub class_name: Option<String>,
    pub schema_name: Option<String>,
    pub filename: Option<String>,
    pub default_datetime_column: Option<bool>,
    pub api_version: Option<String>,
    pub gen_path: Option<String>,
    pub remark: Option<String>,
}

/// 更新业务请求
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateGenBusinessParam {
    pub app_name: Option<String>,
    pub doc_comment: Option<String>,
    pub table_comment: Option<String>,
    pub class_name: Option<String>,
    pub schema_name: Option<String>,
    pub filename: Option<String>,
    pub default_datetime_column: Option<bool>,
    pub api_version: Option<String>,
    pub gen_path: Option<String>,
    pub remark: Option<String>,
}

/// 列详情
#[derive(Debug, Serialize, Deserialize)]
pub struct GenColumnDetail {
    pub id: i64,
    pub business_id: i64,
    pub column_name: String,
    pub column_comment: Option<String>,
    pub column_type: String,
    pub python_type: Option<String>,
    pub ts_type: Option<String>,
    pub required: bool,
    pub is_pk: bool,
    pub is_fk: bool,
    pub is_query: bool,
    pub is_list: bool,
    pub is_form: bool,
    pub query_type: Option<String>,
    pub form_type: Option<String>,
    pub sort: i32,
}

/// 列信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ColumnInfo {
    pub column_name: String,
    pub data_type: String,
    pub column_type: String,
    pub is_nullable: String,
    pub column_key: String,
    pub column_comment: Option<String>,
}

/// 生成代码请求
#[derive(Debug, Deserialize, Validate)]
pub struct GenerateCodeParam {
    #[validate(length(min = 1))]
    pub table_name: String,
    
    #[validate(length(min = 1))]
    pub module_name: String,
    
    pub author: Option<String>,
}

/// 导入表参数
#[derive(Debug, Deserialize, Validate)]
pub struct ImportTableParam {
    #[validate(length(min = 1, max = 64))]
    pub app: String,
    
    #[validate(length(min = 1, max = 256))]
    pub table_name: String,
    
    pub table_schema: Option<String>,
}

/// 代码预览响应
#[derive(Debug, Serialize, Deserialize)]
pub struct CodePreview {
    pub files: HashMap<String, String>,
}
