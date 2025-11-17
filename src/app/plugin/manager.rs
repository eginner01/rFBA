/// 统一插件管理器
/// 整合插件发现、缓存、注入等功能

use axum::Router;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::common::exception::{AppError, ErrorCode};
use super::{
    PluginDiscovery,
    PluginCacheManager,
    RouteInjector,
    DiscoveredPlugin,
};

/// 统一插件管理器
pub struct UnifiedPluginManager {
    /// 插件发现器
    discovery: Arc<PluginDiscovery>,
    /// 路由注入器
    injector: Arc<RouteInjector>,
    /// 插件根目录
    plugin_dir: String,
    /// 已发现的插件缓存
    discovered_plugins: Arc<RwLock<Vec<DiscoveredPlugin>>>,
}

impl UnifiedPluginManager {
    /// 创建新的统一插件管理器
    pub fn new(plugin_dir: impl Into<String>) -> Self {
        let dir = plugin_dir.into();
        Self {
            discovery: Arc::new(PluginDiscovery::new(&dir)),
            injector: Arc::new(RouteInjector::new()),
            plugin_dir: dir,
            discovered_plugins: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// 初始化插件系统
    /// 1. 发现所有插件
    /// 2. 缓存插件信息到Redis
    /// 3. 返回分类后的插件
    pub async fn initialize(&self) -> Result<(), AppError> {
        tracing::info!("Initializing plugin system from: {}", self.plugin_dir);

        // 1. 发现所有插件
        let plugins = self.discovery.discover_all()?;
        tracing::info!("Discovered {} plugins", plugins.len());

        // 2. 缓存插件信息到Redis
        for plugin in &plugins {
            PluginCacheManager::cache_plugin(&plugin.name, &plugin.config).await?;
        }

        // 3. 保存已发现的插件
        let mut discovered = self.discovered_plugins.write().await;
        *discovered = plugins;

        // 4. 重置插件变更状态
        PluginCacheManager::reset_plugin_changed().await?;

        tracing::info!("Plugin system initialized successfully");
        Ok(())
    }

    /// 构建带插件的路由
    /// 先注入扩展级插件，再注入应用级插件
    pub async fn build_router_with_plugins(
        &self,
        base_router: Router,
    ) -> Result<Router, AppError> {
        let plugins = self.discovered_plugins.read().await;
        let (extend_plugins, app_plugins) = self.discovery.classify_plugins(plugins.clone());

        tracing::info!(
            "Building router with {} extend plugins and {} app plugins",
            extend_plugins.len(),
            app_plugins.len()
        );

        // 1. 注入扩展级插件（需要在主路由构建前）
        let mut router = base_router;
        for plugin in &extend_plugins {
            if PluginCacheManager::is_plugin_enabled(&plugin.name).await? {
                router = self.injector.inject_extend_plugin(router, plugin).await?;
            } else {
                tracing::warn!("Skipping disabled extend plugin: {}", plugin.name);
            }
        }

        // 2. 注入应用级插件
        for plugin in &app_plugins {
            if PluginCacheManager::is_plugin_enabled(&plugin.name).await? {
                router = self.injector.inject_app_plugin(router, plugin).await?;
            } else {
                tracing::warn!("Skipping disabled app plugin: {}", plugin.name);
            }
        }

        Ok(router)
    }

    /// 重新加载插件
    /// 用于热重载功能
    pub async fn reload(&self) -> Result<(), AppError> {
        tracing::info!("Reloading plugins...");

        // 清除旧缓存
        self.injector.clear().await;

        // 重新初始化
        self.initialize().await?;

        tracing::info!("Plugins reloaded successfully");
        Ok(())
    }

    /// 启用/禁用插件
    pub async fn set_plugin_status(
        &self,
        plugin_name: &str,
        enabled: bool,
    ) -> Result<(), AppError> {
        // 检查插件是否存在
        if !self.discovery.exists(plugin_name) {
            return Err(AppError::with_message(
                ErrorCode::NotFound,
                format!("Plugin {} not found", plugin_name)
            ));
        }

        // 更新缓存中的状态
        PluginCacheManager::update_plugin_status(plugin_name, enabled).await?;

        tracing::info!("Plugin {} status changed to: {}", plugin_name, enabled);
        Ok(())
    }

    /// 获取所有插件信息
    pub async fn get_all_plugins(&self) -> Vec<DiscoveredPlugin> {
        self.discovered_plugins.read().await.clone()
    }

    /// 获取特定插件
    pub async fn get_plugin(&self, name: &str) -> Result<Option<DiscoveredPlugin>, AppError> {
        self.discovery.get_plugin(name)
    }

    /// 检查插件是否需要重载
    pub async fn needs_reload(&self) -> Result<bool, AppError> {
        PluginCacheManager::is_plugin_changed().await
    }

    /// 获取已注入的扩展级插件
    pub async fn get_injected_extend_plugins(&self) -> std::collections::HashMap<String, Vec<String>> {
        self.injector.get_injected_extend_plugins().await
    }

    /// 获取已注入的应用级插件
    pub async fn get_injected_app_plugins(&self) -> Vec<String> {
        self.injector.get_injected_app_plugins().await
    }
}

impl Default for UnifiedPluginManager {
    fn default() -> Self {
        Self::new("./plugins")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_plugin_manager() {
        let manager = UnifiedPluginManager::new("./plugins");
        
        // 测试基本功能
        let plugins = manager.get_all_plugins().await;
        assert!(plugins.is_empty()); // 初始化前应为空
    }

    #[tokio::test]
    async fn test_plugin_discovery() {
        let manager = UnifiedPluginManager::new("./plugins");
        
        // 如果插件目录存在，测试发现功能
        if std::path::Path::new("./plugins").exists() {
            let result = manager.initialize().await;
            // 可能失败（如果Redis未启动），但不应panic
            match result {
                Ok(_) => println!("Plugin discovery successful"),
                Err(e) => println!("Plugin discovery failed (expected if Redis not available): {:?}", e),
            }
        }
    }
}
