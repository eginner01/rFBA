//! 字典数据DTO定义

use serde::{Deserialize, Serialize};
use validator::Validate;
use chrono::NaiveDateTime;

/// 字典数据详情响应
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DictDataDetail {
    pub id: i64,
    pub label: String,
    pub value: String,
    pub sort: i32,
    pub type_id: i64,
    pub type_code: String,
    pub is_default: String,
    pub status: i16,
    pub remark: Option<String>,
    pub created_time: NaiveDateTime,
    pub updated_time: Option<NaiveDateTime>,
}

impl From<crate::entity::dict_data::Model> for DictDataDetail {
    fn from(model: crate::entity::dict_data::Model) -> Self {
        Self {
            id: model.id,
            label: model.label,
            value: model.value,
            sort: model.sort,
            type_id: model.type_id,
            type_code: model.type_code,
            is_default: model.is_default,
            status: model.status,
            remark: model.remark,
            created_time: model.created_time,
            updated_time: model.updated_time,
        }
    }
}

/// 创建字典数据请求参数
#[derive(Debug, Deserialize, Validate)]
pub struct CreateDictDataParam {
    /// 显示标签
    #[validate(length(min = 1, max = 64, message = "标签长度必须在1-64之间"))]
    pub label: String,
    
    /// 数据值
    #[validate(length(min = 1, max = 64, message = "数据值长度必须在1-64之间"))]
    pub value: String,
    
    /// 排序号
    #[serde(default)]
    pub sort: i32,
    
    /// 所属字典类型ID
    pub type_id: i64,
    
    /// 所属字典类型编码
    #[validate(length(min = 1, max = 32, message = "类型编码长度必须在1-32之间"))]
    pub type_code: String,
    
    /// 是否默认（Y/N）
    #[validate(length(equal = 1, message = "is_default长度必须为1"))]
    #[validate(regex(path = *YN_REGEX, message = "is_default只能是Y或N"))]
    #[serde(default = "default_is_default")]
    pub is_default: String,
    
    /// 状态（1启用/0禁用）
    #[serde(default = "default_status")]
    pub status: i16,
    
    /// 备注
    pub remark: Option<String>,
}

/// 更新字典数据请求参数
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateDictDataParam {
    /// 显示标签
    #[validate(length(min = 1, max = 64, message = "标签长度必须在1-64之间"))]
    pub label: String,
    
    /// 数据值
    #[validate(length(min = 1, max = 64, message = "数据值长度必须在1-64之间"))]
    pub value: String,
    
    /// 排序号
    pub sort: i32,
    
    /// 是否默认（Y/N）
    #[validate(regex(path = *YN_REGEX, message = "is_default只能是Y或N"))]
    pub is_default: String,
    
    /// 状态（1启用/0禁用）
    pub status: i16,
    
    /// 备注
    pub remark: Option<String>,
}

/// 字典数据查询参数
#[derive(Debug, Deserialize, Clone)]
pub struct DictDataQuery {
    /// 类型编码
    pub type_code: Option<String>,
    
    /// 标签（模糊查询）
    pub label: Option<String>,
    
    /// 数据值（模糊查询）
    pub value: Option<String>,
    
    /// 状态
    pub status: Option<i16>,
    
    /// 类型ID
    pub type_id: Option<i64>,
}

fn default_status() -> i16 {
    1
}

fn default_is_default() -> String {
    "N".to_string()
}

// Y/N正则验证
lazy_static::lazy_static! {
    static ref YN_REGEX: regex::Regex = regex::Regex::new(r"^[YN]$").unwrap();
}
