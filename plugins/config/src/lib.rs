//! 配置管理插件
//! 提供系统参数配置管理功能，与Python版本完全对齐
//! 
//! # 功能特性
//! - 系统配置管理（8个API端点）
//! - 完整的CRUD操作
//! - 数据验证
//! - Redis缓存支持
//! - 分页查询
//! 
//! # API端点
//! - GET /all - 获取所有配置
//! - GET /{pk} - 获取单个配置
//! - GET /key/{key} - 根据key获取配置（带缓存）
//! - GET / - 分页查询
//! - POST / - 创建配置
//! - PUT /{pk} - 更新配置
//! - DELETE / - 批量删除
//! - POST /refresh - 刷新缓存

pub mod entity;
pub mod dto;
pub mod service;
pub mod api;
pub mod error;

use axum::Router;
use sea_orm::DatabaseConnection;

/// 插件信息
pub const PLUGIN_INFO: PluginInfo = PluginInfo {
    name: "config",
    version: "0.0.2",
    description: "参数配置 - 通常用于动态配置系统参数/前端工程数据展示",
    author: "wu-clan",
};

#[derive(Debug, Clone)]
pub struct PluginInfo {
    pub name: &'static str,
    pub version: &'static str,
    pub description: &'static str,
    pub author: &'static str,
}

/// 配置管理插件
pub struct ConfigPlugin;

impl ConfigPlugin {
    /// 获取插件信息
    pub fn info() -> PluginInfo {
        PLUGIN_INFO.clone()
    }

    /// 创建插件路由
    /// 
    /// 注意：这个路由会被注入到 /api/v1/sys/configs 路径下
    /// 因为这是一个 extend='admin' 类型的插件
    /// 所以这里直接返回config_routes，不需要再包一层/configs前缀
    pub fn create_router(
        db: DatabaseConnection,
        redis: redis::aio::ConnectionManager,
    ) -> Router {
        let state = api::AppState { db, redis };
        
        api::config_routes().with_state(state)
    }
}

// 导出公共类型
pub use dto::{
    ConfigDetail,
    CreateConfigParam, UpdateConfigParam,
    ConfigQuery,
    PaginationQuery, PageData, ApiResponse,
};
pub use error::ConfigError;
pub use service::ConfigService;
pub use api::AppState;
