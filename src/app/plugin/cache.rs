/// 插件缓存管理模块
/// 使用Redis缓存插件状态和配置信息

use serde::{Deserialize, Serialize};
use crate::common::exception::{AppError, ErrorCode};
use crate::app::plugin::config::PluginConfig;
use crate::database::redis::RedisManager;
use crate::core::SETTINGS;
use redis::AsyncCommands;

/// 插件缓存信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginCacheInfo {
    /// 插件配置
    pub config: PluginConfig,
    /// 是否启用
    pub enabled: bool,
}

/// 插件缓存管理器
pub struct PluginCacheManager;

impl PluginCacheManager {
    /// 获取插件缓存key
    fn get_plugin_key(plugin_name: &str) -> String {
        format!("{}:{}", SETTINGS.plugin_redis_prefix, plugin_name)
    }
    
    /// 获取插件变更标记key
    fn get_changed_key() -> String {
        format!("{}:changed", SETTINGS.plugin_redis_prefix)
    }

    /// 缓存插件信息
    pub async fn cache_plugin(
        plugin_name: &str,
        config: &PluginConfig,
    ) -> Result<(), AppError> {
        let mut conn = RedisManager::get_connection().await.map_err(|e| {
            tracing::error!("Failed to get Redis connection: {:?}", e);
            AppError::with_message(ErrorCode::RedisError, "Failed to connect to Redis")
        })?;

        let cache_info = PluginCacheInfo {
            config: config.clone(),
            enabled: config.plugin.enable == 1,
        };

        let cache_value = serde_json::to_string(&cache_info).map_err(|e| {
            tracing::error!("Failed to serialize plugin info: {:?}", e);
            AppError::with_message(ErrorCode::SerializationError, "Failed to serialize plugin info")
        })?;

        let key = Self::get_plugin_key(plugin_name);
        
        conn.set::<_, _, ()>(&key, cache_value)
            .await
            .map_err(|e| {
                tracing::error!("Failed to cache plugin info: {:?}", e);
                AppError::with_message(ErrorCode::RedisError, "Failed to cache plugin info")
            })?;

        tracing::debug!("Cached plugin info for: {}", plugin_name);
        Ok(())
    }

    /// 获取插件缓存信息
    pub async fn get_plugin_cache(plugin_name: &str) -> Result<Option<PluginCacheInfo>, AppError> {
        let mut conn = RedisManager::get_connection().await.map_err(|e| {
            tracing::error!("Failed to get Redis connection: {:?}", e);
            AppError::with_message(ErrorCode::RedisError, "Failed to connect to Redis")
        })?;

        let key = Self::get_plugin_key(plugin_name);
        
        let cache_value: Option<String> = conn.get(&key)
            .await
            .map_err(|e| {
                tracing::error!("Failed to get plugin cache: {:?}", e);
                AppError::with_message(ErrorCode::RedisError, "Failed to get plugin cache")
            })?;

        match cache_value {
            Some(value) => {
                // 解析插件数据
                let json_value: serde_json::Value = serde_json::from_str(&value).map_err(|e| {
                    tracing::error!("Failed to parse plugin JSON: {:?}", e);
                    AppError::with_message(ErrorCode::SerializationError, "Failed to parse plugin JSON")
                })?;
                
                // 提取enable状态
                let enabled = json_value
                    .get("plugin")
                    .and_then(|p| p.get("enable"))
                    .and_then(|e| e.as_str())
                    .map(|s| s == "1")
                    .unwrap_or(false);
                
                // 构建简化的PluginCacheInfo（暂时不需要完整的config）
                let cache_info = PluginCacheInfo {
                    config: PluginConfig::default(), // 使用默认值，因为不需要完整配置
                    enabled,
                };
                
                Ok(Some(cache_info))
            }
            None => Ok(None),
        }
    }

    /// 检查插件是否启用
    pub async fn is_plugin_enabled(plugin_name: &str) -> Result<bool, AppError> {
        match Self::get_plugin_cache(plugin_name).await? {
            Some(cache_info) => Ok(cache_info.enabled),
            None => {
                tracing::warn!("Plugin {} not found in cache", plugin_name);
                Ok(false)
            }
        }
    }

