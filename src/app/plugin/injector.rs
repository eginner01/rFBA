/// 插件路由注入器
/// 负责将插件路由动态注入到主应用路由中

use axum::Router;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::common::exception::{AppError, ErrorCode};
use crate::app::plugin::discovery::DiscoveredPlugin;

/// 路由注入器
pub struct RouteInjector {
    /// 已注入的扩展级插件路由
    extend_plugins: Arc<RwLock<HashMap<String, Vec<String>>>>,
    /// 已注入的应用级插件路由
    app_plugins: Arc<RwLock<Vec<String>>>,
}

impl RouteInjector {
    /// 创建新的路由注入器
    pub fn new() -> Self {
        Self {
            extend_plugins: Arc::new(RwLock::new(HashMap::new())),
            app_plugins: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// 注入扩展级插件路由
    /// 将插件API路由注入到指定的目标app模块
    pub async fn inject_extend_plugin(
        &self,
        target_router: Router,
        plugin: &DiscoveredPlugin,
    ) -> Result<Router, AppError> {
        let plugin_name = &plugin.name;
        
        tracing::info!("Injecting extend-level plugin: {}", plugin_name);

        let extend_target = plugin.config.get_extend_target()
            .ok_or_else(|| {
                AppError::with_message(
                    ErrorCode::InvalidInput,
                    "Extend plugin must have extend target"
                )
            })?;

        let api_configs = plugin.config.api.as_ref()
            .ok_or_else(|| {
                AppError::with_message(
                    ErrorCode::InvalidInput,
                    "Extend plugin must have API configuration"
                )
            })?;

        let mut router = target_router;

        // 遍历所有API配置
        for (api_name, api_config) in api_configs {
            tracing::debug!(
                "Injecting API {} with prefix {} for plugin {}",
                api_name,
                api_config.prefix,
                plugin_name
            );

            // TODO: 这里需要实际加载插件的路由器
            // 目前返回空路由作为占位符
            // 实际实现需要动态加载插件模块并获取其router
            
            // 创建插件路由（暂时为空，实际使用时需要加载真实路由）
            let plugin_router = Router::new();

            // 将插件路由添加到主路由
            router = router.nest(&api_config.prefix, plugin_router);
        }

        // 记录已注入的插件
        let mut extend_plugins = self.extend_plugins.write().await;
        extend_plugins
            .entry(extend_target.to_string())
            .or_insert_with(Vec::new)
            .push(plugin_name.clone());

        tracing::info!(
            "Successfully injected extend-level plugin {} to {}",
            plugin_name,
            extend_target
        );

        Ok(router)
    }

    /// 注入应用级插件路由
    /// 将插件作为独立的顶级路由添加到应用
    pub async fn inject_app_plugin(
        &self,
        main_router: Router,
        plugin: &DiscoveredPlugin,
    ) -> Result<Router, AppError> {
        let plugin_name = &plugin.name;
        
        tracing::info!("Injecting app-level plugin: {}", plugin_name);

        let routers = plugin.config.get_routers()
            .ok_or_else(|| {
                AppError::with_message(
                    ErrorCode::InvalidInput,
                    "App plugin must have router configuration"
                )
            })?;

        let mut router = main_router;

        // 遍历所有路由器
        for router_name in routers {
            tracing::debug!(
                "Injecting router {} for plugin {}",
                router_name,
                plugin_name
            );

            // TODO: 这里需要实际加载插件的路由器
            // 目前返回空路由作为占位符
            
            // 创建插件路由（暂时为空）
            let plugin_router = Router::new();

            // 将插件路由添加到主路由
            // 使用插件名作为路径前缀
            let route_prefix = format!("/plugin/{}", plugin_name);
            router = router.nest(&route_prefix, plugin_router);
        }

        // 记录已注入的插件
        let mut app_plugins = self.app_plugins.write().await;
        app_plugins.push(plugin_name.clone());

        tracing::info!(
            "Successfully injected app-level plugin {}",
            plugin_name
        );

        Ok(router)
    }

    /// 批量注入扩展级插件
    pub async fn inject_extend_plugins(
        &self,
        target_router: Router,
        plugins: Vec<DiscoveredPlugin>,
    ) -> Result<Router, AppError> {
        let mut router = target_router;

        for plugin in plugins {
            router = self.inject_extend_plugin(router, &plugin).await?;
        }

        Ok(router)
    }

    /// 批量注入应用级插件
    pub async fn inject_app_plugins(
        &self,
        main_router: Router,
        plugins: Vec<DiscoveredPlugin>,
    ) -> Result<Router, AppError> {
        let mut router = main_router;

        for plugin in plugins {
            router = self.inject_app_plugin(router, &plugin).await?;
        }

        Ok(router)
    }

    /// 获取已注入的扩展级插件列表
    pub async fn get_injected_extend_plugins(&self) -> HashMap<String, Vec<String>> {
        self.extend_plugins.read().await.clone()
    }

    /// 获取已注入的应用级插件列表
    pub async fn get_injected_app_plugins(&self) -> Vec<String> {
        self.app_plugins.read().await.clone()
    }

    /// 清除所有已注入的插件记录
    pub async fn clear(&self) {
        self.extend_plugins.write().await.clear();
        self.app_plugins.write().await.clear();
    }
}

impl Default for RouteInjector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::Router;
    #[tokio::test]
    async fn test_route_injector() {
        let injector = RouteInjector::new();
        let router: Router = Router::new();

        // 测试基本功能
        assert!(injector.get_injected_app_plugins().await.is_empty());
        assert!(injector.get_injected_extend_plugins().await.is_empty());
    }
}
