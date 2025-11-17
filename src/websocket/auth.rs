/// WebSocket 认证模块
/// 处理 Socket.IO 连接的 JWT 认证和免授权直连

use serde::{Deserialize, Serialize};
use tracing::{info, warn};
use redis::AsyncCommands;
use crate::common::exception::{AppError, ErrorCode};
use crate::core::SETTINGS;
use crate::database::redis::RedisManager;

/// Socket.IO 认证信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocketAuth {
    pub token: String,
    pub session_uuid: String,
}

/// 验证 Socket.IO 连接认证
pub async fn authenticate_socket(auth: Option<SocketAuth>) -> Result<String, AppError> {
    // 检查认证信息
    let auth = auth.ok_or_else(|| {
        warn!("WebSocket 连接失败：无授权信息");
        AppError::with_message(ErrorCode::Unauthorized, "无授权信息")
    })?;

    let session_uuid = auth.session_uuid.clone();
    let token = auth.token.as_str();

    // 检查必需字段
    if token.is_empty() || session_uuid.is_empty() {
        warn!("WebSocket 连接失败：授权信息不完整");
        return Err(AppError::with_message(ErrorCode::Unauthorized, "授权信息不完整"));
    }

    // 免授权直连（用于测试或特殊场景）
    if token == SETTINGS.ws_no_auth_marker.as_deref().unwrap_or("") {
        info!("WebSocket 免授权直连: session={}", session_uuid);
        add_online_user(&session_uuid).await?;
        return Ok(session_uuid);
    }

    // JWT 令牌验证（简单验证，实际生产中应该解码并验证）
    // TODO: 实现完整的 JWT 验证逻辑
    if token.is_empty() {
        warn!("WebSocket JWT 为空");
        return Err(AppError::with_message(ErrorCode::Unauthorized, "JWT 验证失败"));
    }

    info!("WebSocket 连接成功: session={}", session_uuid);
    add_online_user(&session_uuid).await?;
    Ok(session_uuid)
}

/// 添加在线用户到 Redis
async fn add_online_user(session_uuid: &str) -> Result<(), AppError> {
    let mut redis_conn = RedisManager::get_connection().await
        .map_err(|e| AppError::with_details(
            ErrorCode::RedisError,
            "获取 Redis 连接失败",
            e.to_string()
        ))?;

    let key = format!("{}:{}", SETTINGS.token_online_redis_prefix, session_uuid);
    
    // 设置在线状态，过期时间 24 小时
    redis_conn.set_ex::<_, _, ()>(&key, "1", 86400).await
        .map_err(|e| AppError::with_details(
            ErrorCode::RedisError,
            "设置在线状态失败",
            e.to_string()
        ))?;

    Ok(())
}

/// 移除在线用户
pub async fn remove_online_user(session_uuid: &str) -> Result<(), AppError> {
    let mut redis_conn = RedisManager::get_connection().await
        .map_err(|e| AppError::with_details(
            ErrorCode::RedisError,
            "获取 Redis 连接失败",
            e.to_string()
        ))?;

    let key = format!("{}:{}", SETTINGS.token_online_redis_prefix, session_uuid);
    
    let _: Result<(), _> = redis_conn.del(&key).await;
    
    Ok(())
}

/// 检查用户是否在线
#[allow(dead_code)]
pub async fn is_user_online(session_uuid: &str) -> Result<bool, AppError> {
    let mut redis_conn = RedisManager::get_connection().await
        .map_err(|e| AppError::with_details(
            ErrorCode::RedisError,
            "获取 Redis 连接失败",
            e.to_string()
        ))?;

    let key = format!("{}:{}", SETTINGS.token_online_redis_prefix, session_uuid);
    
    let exists: bool = redis_conn.exists(&key).await
        .map_err(|e| AppError::with_details(
            ErrorCode::RedisError,
            "检查在线状态失败",
            e.to_string()
        ))?;

    Ok(exists)
}
