//! 字典类型DTO定义

use serde::{Deserialize, Serialize};
use validator::Validate;
use chrono::NaiveDateTime;

/// 字典类型详情响应
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DictTypeDetail {
    pub id: i64,
    pub name: String,
    pub code: String,
    pub status: i16,
    pub remark: Option<String>,
    pub created_time: NaiveDateTime,
    pub updated_time: Option<NaiveDateTime>,
}

impl From<crate::entity::dict_type::Model> for DictTypeDetail {
    fn from(model: crate::entity::dict_type::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            code: model.code,
            status: model.status,
            remark: model.remark,
            created_time: model.created_time,
            updated_time: model.updated_time,
        }
    }
}

/// 创建字典类型请求参数
#[derive(Debug, Deserialize, Validate)]
pub struct CreateDictTypeParam {
    /// 字典名称
    #[validate(length(min = 1, max = 32, message = "字典名称长度必须在1-32之间"))]
    pub name: String,
    
    /// 字典编码
    #[validate(length(min = 1, max = 32, message = "字典编码长度必须在1-32之间"))]
    #[validate(regex(path = *CODE_REGEX, message = "字典编码只能包含字母、数字和下划线"))]
    pub code: String,
    
    /// 状态（1启用/0禁用）
    #[serde(default = "default_status")]
    pub status: i16,
    
    /// 备注
    pub remark: Option<String>,
}

/// 更新字典类型请求参数
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateDictTypeParam {
    /// 字典名称
    #[validate(length(min = 1, max = 32, message = "字典名称长度必须在1-32之间"))]
    pub name: String,
    
    /// 状态（1启用/0禁用）
    pub status: i16,
    
    /// 备注
    pub remark: Option<String>,
}

/// 字典类型查询参数
#[derive(Debug, Deserialize, Clone)]
pub struct DictTypeQuery {
    /// 字典名称（模糊查询）
    pub name: Option<String>,
    
    /// 字典编码（模糊查询）
    pub code: Option<String>,
    
    /// 状态
    pub status: Option<i16>,
}

fn default_status() -> i16 {
    1
}

// 编码正则验证
lazy_static::lazy_static! {
    static ref CODE_REGEX: regex::Regex = regex::Regex::new(r"^[a-zA-Z0-9_]+$").unwrap();
}
