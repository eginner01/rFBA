//! 插件系统初始化
//! 在应用启动时扫描plugins目录，加载plugin.toml配置，并缓存到Redis

use std::fs;
use std::path::Path;
use serde_json::json;
use redis::AsyncCommands;
use tracing::{info, warn, error};

use crate::common::exception::{AppError, ErrorCode};
use crate::database::redis::RedisManager;
use crate::core::SETTINGS;

/// 插件配置结构（用于解析plugin.toml）
#[derive(Debug, Clone, serde::Deserialize)]
pub struct PluginTomlConfig {
    pub plugin: PluginInfo,
    #[serde(default)]
    pub app: Option<AppConfig>,
    #[serde(default)]
    pub api: Option<serde_json::Value>,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct PluginInfo {
    pub summary: String,
    pub version: String,
    pub description: String,
    pub author: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct AppConfig {
    #[serde(default)]
    pub extend: Option<String>,
    #[serde(default)]
    pub router: Option<Vec<String>>,
}

/// 初始化所有插件
/// 扫描plugins目录，读取plugin.toml配置，并将信息缓存到Redis
pub async fn init_plugins() -> Result<(), AppError> {
    info!("正在初始化插件系统...");
    
    let plugin_dir = Path::new("./plugins");
    
    // 检查plugins目录是否存在
    if !plugin_dir.exists() {
        warn!("插件目录不存在，正在创建: {:?}", plugin_dir);
        fs::create_dir_all(plugin_dir).map_err(|e| {
            AppError::with_message(ErrorCode::IOError, format!("Failed to create plugins directory: {}", e))
        })?;
        info!("插件系统已初始化（未找到插件）");
        return Ok(());
    }
    
    // 获取Redis连接
    let mut redis_conn = RedisManager::get_connection().await
        .map_err(|e| AppError::with_message(ErrorCode::RedisError, format!("Failed to get Redis connection: {}", e)))?;
    
    let prefix = &SETTINGS.plugin_redis_prefix;
    let mut loaded_count = 0;
    let mut error_count = 0;
    
    // 清理Redis中的changed标记（如果存在）
    let changed_key = format!("{}:changed", prefix);
    let _: Result<(), redis::RedisError> = redis_conn.del(&changed_key).await;
    
    // 获取现有的所有插件键，用于清理不存在的插件
    let existing_keys: Vec<String> = redis_conn.keys(format!("{}:*", prefix)).await
        .unwrap_or_default();
    
    let mut valid_plugin_keys = vec![];
    
    // 扫描plugins目录
    let entries = fs::read_dir(plugin_dir).map_err(|e| {
        AppError::with_message(ErrorCode::IOError, format!("Failed to read plugins directory: {}", e))
    })?;
    
    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                warn!("Failed to read directory entry: {}", e);
                continue;
            }
        };
        
        let path = entry.path();
        
        // 只处理目录
        if !path.is_dir() {
            continue;
        }
        
        let plugin_name = match path.file_name().and_then(|n| n.to_str()) {
            Some(name) => name,
            None => continue,
        };
        
        // 跳过备份目录和隐藏目录
        if plugin_name.ends_with(".backup") || plugin_name.starts_with('.') || plugin_name == "target" {
            continue;
        }
        
        // 加载插件
        match load_plugin(&path, plugin_name, &mut redis_conn, prefix).await {
            Ok(_) => {
                loaded_count += 1;
                valid_plugin_keys.push(format!("{}:{}", prefix, plugin_name));
            }
            Err(e) => {
                error!("加载插件 '{}' 失败: {}", plugin_name, e);
                error_count += 1;
            }
        }
    }
    
    // 清理Redis中不存在的插件
    for key in existing_keys {
        if key.ends_with(":changed") {
            continue;
        }
        if !valid_plugin_keys.contains(&key) {
            let plugin_name = key.strip_prefix(&format!("{}:", prefix)).unwrap_or(&key);
            warn!("清理已删除的插件: {}", plugin_name);
            let _: Result<(), redis::RedisError> = redis_conn.del(&key).await;
        }
    }
    
    if error_count > 0 {
        warn!("插件系统已初始化，但有 {} 个错误", error_count);
    }
    
    info!("插件系统已初始化: 已加载 {} 个插件", loaded_count);
    
    Ok(())
}

