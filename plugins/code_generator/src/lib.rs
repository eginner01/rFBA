//! 代码生成器插件
//! 提供数据库表扫描和CRUD代码生成功能，与Python版本完全对齐
//! 
//! # 功能特性
//! - 代码生成器（6个API端点，完整实现）
//! - 数据库表扫描（MySQL/PostgreSQL）
//! - 字段信息自动提取
//! - 模板代码生成（Entity/DTO/Service/API）
//! - ZIP打包下载
//! - 类型智能映射
//! 
//! # API端点
//! - GET /tables - 获取数据库所有表
//! - GET /tables/{name}/columns - 获取表字段信息
//! - GET /templates - 获取可用模板列表
//! - GET /preview - 预览生成代码
//! - POST /generate - 生成代码
//! - GET /download - 下载代码ZIP

pub mod entity;
pub mod dto;
pub mod service;
pub mod api;
pub mod error;

use axum::Router;
use sea_orm::DatabaseConnection;

/// 插件信息
pub const PLUGIN_INFO: PluginInfo = PluginInfo {
    name: "code_generator",
    version: "0.0.6",
    description: "代码生成器 - 数据库表扫描、模板代码生成、ZIP打包下载",
    author: "wu-clan",
};

#[derive(Debug, Clone)]
pub struct PluginInfo {
    pub name: &'static str,
    pub version: &'static str,
    pub description: &'static str,
    pub author: &'static str,
}

/// 代码生成器插件
pub struct CodeGeneratorPlugin;

impl CodeGeneratorPlugin {
    /// 获取插件信息
    pub fn info() -> PluginInfo {
        PLUGIN_INFO.clone()
    }

    /// 创建插件路由
    /// 注意：这是一个独立路由插件 (router = ['v1'])
    /// Python版本路由：/api/v1/generates/{businesses,codes,columns}
    pub fn create_router(db: DatabaseConnection) -> Router {
        let state = api::AppState { db };
        
        Router::new()
            .nest("/businesses", api::business_routes())
            .nest("/codes", api::codegen_routes())
            .with_state(state)
    }
}

// 导出公共类型
pub use dto::{
    TableInfo, ColumnInfo, GenerateCodeParam, CodePreview, ImportTableParam,
    GenBusinessDetail, CreateGenBusinessParam, UpdateGenBusinessParam, GenColumnDetail,
    PaginationQuery, PageData, ApiResponse,
};
pub use error::CodeGenError;
pub use service::{CodeGenService, BusinessService};
pub use api::AppState;
