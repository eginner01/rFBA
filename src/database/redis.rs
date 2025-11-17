/// Redis管理器模块
/// 提供Redis连接池和基础操作

use redis::aio::ConnectionManager;
use redis::Client;
use crate::common::exception::{AppError, ErrorCode};
use once_cell::sync::OnceCell;
use tokio::sync::RwLock;

/// 全局Redis客户端
static REDIS_CLIENT: OnceCell<Client> = OnceCell::new();
static REDIS_CONNECTION: OnceCell<RwLock<Option<ConnectionManager>>> = OnceCell::new();

/// Redis管理器
pub struct RedisManager;

impl RedisManager {
    /// 初始化Redis客户端
    pub fn init(redis_url: &str) -> Result<(), AppError> {
        let client = Client::open(redis_url).map_err(|e| {
            tracing::error!("Failed to create Redis client: {:?}", e);
            AppError::with_message(ErrorCode::RedisError, "Failed to create Redis client")
        })?;

        REDIS_CLIENT.set(client).map_err(|_| {
            AppError::with_message(ErrorCode::RedisError, "Redis client already initialized")
        })?;

        REDIS_CONNECTION.set(RwLock::new(None)).map_err(|_| {
            AppError::with_message(ErrorCode::RedisError, "Redis connection already initialized")
        })?;

        Ok(())
    }

    /// 获取Redis连接
    pub async fn get_connection() -> Result<ConnectionManager, AppError> {
        let conn_lock = REDIS_CONNECTION.get()
            .ok_or_else(|| {
                AppError::with_message(ErrorCode::RedisError, "Redis not initialized")
            })?;

        let mut conn_guard = conn_lock.write().await;

        // 如果连接不存在或已断开，创建新连接
        if conn_guard.is_none() {
            let client = REDIS_CLIENT.get()
                .ok_or_else(|| {
                    AppError::with_message(ErrorCode::RedisError, "Redis client not initialized")
                })?;

            let conn = ConnectionManager::new(client.clone())
                .await
                .map_err(|e| {
                    tracing::error!("Failed to get Redis connection: {:?}", e);
                    AppError::with_message(ErrorCode::RedisError, "Failed to connect to Redis")
                })?;

            *conn_guard = Some(conn);
        }

        Ok(conn_guard.as_ref().unwrap().clone())
    }

    /// 获取Redis客户端
    pub fn get_client() -> Result<&'static Client, AppError> {
        REDIS_CLIENT.get().ok_or_else(|| {
            AppError::with_message(ErrorCode::RedisError, "Redis client not initialized")
        })
    }
}

#[cfg(test)]
mod tests {
    

    #[test]
    fn test_redis_manager_init() {
        // 测试需要实际的Redis连接
        // 这里只是示例
    }
}