/// 加载单个插件到Redis（公共函数，可被install.rs调用）
pub async fn load_single_plugin(plugin_name: &str) -> Result<(), AppError> {
    let plugin_path = Path::new("./plugins").join(plugin_name);
    let mut redis_conn = RedisManager::get_connection().await
        .map_err(|e| AppError::with_message(ErrorCode::RedisError, format!("Failed to get Redis connection: {}", e)))?;
    let prefix = &SETTINGS.plugin_redis_prefix;
    
    load_plugin(&plugin_path, plugin_name, &mut redis_conn, prefix).await
}

/// 加载单个插件
async fn load_plugin(
    plugin_path: &Path,
    plugin_name: &str,
    redis_conn: &mut redis::aio::ConnectionManager,
    prefix: &str,
) -> Result<(), AppError> {
    // 读取plugin.toml
    let toml_path = plugin_path.join("plugin.toml");
    
    if !toml_path.exists() {
        return Err(AppError::with_message(
            ErrorCode::NotFound,
            format!("plugin.toml not found in {}", plugin_name)
        ));
    }
    
    let toml_content = fs::read_to_string(&toml_path).map_err(|e| {
        AppError::with_message(ErrorCode::IOError, format!("Failed to read plugin.toml: {}", e))
    })?;
    
    // 解析TOML配置
    let config: PluginTomlConfig = toml::from_str(&toml_content).map_err(|e| {
        AppError::with_message(ErrorCode::InvalidInput, format!("Failed to parse plugin.toml: {}", e))
    })?;
    
    // 检查Redis中是否已有该插件的启用状态
    let redis_key = format!("{}:{}", prefix, plugin_name);
    let existing_data: Option<String> = redis_conn.get(&redis_key).await.ok().flatten();
    
    let enable_status = if let Some(data) = existing_data {
        // 尝试解析现有数据获取enable状态
        match serde_json::from_str::<serde_json::Value>(&data) {
            Ok(json_val) => {
                json_val
                    .get("plugin")
                    .and_then(|p| p.get("enable"))
                    .and_then(|e| e.as_str())
                    .unwrap_or("1")
                    .to_string()
            }
            Err(_) => "1".to_string(),
        }
    } else {
        "1".to_string() // 默认启用
    };
    
    // 构建插件数据
    let mut plugin_data = json!({
        "plugin": {
            "summary": config.plugin.summary,
            "version": config.plugin.version,
            "description": config.plugin.description,
            "author": config.plugin.author,
            "enable": enable_status,
            "name": plugin_name,
        }
    });
    
    // 添加app配置（如果存在）
    if let Some(app_config) = config.app {
        let mut app_json = json!({});
        if let Some(extend) = app_config.extend {
            app_json["extend"] = json!(extend);
        }
        if let Some(router) = app_config.router {
            app_json["router"] = json!(router);
        }
        plugin_data["app"] = app_json;
    }
    
    // 添加api配置（如果存在）
    if let Some(api_config) = config.api {
        plugin_data["api"] = api_config;
    }
    
    // 存入Redis
    let plugin_data_str = serde_json::to_string(&plugin_data).map_err(|e| {
        AppError::with_message(ErrorCode::SerializationError, format!("Failed to serialize plugin data: {}", e))
    })?;
    
    redis_conn.set::<_, _, ()>(&redis_key, plugin_data_str).await.map_err(|e| {
        AppError::with_message(ErrorCode::RedisError, format!("Failed to cache plugin data: {}", e))
    })?;
    
    info!("已加载插件: {} (版本 {})", plugin_name, config.plugin.version);
    
    Ok(())
}

/// 重新加载所有插件（用于热重载）
pub async fn reload_plugins() -> Result<(), AppError> {
    info!("正在重新加载插件系统...");
    init_plugins().await?;
    info!("插件系统重新加载完成");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_config_parsing() {
        let toml_str = r#"
[plugin]
summary = "测试插件"
version = "0.0.1"
description = "这是一个测试插件"
author = "test"

[app]
extend = "admin"

[api.test]
prefix = "/test"
tags = "测试标签"
        "#;
        
        let config: PluginTomlConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(config.plugin.summary, "测试插件");
        assert_eq!(config.plugin.version, "0.0.1");
        assert!(config.app.is_some());
        assert!(config.api.is_some());
    }
}
