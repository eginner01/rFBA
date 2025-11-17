//! 字典管理插件
//! 提供系统数据字典管理功能，与Python版本完全对齐
//! 
//! # 功能特性
//! - 字典类型管理（7个API端点）
//! - 字典数据管理（8个API端点）
//! - 完整的CRUD操作
//! - 数据验证
//! - 分页查询
//! 
//! # API端点
//! ## 字典类型 (dict-types)
//! - GET /all - 获取所有类型
//! - GET /{pk} - 获取单个类型
//! - GET / - 分页查询
//! - POST / - 创建类型
//! - PUT /{pk} - 更新类型
//! - DELETE / - 批量删除
//! 
//! ## 字典数据 (dict-datas)
//! - GET /all - 获取所有数据
//! - GET /{pk} - 获取单个数据
//! - GET /type-codes/{code} - 根据类型编码获取
//! - GET / - 分页查询
//! - POST / - 创建数据
//! - PUT /{pk} - 更新数据
//! - DELETE / - 批量删除

pub mod entity;
pub mod dto;
pub mod service;
pub mod api;
pub mod error;

use axum::Router;
use sea_orm::DatabaseConnection;

/// 插件信息
pub const PLUGIN_INFO: PluginInfo = PluginInfo {
    name: "dict",
    version: "0.0.8",
    description: "数据字典 - 通常用于约束前端工程数据展示",
    author: "wu-clan",
};

#[derive(Debug, Clone)]
pub struct PluginInfo {
    pub name: &'static str,
    pub version: &'static str,
    pub description: &'static str,
    pub author: &'static str,
}

/// 字典插件
pub struct DictPlugin;

impl DictPlugin {
    /// 获取插件信息
    pub fn info() -> PluginInfo {
        PLUGIN_INFO.clone()
    }

    /// 创建插件路由
    /// 
    /// 注意：这个路由会被注入到 /api/v1/sys/ 路径下
    /// 因为这是一个 extend='admin' 类型的插件
    pub fn create_router(db: DatabaseConnection) -> Router {
        Router::new()
            .nest("/dict-types", api::dict_type_routes())
            .nest("/dict-datas", api::dict_data_routes())
            .with_state(db)
    }
}

// 导出公共类型
pub use dto::{
    DictTypeDetail, DictDataDetail,
    CreateDictTypeParam, UpdateDictTypeParam,
    CreateDictDataParam, UpdateDictDataParam,
    DictTypeQuery, DictDataQuery,
    PaginationQuery, PageData, ApiResponse,
};
pub use error::DictError;
pub use service::{DictTypeService, DictDataService};
