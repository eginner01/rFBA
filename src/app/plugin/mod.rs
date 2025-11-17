// 插件系统核心模块
pub mod config;
pub mod discovery;
pub mod cache;
pub mod middleware;
pub mod injector;
pub mod manager;
pub mod dynamic_loader;
pub mod signature;
pub mod sandbox;
pub mod metrics;
pub mod init;  // 插件初始化模块
pub mod install;  // 插件安装模块

// API和路由模块
pub mod api;
pub mod router;
pub mod dto;
pub mod service;
pub mod loader;
// pub mod hot_reload;  // 暂时禁用热重载模块，稍后重新实现

// 导出核心类型
pub use config::{PluginConfig, PluginInfo, AppConfig, ApiConfig};
pub use discovery::{PluginDiscovery, DiscoveredPlugin};
pub use cache::{PluginCacheManager, PluginCacheInfo};
pub use middleware::check_plugin_status;
pub use injector::RouteInjector;
pub use manager::UnifiedPluginManager;
pub use init::{init_plugins, reload_plugins};  // 导出初始化函数
pub use install::{install_zip_plugin, install_git_plugin, InstallResult};  // 导出安装函数

// 导出API和路由
pub use api::*;
pub use router::*;
