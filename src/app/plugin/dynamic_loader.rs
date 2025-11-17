/// 动态库加载器
/// 使用libloading实现插件动态加载

use libloading::{Library, Symbol};
use std::path::Path;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;
use crate::common::exception::{AppError, ErrorCode};
use axum::Router;

/// 插件接口trait
/// 所有动态加载的插件都必须实现此接口
pub trait DynamicPlugin: Send + Sync {
    /// 获取插件名称
    fn name(&self) -> &str;
    
    /// 获取插件版本
    fn version(&self) -> &str;
    
    /// 初始化插件
    fn initialize(&mut self) -> Result<(), String>;
    
    /// 获取插件路由
    fn get_router(&self) -> Router;
    
    /// 关闭插件
    fn shutdown(&mut self) -> Result<(), String>;
}

/// 动态库加载器
pub struct DynamicPluginLoader {
    /// 已加载的库
    libraries: Arc<RwLock<HashMap<String, Library>>>,
    /// 已加载的插件实例
    plugins: Arc<RwLock<HashMap<String, Box<dyn DynamicPlugin>>>>,
}

impl DynamicPluginLoader {
    /// 创建新的加载器
    pub fn new() -> Self {
        Self {
            libraries: Arc::new(RwLock::new(HashMap::new())),
            plugins: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// 加载插件动态库
    pub async fn load_plugin<P: AsRef<Path>>(
        &self,
        plugin_name: &str,
        library_path: P,
    ) -> Result<(), AppError> {
        tracing::info!("Loading plugin {} from {:?}", plugin_name, library_path.as_ref());

        // 加载动态库
        let library = unsafe {
            Library::new(library_path.as_ref()).map_err(|e| {
                tracing::error!("Failed to load library: {:?}", e);
                AppError::with_message(
                    ErrorCode::IOError,
                    format!("Failed to load plugin library: {}", e)
                )
            })?
        };

        // 获取插件创建函数
        // 插件库必须导出 create_plugin() 函数
        let create_plugin: Symbol<unsafe extern "C" fn() -> *mut dyn DynamicPlugin> = unsafe {
            library.get(b"create_plugin").map_err(|e| {
                tracing::error!("Failed to get create_plugin symbol: {:?}", e);
                AppError::with_message(
                    ErrorCode::InvalidInput,
                    "Plugin library does not export create_plugin function"
                )
            })?
        };

        // 创建插件实例
        let plugin_ptr = unsafe { create_plugin() };
        let mut plugin = unsafe { Box::from_raw(plugin_ptr) };

        // 初始化插件
        plugin.initialize().map_err(|e| {
            tracing::error!("Failed to initialize plugin: {}", e);
            AppError::with_message(ErrorCode::OperationFailed, format!("Plugin initialization failed: {}", e))
        })?;

        // 保存库和插件
        let mut libraries = self.libraries.write().await;
        let mut plugins = self.plugins.write().await;

        libraries.insert(plugin_name.to_string(), library);
        plugins.insert(plugin_name.to_string(), plugin);

        tracing::info!("Plugin {} loaded successfully", plugin_name);
        Ok(())
    }

    /// 卸载插件
    pub async fn unload_plugin(&self, plugin_name: &str) -> Result<(), AppError> {
        tracing::info!("Unloading plugin {}", plugin_name);

        let mut plugins = self.plugins.write().await;
        let mut libraries = self.libraries.write().await;

        // 关闭插件
        if let Some(mut plugin) = plugins.remove(plugin_name) {
            plugin.shutdown().map_err(|e| {
                tracing::error!("Failed to shutdown plugin: {}", e);
                AppError::with_message(ErrorCode::OperationFailed, format!("Plugin shutdown failed: {}", e))
            })?;
        }

        // 卸载库
        libraries.remove(plugin_name);

        tracing::info!("Plugin {} unloaded successfully", plugin_name);
        Ok(())
    }

    /// 获取插件的路由
    pub async fn get_plugin_router(&self, plugin_name: &str) -> Result<Router, AppError> {
        let plugins = self.plugins.read().await;
        
        let plugin = plugins.get(plugin_name)
            .ok_or_else(|| {
                AppError::with_message(
                    ErrorCode::NotFound,
                    format!("Plugin {} not found", plugin_name)
                )
            })?;

        Ok(plugin.get_router())
    }

    /// 检查插件是否已加载
    pub async fn is_loaded(&self, plugin_name: &str) -> bool {
        let plugins = self.plugins.read().await;
        plugins.contains_key(plugin_name)
    }

    /// 获取所有已加载的插件名称
    pub async fn list_loaded_plugins(&self) -> Vec<String> {
        let plugins = self.plugins.read().await;
        plugins.keys().cloned().collect()
    }

    /// 重新加载插件
    pub async fn reload_plugin<P: AsRef<Path>>(
        &self,
        plugin_name: &str,
        library_path: P,
    ) -> Result<(), AppError> {
        tracing::info!("Reloading plugin {}", plugin_name);

        // 先卸载
        if self.is_loaded(plugin_name).await {
            self.unload_plugin(plugin_name).await?;
        }

        // 再加载
        self.load_plugin(plugin_name, library_path).await?;

        Ok(())
    }
}

impl Default for DynamicPluginLoader {
    fn default() -> Self {
        Self::new()
    }
}

/// 插件宏：用于简化插件导出
/// 
/// # 示例
/// ```rust,ignore
/// use plugin_macro::export_plugin;
/// 
/// struct MyPlugin;
/// impl DynamicPlugin for MyPlugin {
///     // ... 实现
/// }
/// 
/// export_plugin!(MyPlugin);
/// ```
#[macro_export]
macro_rules! export_plugin {
    ($plugin_type:ty) => {
        #[no_mangle]
        pub extern "C" fn create_plugin() -> *mut dyn $crate::app::plugin::DynamicPlugin {
            let plugin = <$plugin_type>::default();
            Box::into_raw(Box::new(plugin))
        }

        #[no_mangle]
        pub extern "C" fn destroy_plugin(ptr: *mut dyn $crate::app::plugin::DynamicPlugin) {
            if !ptr.is_null() {
                unsafe {
                    drop(Box::from_raw(ptr));
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_dynamic_loader() {
        let loader = DynamicPluginLoader::new();
        assert!(loader.list_loaded_plugins().await.is_empty());
    }
}
