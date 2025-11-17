/// 数据字典查询 DTO

use serde::{Deserialize, Serialize};
use validator::Validate;

/// 分页查询参数
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct DictDataQuery {
    /// 页码
    #[validate(range(min = 1, message = "页码必须大于0"))]
    pub page_num: Option<usize>,

    /// 每页数量
    #[validate(range(min = 1, max = 100, message = "每页数量必须在1-100之间"))]
    pub page_size: Option<usize>,

    /// 字典标签
    pub dict_label: Option<String>,

    /// 字典键值
    pub dict_value: Option<String>,

    /// 字典类型编码
    pub dict_type: Option<String>,

    /// 状态
    pub status: Option<i32>,
}

/// 字典类型查询
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DictTypeQuery {
    /// 字典类型编码
    pub dict_type: String,
}

/// 字典项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DictItem {
    /// 字典标签
    pub label: String,
    /// 字典键值
    pub value: String,
    /// 表格回显样式
    pub list_class: Option<String>,
}

/// 字典数据列表
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DictDataList {
    /// 字典类型编码
    pub dict_type: String,
    /// 字典项列表
    pub items: Vec<DictItem>,
}

/// 分页响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DictDataListResponse {
    /// 字典列表
    pub list: Vec<DictDataListItem>,
    /// 总数
    pub total: usize,
    /// 页码
    pub page_num: usize,
    /// 每页数量
    pub page_size: usize,
}

/// 字典列表项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DictDataListItem {
    /// 字典编码
    pub dict_code: i64,
    /// 字典排序
    pub dict_sort: i32,
    /// 字典标签
    pub dict_label: String,
    /// 字典键值
    pub dict_value: String,
    /// 字典类型编码
    pub dict_type: String,
    /// 字典类型名称
    pub dict_type_name: String,
    /// 样式属性
    pub css_class: Option<String>,
    /// 表格回显样式
    pub list_class: Option<String>,
    /// 是否默认
    pub is_default: i32,
    /// 是否默认名称
    pub is_default_name: String,
    /// 状态
    pub status: i32,
    /// 状态名称
    pub status_name: String,
    /// 备注
    pub remark: Option<String>,
    /// 创建时间
    pub created_time: chrono::DateTime<chrono::Utc>,
    /// 更新时间
    pub updated_time: chrono::DateTime<chrono::Utc>,
}
