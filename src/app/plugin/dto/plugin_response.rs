/// 插件响应 DTO

use serde::{Deserialize, Serialize};
use validator::Validate;

/// 插件详情响应
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PluginDetailResponse {
    /// 插件ID
    pub id: i64,
    /// 插件名称
    pub name: String,
    /// 插件编码
    pub code: String,
    /// 插件版本
    pub version: String,
    /// 插件类型
    pub plugin_type: i32,
    /// 插件类型名称
    pub plugin_type_name: String,
    /// 插件描述
    pub description: Option<String>,
    /// 插件作者
    pub author: Option<String>,
    /// 插件主页
    pub homepage: Option<String>,
    /// 插件文件路径
    pub file_path: String,
    /// 插件类名
    pub class_name: String,
    /// 插件配置
    pub config: Option<String>,
    /// 插件状态
    pub status: i32,
    /// 插件状态名称
    pub status_name: String,
    /// 插件排序
    pub sort_order: i32,
    /// 是否系统插件
    pub is_system: i32,
    /// 依赖插件
    pub dependencies: Option<String>,
    /// 安装时间
    pub install_time: Option<chrono::DateTime<chrono::Utc>>,
    /// 卸载时间
    pub uninstall_time: Option<chrono::DateTime<chrono::Utc>>,
    /// 创建时间
    pub created_time: chrono::DateTime<chrono::Utc>,
    /// 更新时间
    pub updated_time: chrono::DateTime<chrono::Utc>,
}

/// 插件安装请求
#[derive(Debug, Deserialize)]
pub struct InstallPluginRequest {
    /// 插件文件路径
    pub file_path: String,
}

/// 插件安装响应
#[derive(Debug, Serialize)]
pub struct InstallPluginResponse {
    /// 插件ID
    pub id: i64,
    /// 插件名称
    pub name: String,
    /// 插件编码
    pub code: String,
    /// 安装状态（0: 成功, 1: 失败）
    pub status: i32,
    /// 安装消息
    pub message: String,
    /// 安装时间
    pub install_time: chrono::DateTime<chrono::Utc>,
}

/// 插件卸载请求
#[derive(Debug, Deserialize)]
pub struct UninstallPluginRequest {
    /// 插件ID
    pub id: i64,
}

/// 插件卸载响应
#[derive(Debug, Serialize)]
pub struct UninstallPluginResponse {
    /// 插件ID
    pub id: i64,
    /// 插件名称
    pub name: String,
    /// 卸载状态（0: 成功, 1: 失败）
    pub status: i32,
    /// 卸载消息
    pub message: String,
    /// 卸载时间
    pub uninstall_time: chrono::DateTime<chrono::Utc>,
}

/// 插件启用请求
#[derive(Debug, Deserialize)]
pub struct EnablePluginRequest {
    /// 插件ID
    pub id: i64,
}

/// 插件启用响应
#[derive(Debug, Serialize)]
pub struct EnablePluginResponse {
    /// 插件ID
    pub id: i64,
    /// 插件名称
    pub name: String,
    /// 状态
    pub status: i32,
    /// 消息
    pub message: String,
    /// 时间
    pub time: chrono::DateTime<chrono::Utc>,
}

/// 插件禁用请求
#[derive(Debug, Deserialize)]
pub struct DisablePluginRequest {
    /// 插件ID
    pub id: i64,
}

/// 插件禁用响应
#[derive(Debug, Serialize)]
pub struct DisablePluginResponse {
    /// 插件ID
    pub id: i64,
    /// 插件名称
    pub name: String,
    /// 状态
    pub status: i32,
    /// 消息
    pub message: String,
    /// 时间
    pub time: chrono::DateTime<chrono::Utc>,
}

/// 插件更新请求
#[derive(Debug, Deserialize)]
pub struct UpdatePluginRequest {
    /// 插件ID
    pub id: i64,
    /// 插件名称
    pub name: Option<String>,
    /// 插件描述
    pub description: Option<String>,
    /// 插件排序
    pub sort_order: Option<i32>,
}

/// 插件更新响应
#[derive(Debug, Serialize)]
pub struct UpdatePluginResponse {
    /// 插件ID
    pub id: i64,
    /// 插件名称
    pub name: String,
    /// 更新时间
    pub updated_time: chrono::DateTime<chrono::Utc>,
}

/// 插件统计
#[derive(Debug, Serialize, Deserialize)]
pub struct PluginStatistics {
    /// 插件总数
    pub total_count: usize,
    /// 启用插件数
    pub enabled_count: usize,
    /// 禁用插件数
    pub disabled_count: usize,
    /// 已卸载插件数
    pub uninstalled_count: usize,
    /// 系统插件数
    pub system_count: usize,
    /// 第三方插件数
    pub custom_count: usize,
    /// 按类型统计
    pub type_stats: Vec<PluginTypeStat>,
}

/// 插件类型统计
#[derive(Debug, Serialize, Deserialize)]
pub struct PluginTypeStat {
    /// 插件类型
    pub plugin_type: i32,
    /// 插件类型名称
    pub plugin_type_name: String,
    /// 插件数量
    pub count: usize,
}

/// 插件配置查询 DTO
#[derive(Debug, Deserialize, Validate, Default)]
pub struct PluginConfigPaginationQuery {
    /// 插件ID
    pub plugin_id: Option<i64>,

    /// 页码（从1开始）
    #[validate(range(min = 1))]
    pub page: Option<usize>,

    /// 每页数量
    #[validate(range(min = 1, max = 100))]
    pub size: Option<usize>,
}

/// 插件配置分页响应
#[derive(Debug, Serialize)]
pub struct PluginConfigPaginationResponse {
    /// 配置列表
    pub list: Vec<PluginConfigItem>,

    /// 总数量
    pub total: usize,

    /// 当前页码
    pub page: usize,

    /// 每页数量
    pub size: usize,

    /// 总页数
    pub pages: usize,
}

/// 插件配置项
#[derive(Debug, Serialize, Deserialize)]
pub struct PluginConfigItem {
    /// 配置ID
    pub id: i64,
    /// 插件ID
    pub plugin_id: i64,
    /// 配置项名称
    pub config_key: String,
    /// 配置项值
    pub config_value: String,
    /// 配置项描述
    pub description: Option<String>,
    /// 配置类型
    pub value_type: i32,
    /// 配置类型名称
    pub value_type_name: String,
    /// 是否必填
    pub is_required: i32,
    /// 默认值
    pub default_value: Option<String>,
    /// 创建时间
    pub created_time: chrono::DateTime<chrono::Utc>,
    /// 更新时间
    pub updated_time: chrono::DateTime<chrono::Utc>,
}

/// 插件配置更新请求
#[derive(Debug, Deserialize)]
pub struct UpdatePluginConfigRequest {
    /// 配置ID
    pub id: i64,
    /// 配置项值
    pub config_value: String,
}

/// 插件配置更新响应
#[derive(Debug, Serialize)]
pub struct UpdatePluginConfigResponse {
    /// 配置ID
    pub id: i64,
    /// 配置项名称
    pub config_key: String,
    /// 更新时间
    pub updated_time: chrono::DateTime<chrono::Utc>,
}