    /// 更新插件启用状态
    pub async fn update_plugin_status(
        plugin_name: &str,
        enabled: bool,
    ) -> Result<(), AppError> {
        let mut conn = RedisManager::get_connection().await.map_err(|e| {
            tracing::error!("Failed to get Redis connection: {:?}", e);
            AppError::with_message(ErrorCode::RedisError, "Failed to connect to Redis")
        })?;

        let key = Self::get_plugin_key(plugin_name);
        
        // 获取现有数据
        let existing_data: Option<String> = conn.get(&key).await.map_err(|e| {
            tracing::error!("Failed to get plugin data: {:?}", e);
            AppError::with_message(ErrorCode::RedisError, "Failed to get plugin data")
        })?;
        
        let existing_data = existing_data.ok_or_else(|| {
            AppError::with_message(ErrorCode::NotFound, "Plugin not found in cache")
        })?;
        
        // 解析并更新enable字段
        let mut json_value: serde_json::Value = serde_json::from_str(&existing_data).map_err(|e| {
            tracing::error!("Failed to parse plugin data: {:?}", e);
            AppError::with_message(ErrorCode::SerializationError, "Failed to parse plugin data")
        })?;
        
        // 更新enable状态："0"或"1"
        if let Some(plugin) = json_value.get_mut("plugin") {
            if let Some(obj) = plugin.as_object_mut() {
                obj.insert("enable".to_string(), serde_json::Value::String(if enabled { "1" } else { "0" }.to_string()));
            }
        }
        
        // 写回Redis
        let updated_data = serde_json::to_string(&json_value).map_err(|e| {
            tracing::error!("Failed to serialize updated plugin data: {:?}", e);
            AppError::with_message(ErrorCode::SerializationError, "Failed to serialize plugin data")
        })?;
        
        conn.set::<_, _, ()>(&key, updated_data).await.map_err(|e| {
            tracing::error!("Failed to update plugin status: {:?}", e);
            AppError::with_message(ErrorCode::RedisError, "Failed to update plugin status")
        })?;

        tracing::info!("Updated plugin {} status to: {}", plugin_name, enabled);
        Ok(())
    }

    /// 删除插件缓存
    pub async fn delete_plugin_cache(plugin_name: &str) -> Result<(), AppError> {
        let mut conn = RedisManager::get_connection().await.map_err(|e| {
            tracing::error!("Failed to get Redis connection: {:?}", e);
            AppError::with_message(ErrorCode::RedisError, "Failed to connect to Redis")
        })?;

        let key = Self::get_plugin_key(plugin_name);
        
        conn.del::<_, ()>(&key)
            .await
            .map_err(|e| {
                tracing::error!("Failed to delete plugin cache: {:?}", e);
                AppError::with_message(ErrorCode::RedisError, "Failed to delete plugin cache")
            })?;

        tracing::debug!("Deleted plugin cache for: {}", plugin_name);
        Ok(())
    }

    /// 清除所有插件缓存
    pub async fn clear_all_plugin_caches() -> Result<(), AppError> {
        // 目前简化实现：需要知道所有插件名称
        // 在生产环境中应该使用SCAN命令
        tracing::warn!("clear_all_plugin_caches: simplified implementation");
        
        // TODO: 实现完整的SCAN逻辑
        // 目前只记录警告
        
        Ok(())
    }

    /// 标记插件已变更
    pub async fn mark_plugin_changed() -> Result<(), AppError> {
        let mut conn = RedisManager::get_connection().await.map_err(|e| {
            tracing::error!("Failed to get Redis connection: {:?}", e);
            AppError::with_message(ErrorCode::RedisError, "Failed to connect to Redis")
        })?;

        let changed_key = Self::get_changed_key();
        conn.set::<_, _, ()>(&changed_key, "true")
            .await
            .map_err(|e| {
                tracing::error!("Failed to mark plugin changed: {:?}", e);
                AppError::with_message(ErrorCode::RedisError, "Failed to mark plugin changed")
            })?;

        Ok(())
    }

    /// 检查插件是否已变更
    pub async fn is_plugin_changed() -> Result<bool, AppError> {
        let mut conn = RedisManager::get_connection().await.map_err(|e| {
            tracing::error!("Failed to get Redis connection: {:?}", e);
            AppError::with_message(ErrorCode::RedisError, "Failed to connect to Redis")
        })?;

        let changed_key = Self::get_changed_key();
        let changed: Option<String> = conn.get(&changed_key)
            .await
            .map_err(|e| {
                tracing::error!("Failed to check plugin changed: {:?}", e);
                AppError::with_message(ErrorCode::RedisError, "Failed to check plugin changed")
            })?;

        Ok(changed.is_some())
    }

    /// 重置插件变更状态
    pub async fn reset_plugin_changed() -> Result<(), AppError> {
        let mut conn = RedisManager::get_connection().await.map_err(|e| {
            tracing::error!("Failed to get Redis connection: {:?}", e);
            AppError::with_message(ErrorCode::RedisError, "Failed to connect to Redis")
        })?;

        let changed_key = Self::get_changed_key();
        conn.del::<_, ()>(&changed_key)
            .await
            .map_err(|e| {
                tracing::error!("Failed to reset plugin changed: {:?}", e);
                AppError::with_message(ErrorCode::RedisError, "Failed to reset plugin changed")
            })?;

        tracing::debug!("Reset plugin changed status");
        Ok(())
    }

    /// 批量缓存插件
    pub async fn batch_cache_plugins(
        plugins: Vec<(&str, &PluginConfig)>,
    ) -> Result<(), AppError> {
        for (name, config) in plugins {
            Self::cache_plugin(name, config).await?;
        }
        Ok(())
    }
}
