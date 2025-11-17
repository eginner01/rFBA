/// 插件发现模块
/// 负责扫描和发现插件目录中的所有插件

use std::path::{Path, PathBuf};
use std::fs;
use crate::common::exception::{AppError, ErrorCode};
use crate::app::plugin::config::PluginConfig;

/// 插件发现器
pub struct PluginDiscovery {
    /// 插件根目录
    plugin_dir: PathBuf,
}

/// 已发现的插件信息
#[derive(Debug, Clone)]
pub struct DiscoveredPlugin {
    /// 插件名称（目录名）
    pub name: String,
    /// 插件配置
    pub config: PluginConfig,
    /// 插件目录路径
    pub path: PathBuf,
}

impl PluginDiscovery {
    /// 创建插件发现器
    pub fn new<P: AsRef<Path>>(plugin_dir: P) -> Self {
        Self {
            plugin_dir: plugin_dir.as_ref().to_path_buf(),
        }
    }

    /// 扫描并发现所有插件
    pub fn discover_all(&self) -> Result<Vec<DiscoveredPlugin>, AppError> {
        if !self.plugin_dir.exists() {
            tracing::warn!("Plugin directory does not exist: {:?}", self.plugin_dir);
            return Ok(Vec::new());
        }

        let mut plugins = Vec::new();

        let entries = fs::read_dir(&self.plugin_dir).map_err(|e| {
            tracing::error!("Failed to read plugin directory: {:?}", e);
            AppError::with_message(
                ErrorCode::IOError,
                format!("Failed to read plugin directory: {}", e)
            )
        })?;

        for entry in entries {
            let entry = entry.map_err(|e| {
                tracing::error!("Failed to read directory entry: {:?}", e);
                AppError::with_message(ErrorCode::IOError, "Failed to read directory entry")
            })?;

            let path = entry.path();
            
            // 跳过非目录项
            if !path.is_dir() {
                continue;
            }

            // 跳过 __pycache__ 等特殊目录
            if let Some(dir_name) = path.file_name() {
                if let Some(name_str) = dir_name.to_str() {
                    if name_str.starts_with("__") || name_str.starts_with(".") {
                        continue;
                    }

                    // 检查是否包含 plugin.toml 配置文件
                    let config_path = path.join("plugin.toml");
                    if config_path.exists() {
                        match self.load_plugin(&path) {
                            Ok(plugin) => plugins.push(plugin),
                            Err(e) => {
                                tracing::error!("Failed to load plugin {}: {:?}", name_str, e);
                                // 继续处理其他插件，不中断整个发现过程
                            }
                        }
                    }
                }
            }
        }

        tracing::info!("Discovered {} plugins", plugins.len());
        Ok(plugins)
    }

    /// 加载单个插件
    fn load_plugin(&self, plugin_path: &Path) -> Result<DiscoveredPlugin, AppError> {
        let plugin_name = plugin_path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| {
                AppError::with_message(ErrorCode::InvalidInput, "Invalid plugin directory name")
            })?
            .to_string();

        let config_path = plugin_path.join("plugin.toml");
        let mut config = PluginConfig::load_from_file(&config_path)?;
        
        // 设置插件名称
        config.plugin.name = Some(plugin_name.clone());
        
        // 验证配置
        config.validate()?;

        Ok(DiscoveredPlugin {
            name: plugin_name,
            config,
            path: plugin_path.to_path_buf(),
        })
    }

    /// 获取特定插件
    pub fn get_plugin(&self, name: &str) -> Result<Option<DiscoveredPlugin>, AppError> {
        let plugin_path = self.plugin_dir.join(name);
        
        if !plugin_path.exists() {
            return Ok(None);
        }

        let config_path = plugin_path.join("plugin.toml");
        if !config_path.exists() {
            return Ok(None);
        }

        self.load_plugin(&plugin_path).map(Some)
    }

    /// 按类型分类插件
    pub fn classify_plugins(
        &self,
        plugins: Vec<DiscoveredPlugin>,
    ) -> (Vec<DiscoveredPlugin>, Vec<DiscoveredPlugin>) {
        let mut extend_plugins = Vec::new();
        let mut app_plugins = Vec::new();

        for plugin in plugins {
            if plugin.config.is_extend_plugin() {
                extend_plugins.push(plugin);
            } else if plugin.config.is_app_plugin() {
                app_plugins.push(plugin);
            } else {
                tracing::warn!("Plugin {} is neither extend nor app type", plugin.name);
            }
        }

        (extend_plugins, app_plugins)
    }

    /// 检查插件是否存在
    pub fn exists(&self, name: &str) -> bool {
        let plugin_path = self.plugin_dir.join(name);
        let config_path = plugin_path.join("plugin.toml");
        plugin_path.exists() && config_path.exists()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_discovery() {
        // 测试需要实际的插件目录
        let discovery = PluginDiscovery::new("./plugins");
        if discovery.plugin_dir.exists() {
            let result = discovery.discover_all();
            assert!(result.is_ok());
        }
    }
}
