//! 邮件发送插件
//! 提供SMTP邮件发送、模板邮件、发送记录管理功能，与Python版本完全对齐
//! 
//! # 功能特性
//! - 邮件发送系统（5个API端点）
//! - SMTP配置管理
//! - 邮件模板系统
//! - 异步发送队列
//! - 发送记录管理
//! 
//! # API端点
//! - POST /send - 发送邮件
//! - POST /send-template - 发送模板邮件
//! - POST /test-smtp - 测试SMTP配置
//! - GET /records - 查询发送记录
//! - GET /records/{id} - 获取记录详情

pub mod entity;
pub mod dto;
pub mod service;
pub mod api;
pub mod error;

use axum::Router;
use sea_orm::DatabaseConnection;

/// 插件信息
pub const PLUGIN_INFO: PluginInfo = PluginInfo {
    name: "email",
    version: "0.0.2",
    description: "电子邮件 - SMTP邮件发送、模板邮件、发送记录管理",
    author: "wu-clan",
};

#[derive(Debug, Clone)]
pub struct PluginInfo {
    pub name: &'static str,
    pub version: &'static str,
    pub description: &'static str,
    pub author: &'static str,
}

/// 邮件插件
pub struct EmailPlugin;

impl EmailPlugin {
    /// 获取插件信息
    pub fn info() -> PluginInfo {
        PLUGIN_INFO.clone()
    }

    /// 创建插件路由
    /// 
    /// 注意：这是一个独立路由插件 (router = ['v1'])
    pub fn create_router(db: DatabaseConnection, smtp_config: SmtpConfig) -> Router {
        let state = api::AppState { db, smtp_config };
        
        Router::new()
            .nest("/email", api::email_routes())
            .with_state(state)
    }
}

// 导出公共类型
pub use dto::{
    EmailRecordDetail,
    SendEmailParam, SendTemplateEmailParam, TestSmtpParam,
    EmailRecordQuery,
    PaginationQuery, PageData, ApiResponse,
};
pub use error::EmailError;
pub use service::{EmailService, SmtpConfig};
pub use api::AppState;
