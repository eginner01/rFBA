/// 模块信息响应 DTO

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModuleInfoResponse {
    /// 系统名称
    pub system_name: String,
    /// 系统版本
    pub system_version: String,
    /// 模块总数
    pub total_modules: u32,
    /// 已实现模块数
    pub implemented_modules: u32,
    /// 未实现模块数
    pub unimplemented_modules: u32,
    /// 模块列表
    pub modules: Vec<ModuleInfo>,
    /// 获取时间
    pub fetch_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModuleInfo {
    /// 模块名称
    pub name: String,
    /// 模块路径
    pub path: String,
    /// 模块状态
    pub status: String,
    /// 模块描述
    pub description: String,
    /// 是否已实现
    pub implemented: bool,
    /// API端点数量
    pub api_endpoints: u32,
    /// 创建时间
    pub created_time: Option<String>,
    /// 最后更新时间
    pub updated_time: Option<String>,
}
