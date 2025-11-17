/// Complete Module
/// 系统完整性检查和功能汇总模块
/// 提供系统状态检查、模块信息查询、健康检查等功能

pub mod dto;
pub mod service;
pub mod api;
pub mod router;

// 导出 API 处理器
pub use api::{
    get_system_status,
    get_module_info,
    health_check,
    get_system_summary,
    SystemStatusQuery,
};

// 导出服务
pub use service::CompleteService;
