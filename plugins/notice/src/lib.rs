//! 通知公告插件
//! 提供系统通知和公告管理功能，与Python版本完全对齐
//! 
//! # 功能特性
//! - 通知公告管理（9个API端点）
//! - 完整的CRUD操作
//! - 发布/撤回状态管理
//! - 置顶功能
//! - 数据验证
//! - 分页查询
//! 
//! # API端点
//! - GET /all - 获取所有通知
//! - GET /{pk} - 获取单个通知
//! - GET /published - 获取已发布通知（公开）
//! - GET / - 分页查询
//! - POST / - 创建通知
//! - PUT /{pk} - 更新通知
//! - PUT /{pk}/publish - 发布通知
//! - PUT /{pk}/revoke - 撤回通知
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
    name: "notice",
    version: "0.0.2",
    description: "通知公告 - 发布系统内部通知、公告",
    author: "wu-clan",
};

#[derive(Debug, Clone)]
pub struct PluginInfo {
    pub name: &'static str,
    pub version: &'static str,
    pub description: &'static str,
    pub author: &'static str,
}

/// 通知公告插件
pub struct NoticePlugin;

impl NoticePlugin {
    /// 获取插件信息
    pub fn info() -> PluginInfo {
        PLUGIN_INFO.clone()
    }

    /// 创建插件路由
    /// 
    /// 注意：这个路由会被注入到 /api/v1/sys/notices 路径下
    /// 因为这是一个 extend='admin' 类型的插件
    /// 所以这里直接返回notice_routes，不需要再包一层/notices前缀
    pub fn create_router(db: DatabaseConnection) -> Router {
        api::notice_routes().with_state(db)
    }
}

// 导出公共类型
pub use dto::{
    NoticeDetail,
    CreateNoticeParam, UpdateNoticeParam,
    NoticeQuery,
    PaginationQuery, PageData, ApiResponse,
};
pub use error::NoticeError;
pub use service::NoticeService;
