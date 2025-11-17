/// 字典数据API请求和响应DTO
/// 匹配Python后端的接口规范

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// 字典数据类型编码查询参数
#[derive(Debug, Deserialize)]
pub struct DictDataByTypeCodePath {
    pub code: String,
}

/// 字典数据分页查询参数
#[derive(Debug, Deserialize, Default)]
pub struct DictDataListQuery {
    pub page: Option<i32>,
    pub size: Option<i32>,
    pub type_code: Option<String>,
    pub label: Option<String>,
    pub value: Option<String>,
    pub status: Option<i32>,
    pub type_id: Option<i64>,
}

/// 创建字典数据请求
#[derive(Debug, Deserialize)]
pub struct CreateDictDataRequest {
    pub type_id: i64,
    pub label: String,
    pub value: String,
    pub color: Option<String>,
    pub sort: Option<i32>,
    pub status: Option<i32>,
    pub remark: Option<String>,
}

/// 更新字典数据请求
#[derive(Debug, Deserialize)]
pub struct UpdateDictDataRequest {
    pub type_id: i64,
    pub label: String,
    pub value: String,
    pub color: Option<String>,
    pub sort: Option<i32>,
    pub status: Option<i32>,
    pub remark: Option<String>,
}

/// 批量删除字典数据请求
#[derive(Debug, Deserialize)]
pub struct DeleteDictDataRequest {
    pub pks: Vec<i64>,
}

/// 字典数据详情响应
#[derive(Debug, Serialize)]
pub struct DictDataDetailResponse {
    pub id: i64,
    pub type_code: String,
    pub label: String,
    pub value: String,
    pub color: Option<String>,
    pub sort: i32,
    pub status: i32,
    pub remark: Option<String>,
    pub created_time: DateTime<Utc>,
    pub updated_time: Option<DateTime<Utc>>,
}

/// 字典数据列表项响应
#[derive(Debug, Serialize)]
pub struct DictDataListItem {
    pub id: i64,
    pub type_code: String,
    pub label: String,
    pub value: String,
    pub color: Option<String>,
    pub sort: i32,
    pub status: i32,
    pub remark: Option<String>,
    pub created_time: DateTime<Utc>,
}

/// 字典数据分页响应
#[derive(Debug, Serialize)]
pub struct DictDataPaginationResponse {
    pub list: Vec<DictDataListItem>,
    pub total: usize,
    pub page: i32,
    pub size: i32,
    pub pages: usize,
}
