/// 插件服务模块

// 旧的基于数据库的服务（暂时禁用）
// pub mod plugin_service;

// 新的基于文件和Redis的服务
pub mod plugin_service_new;

pub use plugin_service_new::*;
