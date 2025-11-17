//! OAuth2认证插件
//! 提供GitHub/Google/LinuxDo第三方登录功能，与Python版本完全对齐
//! 
//! # 功能特性
//! - OAuth2第三方登录（8个API端点）
//! - GitHub OAuth2集成（完整实现）
//! - Google OAuth2集成（完整实现）
//! - LinuxDo OAuth2集成（完整实现）✨
//! - 用户绑定/解绑管理
//! - Token自动管理
//! - 数据库持久化
//! 
//! # API端点
//! - GET /github/authorize - GitHub授权重定向
//! - GET /github/callback - GitHub回调处理
//! - GET /google/authorize - Google授权重定向
//! - GET /google/callback - Google回调处理
//! - GET /linux-do/authorize - LinuxDo授权重定向 ✨
//! - GET /linux-do/callback - LinuxDo回调处理 ✨
//! - POST /bind - 绑定第三方账号
//! - DELETE /unbind - 解绑第三方账号

pub mod entity;
pub mod dto;
pub mod service;
pub mod api;
pub mod error;

use axum::Router;
use sea_orm::DatabaseConnection;

/// 插件信息
pub const PLUGIN_INFO: PluginInfo = PluginInfo {
    name: "oauth2",
    version: "0.1.0",
    description: "OAuth2 - 第三方登录（GitHub、Google、LinuxDo，完整实现）",
    author: "wu-clan",
};

#[derive(Debug, Clone)]
pub struct PluginInfo {
    pub name: &'static str,
    pub version: &'static str,
    pub description: &'static str,
    pub author: &'static str,
}

/// OAuth2插件
pub struct OAuth2Plugin;

impl OAuth2Plugin {
    /// 获取插件信息
    pub fn info() -> PluginInfo {
        PLUGIN_INFO.clone()
    }

    /// 创建插件路由
    /// 注意：这是一个独立路由插件 (router = ['v1'])
    pub fn create_router(db: DatabaseConnection, oauth2_config: OAuth2Config) -> Router {
        let state = api::AppState { db, oauth2_config };
        
        Router::new()
            .nest("/oauth2", api::oauth2_routes())
            .with_state(state)
    }
}

// 导出公共类型
pub use dto::{
    OAuthBindInfo, BindOAuthParam, UnbindOAuthParam,
    OAuthCallbackResponse, OAuthUserInfo, ApiResponse,
};
pub use error::OAuth2Error;
pub use service::{OAuth2Service, OAuth2Config};
pub use api::AppState;
