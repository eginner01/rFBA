/// 数据字典创建和更新 DTO

use serde::{Deserialize, Serialize};
use validator::Validate;

/// 创建数据字典请求
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateDictDataRequest {
    /// 字典排序
    #[validate(range(min = 0, message = "字典排序必须大于等于0"))]
    pub dict_sort: i32,

    /// 字典标签
    #[validate(length(min = 1, max = 100, message = "字典标签长度必须在1-100个字符之间"))]
    pub dict_label: String,

    /// 字典键值
    #[validate(length(min = 1, max = 100, message = "字典键值长度必须在1-100个字符之间"))]
    pub dict_value: String,

    /// 字典类型编码
    #[validate(length(min = 1, max = 100, message = "字典类型编码长度必须在1-100个字符之间"))]
    pub dict_type: String,

    /// 样式属性
    pub css_class: Option<String>,

    /// 表格回显样式
    pub list_class: Option<String>,

    /// 是否默认（0:否 1:是）
    pub is_default: Option<i32>,

    /// 状态（0:正常 1:停用）
    #[validate(range(min = 0, max = 1, message = "状态必须是0或1"))]
    pub status: i32,

    /// 备注
    pub remark: Option<String>,
}

/// 更新数据字典请求
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateDictDataRequest {
    /// 字典编码
    #[validate(range(min = 1, message = "字典编码必须大于0"))]
    pub dict_code: i64,

    /// 字典排序
    #[validate(range(min = 0, message = "字典排序必须大于等于0"))]
    pub dict_sort: i32,

    /// 字典标签
    #[validate(length(min = 1, max = 100, message = "字典标签长度必须在1-100个字符之间"))]
    pub dict_label: String,

    /// 字典键值
    #[validate(length(min = 1, max = 100, message = "字典键值长度必须在1-100个字符之间"))]
    pub dict_value: String,

    /// 字典类型编码
    #[validate(length(min = 1, max = 100, message = "字典类型编码长度必须在1-100个字符之间"))]
    pub dict_type: String,

    /// 样式属性
    pub css_class: Option<String>,

    /// 表格回显样式
    pub list_class: Option<String>,

    /// 是否默认（0:否 1:是）
    pub is_default: Option<i32>,

    /// 状态（0:正常 1:停用）
    #[validate(range(min = 0, max = 1, message = "状态必须是0或1"))]
    pub status: i32,

    /// 备注
    pub remark: Option<String>,
}

/// 删除数据字典请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteDictDataRequest {
    /// 字典编码列表
    pub dict_codes: Vec<i64>,
}

/// 创建数据字典响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDictDataResponse {
    /// 字典编码
    pub dict_code: i64,
    /// 字典标签
    pub dict_label: String,
    /// 字典键值
    pub dict_value: String,
    /// 字典类型编码
    pub dict_type: String,
    /// 创建时间
    pub created_time: chrono::DateTime<chrono::Utc>,
}

/// 更新数据字典响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDictDataResponse {
    /// 字典编码
    pub dict_code: i64,
    /// 字典标签
    pub dict_label: String,
    /// 更新时间
    pub updated_time: chrono::DateTime<chrono::Utc>,
}
