/// 数据字典响应 DTO

use serde::{Deserialize, Serialize};

/// 字典详情响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DictDataDetailResponse {
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
    /// 创建人
    pub create_by: Option<String>,
    /// 更新人
    pub update_by: Option<String>,
    /// 创建时间
    pub created_time: chrono::DateTime<chrono::Utc>,
    /// 更新时间
    pub updated_time: chrono::DateTime<chrono::Utc>,
}

/// 字典类型统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DictTypeStatistics {
    /// 字典类型编码
    pub dict_type: String,
    /// 字典类型名称
    pub dict_type_name: String,
    /// 字典项数量
    pub count: usize,
    /// 正常状态数量
    pub normal_count: usize,
    /// 停用状态数量
    pub disabled_count: usize,
}

/// 字典分组统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DictDataGroupStatistics {
    /// 默认字典数量
    pub default_count: usize,
    /// 非默认字典数量
    pub non_default_count: usize,
    /// 正常状态数量
    pub normal_count: usize,
    /// 停用状态数量
    pub disabled_count: usize,
    /// 总字典数量
    pub total_count: usize,
}
