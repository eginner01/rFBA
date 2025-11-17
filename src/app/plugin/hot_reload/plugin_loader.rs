//! 插件热加载器
//! 支持插件文件变化时自动重启服务

use axum::Router;
use notify::{RecommendedWatcher, Watcher, Event, RecursiveMode};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::sync::mpsc;
use tracing::{error, info, warn};
use tokio::sync::RwLock;

use crate::app::plugin::manager::PluginManager;

/// 插件加载器状态
#[derive(Debug, Clone)]
pub struct PluginLoaderState {
    pub plugins_dir: std::path::PathBuf,
    pub main_router: Option<Router>,
    pub watcher: Option<RecommendedWatcher>,
}

/// 插件热加载器
pub struct PluginHotReloader {
    state: RwLock<PluginLoaderState>,
}

impl PluginHotReloader {
    /// 创建新的插件热加载器
    pub fn new(plugins_dir: std::path::PathBuf) -> Self {
        Self {
            state: RwLock::new(PluginLoaderState {
                plugins_dir,
                main_router: None,
                watcher: None,
            }),
        }
    }

    /// 初始化插件热加载器
    pub async fn initialize(&self, main_router: Router) -> Result<(), PluginLoadError> {
        let mut state = self.state.write().await;
        state.main_router = Some(main_router);

        // 启动文件系统监控
        self.start_file_watcher().await?;

        Ok(())
    }

    /// 启动文件系统监控
    async fn start_file_watcher(&self) -> Result<(), PluginLoadError> {
        let plugins_dir = {
            let state = self.state.read().await;
            state.plugins_dir.clone()
        };

        if !plugins_dir.exists() {
            warn!("Plugins directory does not exist: {:?}", plugins_dir);
            return Ok(());
        }

        let (tx, rx) = mpsc::channel();

        let mut watcher = RecommendedWatcher::new(tx, notify::Config::default())
            .map_err(|e| PluginLoadError::WatcherError(format!("Failed to create watcher: {}", e)))?;

        watcher.watch(&plugins_dir, RecursiveMode::Recursive)
            .map_err(|e| PluginLoadError::WatcherError(format!("Failed to watch directory: {}", e)))?;

        info!("Started watching plugins directory: {:?}", plugins_dir);

        // 保存watcher
        {
            let mut state = self.state.write().await;
            state.watcher = Some(watcher);
        }

        // 启动监控任务
        tokio::spawn(async move {
            if let Err(e) = Self::watch_files(rx).await {
                error!("File watcher error: {}", e);
            }
        });

        Ok(())
    }

    /// 监控文件变化
    async fn watch_files(mut rx: mpsc::Receiver<Result<Event, notify::Error>>) -> Result<(), PluginLoadError> {
        while let Ok(event) = rx.recv() {
            match event {
                Ok(event) => {
                    if Self::is_plugin_file_event(&event) {
                        info!("Plugin file changed: {:?}", event);
                        Self::handle_plugin_change().await;
                    }
                }
                Err(e) => {
                    error!("File watch event error: {}", e);
                }
            }
        }
        Ok(())
    }

    /// 检查是否是插件文件变化事件
    fn is_plugin_file_event(event: &Event) -> bool {
        // 检查是否是plugin.toml或Cargo.toml文件变化
        for path in &event.paths {
            if path.ends_with("plugin.toml") || path.ends_with("Cargo.toml") || path.ends_with("lib.rs") {
                return true;
            }
        }
        false
    }

    /// 处理插件变化
    async fn handle_plugin_change() {
        info!("Plugin changes detected, triggering reload...");

        // 发送重启信号
        // 在实际应用中，这里应该触发优雅的服务器重启
        // 可以通过发送信号给主进程或重启整个服务来实现

        // 注意：这里只是日志记录，实际重启逻辑需要在main.rs中实现
    }

    /// 获取插件目录
    pub async fn get_plugins_dir(&self) -> std::path::PathBuf {
        let state = self.state.read().await;
        state.plugins_dir.clone()
    }
}

/// 插件加载错误
#[derive(Debug, thiserror::Error)]
pub enum PluginLoadError {
    #[error("Watcher error: {0}")]
    WatcherError(String),

    #[error("Plugin load error: {0}")]
    LoadError(String),

    #[error("Plugin not found: {0}")]
    NotFound(String),

    #[error("Plugin config error: {0}")]
    ConfigError(String),
}

/// 插件信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoadedPlugin {
    pub name: String,
    pub version: String,
    pub description: String,
    pub router: Router,
    pub is_active: bool,
}

/// 插件路由器管理器
pub struct PluginRouterManager {
    plugins: RwLock<HashMap<String, LoadedPlugin>>,
}

impl PluginRouterManager {
    /// 创建新的路由器管理器
    pub fn new() -> Self {
        Self {
            plugins: RwLock::new(HashMap::new()),
        }
    }

    /// 添加插件
    pub async fn add_plugin(&self, plugin: LoadedPlugin) {
        let mut plugins = self.plugins.write().await;
        plugins.insert(plugin.name.clone(), plugin);
        info!("Added plugin: {}", plugin.name);
    }

    /// 移除插件
    pub async fn remove_plugin(&self, name: &str) {
        let mut plugins = self.plugins.write().await;
        plugins.remove(name);
        info!("Removed plugin: {}", name);
    }

    /// 获取所有插件
    pub async fn get_all_plugins(&self) -> HashMap<String, LoadedPlugin> {
        let plugins = self.plugins.read().await;
        plugins.clone()
    }

    /// 检查插件是否存在
    pub async fn has_plugin(&self, name: &str) -> bool {
        let plugins = self.plugins.read().await;
        plugins.contains_key(name)
    }

    /// 获取活跃插件列表
    pub async fn get_active_plugins(&self) -> Vec<LoadedPlugin> {
        let plugins = self.plugins.read().await;
        plugins.values()
            .filter(|p| p.is_active)
            .cloned()
            .collect()
    }

    /// 构建完整的应用路由
    pub async fn build_complete_router(&self, main_router: Router) -> Router {
        let active_plugins = self.get_active_plugins().await;

        let mut router = main_router;

        // 合并所有活跃插件的路由
        for plugin in active_plugins {
            router = router.merge(plugin.router);
            info!("Merged plugin router: {}", plugin.name);
        }

        router
    }
}
