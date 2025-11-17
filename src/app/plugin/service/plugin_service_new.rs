/// 新的插件服务实现
/// 基于文件系统和Redis，不依赖数据库表

use tracing::info;
use crate::common::exception::{AppError, ErrorCode};
use crate::app::plugin::{
    PluginCacheManager, 
    UnifiedPluginManager,
};
use std::sync::Arc;

/// 简化的插件服务
/// 使用UnifiedPluginManager作为核心
pub struct PluginService {
    manager: Arc<UnifiedPluginManager>,
}

impl PluginService {
    /// 创建新的插件服务
    pub fn new(plugin_dir: impl Into<String>) -> Self {
        Self {
            manager: Arc::new(UnifiedPluginManager::new(plugin_dir)),
        }
    }

    /// 获取所有插件
    pub async fn get_all_plugins(&self) -> Vec<PluginInfo> {
        let plugins = self.manager.get_all_plugins().await;
        
        let mut result = Vec::new();
        for plugin in plugins {
            // 获取Redis中的启用状态
            let enabled = PluginCacheManager::is_plugin_enabled(&plugin.name)
                .await
                .unwrap_or(false);
            
            let is_extend = plugin.config.is_extend_plugin();
            let is_app = plugin.config.is_app_plugin();
            
            result.push(PluginInfo {
                name: plugin.name,
                summary: plugin.config.plugin.summary,
                version: plugin.config.plugin.version,
                description: plugin.config.plugin.description,
                author: plugin.config.plugin.author,
                enabled,
                is_extend,
                is_app,
            });
        }
        
        result
    }

    /// 获取特定插件详情
    pub async fn get_plugin(&self, name: &str) -> Result<PluginInfo, AppError> {
        let plugin = self.manager.get_plugin(name).await?
            .ok_or_else(|| AppError::with_message(
                ErrorCode::NotFound,
                format!("Plugin {} not found", name)
            ))?;
        
        let enabled = PluginCacheManager::is_plugin_enabled(name).await?;
        let is_extend = plugin.config.is_extend_plugin();
        let is_app = plugin.config.is_app_plugin();
        
        Ok(PluginInfo {
            name: plugin.name,
            summary: plugin.config.plugin.summary,
            version: plugin.config.plugin.version,
            description: plugin.config.plugin.description,
            author: plugin.config.plugin.author,
            enabled,
            is_extend,
            is_app,
        })
    }

    /// 启用插件
    pub async fn enable_plugin(&self, name: &str) -> Result<(), AppError> {
        info!("Enabling plugin: {}", name);
        self.manager.set_plugin_status(name, true).await?;
        Ok(())
    }

    /// 禁用插件
    pub async fn disable_plugin(&self, name: &str) -> Result<(), AppError> {
        info!("Disabling plugin: {}", name);
        self.manager.set_plugin_status(name, false).await?;
        Ok(())
    }

    /// 重新加载所有插件
    pub async fn reload_plugins(&self) -> Result<(), AppError> {
        info!("Reloading all plugins");
        self.manager.reload().await?;
        Ok(())
    }

    /// 检查插件是否需要重载
    pub async fn needs_reload(&self) -> Result<bool, AppError> {
        self.manager.needs_reload().await
    }

    /// 初始化插件系统
    pub async fn initialize(&self) -> Result<(), AppError> {
        info!("Initializing plugin system");
        self.manager.initialize().await?;
        Ok(())
    }
}

/// 插件信息
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PluginInfo {
    pub name: String,
    pub summary: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub enabled: bool,
    pub is_extend: bool,
    pub is_app: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_plugin_service() {
        let service = PluginService::new("./plugins");
        // 测试基本功能
    }
}
