/// 插件沙箱隔离模块
/// 限制插件访问系统资源，提供安全运行环境

use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::{HashMap, HashSet};
use crate::common::exception::{AppError, ErrorCode};

/// 插件权限
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PluginPermission {
    /// 网络访问
    Network,
    /// 文件系统读取
    FileRead,
    /// 文件系统写入
    FileWrite,
    /// 数据库访问
    Database,
    /// Redis访问
    Redis,
    /// HTTP客户端
    HttpClient,
    /// 系统信息读取
    SystemInfo,
    /// 环境变量访问
    Environment,
}

/// 资源限制
#[derive(Debug, Clone)]
pub struct ResourceLimits {
    /// 最大内存使用（字节）
    pub max_memory: Option<usize>,
    /// 最大CPU时间（秒）
    pub max_cpu_time: Option<u64>,
    /// 最大文件打开数
    pub max_open_files: Option<usize>,
    /// 最大网络连接数
    pub max_connections: Option<usize>,
    /// 请求速率限制（请求/秒）
    pub rate_limit: Option<u32>,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_memory: Some(256 * 1024 * 1024), // 256MB
            max_cpu_time: Some(60), // 60秒
            max_open_files: Some(100),
            max_connections: Some(50),
            rate_limit: Some(100), // 100 req/s
        }
    }
}

/// 插件沙箱
pub struct PluginSandbox {
    /// 插件名称
    plugin_name: String,
    /// 允许的权限
    allowed_permissions: Arc<RwLock<HashSet<PluginPermission>>>,
    /// 资源限制
    resource_limits: Arc<RwLock<ResourceLimits>>,
    /// 当前资源使用统计
    resource_usage: Arc<RwLock<ResourceUsage>>,
}

/// 资源使用统计
#[derive(Debug, Clone, Default)]
pub struct ResourceUsage {
    /// 当前内存使用
    pub memory_used: usize,
    /// CPU时间使用
    pub cpu_time_used: u64,
    /// 打开的文件数
    pub open_files: usize,
    /// 活跃连接数
    pub active_connections: usize,
    /// 请求计数
    pub request_count: u64,
}

impl PluginSandbox {
    /// 创建新的沙箱
    pub fn new(plugin_name: String) -> Self {
        Self {
            plugin_name,
            allowed_permissions: Arc::new(RwLock::new(HashSet::new())),
            resource_limits: Arc::new(RwLock::new(ResourceLimits::default())),
            resource_usage: Arc::new(RwLock::new(ResourceUsage::default())),
        }
    }

    /// 授予权限
    pub async fn grant_permission(&self, permission: PluginPermission) {
        let mut perms = self.allowed_permissions.write().await;
        tracing::info!("Granted permission {:?} to plugin {}", permission, self.plugin_name);
        perms.insert(permission);
    }

    /// 撤销权限
    pub async fn revoke_permission(&self, permission: &PluginPermission) {
        let mut perms = self.allowed_permissions.write().await;
        perms.remove(permission);
        tracing::info!("Revoked permission {:?} from plugin {}", permission, self.plugin_name);
    }

    /// 检查是否有权限
    pub async fn has_permission(&self, permission: &PluginPermission) -> bool {
        let perms = self.allowed_permissions.read().await;
        perms.contains(permission)
    }

    /// 检查并验证权限
    pub async fn check_permission(&self, permission: &PluginPermission) -> Result<(), AppError> {
        if self.has_permission(permission).await {
            Ok(())
        } else {
            tracing::warn!(
                "Plugin {} attempted to use {:?} without permission",
                self.plugin_name, permission
            );
            Err(AppError::with_message(
                ErrorCode::PermissionDenied,
                format!("Plugin does not have {:?} permission", permission)
            ))
        }
    }

    /// 设置资源限制
    pub async fn set_resource_limits(&self, limits: ResourceLimits) {
        let mut resource_limits = self.resource_limits.write().await;
        *resource_limits = limits;
        tracing::info!("Updated resource limits for plugin {}", self.plugin_name);
    }

    /// 检查资源使用是否超限
    pub async fn check_resource_limits(&self) -> Result<(), AppError> {
        let usage = self.resource_usage.read().await;
        let limits = self.resource_limits.read().await;

        // 检查内存
        if let Some(max_memory) = limits.max_memory {
            if usage.memory_used > max_memory {
                return Err(AppError::with_message(
                    ErrorCode::OperationFailed,
                    format!("Plugin {} exceeded memory limit", self.plugin_name)
                ));
            }
        }

        // 检查CPU时间
        if let Some(max_cpu_time) = limits.max_cpu_time {
            if usage.cpu_time_used > max_cpu_time {
                return Err(AppError::with_message(
                    ErrorCode::OperationFailed,
                    format!("Plugin {} exceeded CPU time limit", self.plugin_name)
                ));
            }
        }

        // 检查打开文件数
        if let Some(max_files) = limits.max_open_files {
            if usage.open_files > max_files {
                return Err(AppError::with_message(
                    ErrorCode::OperationFailed,
                    format!("Plugin {} exceeded open files limit", self.plugin_name)
                ));
            }
        }

        // 检查连接数
        if let Some(max_conns) = limits.max_connections {
            if usage.active_connections > max_conns {
                return Err(AppError::with_message(
                    ErrorCode::OperationFailed,
                    format!("Plugin {} exceeded connections limit", self.plugin_name)
                ));
            }
        }

        Ok(())
    }

    /// 更新资源使用统计
    pub async fn update_usage<F>(&self, update_fn: F)
    where
        F: FnOnce(&mut ResourceUsage),
    {
        let mut usage = self.resource_usage.write().await;
        update_fn(&mut usage);
    }

    /// 获取资源使用统计
    pub async fn get_usage(&self) -> ResourceUsage {
        self.resource_usage.read().await.clone()
    }

    /// 重置资源使用统计
    pub async fn reset_usage(&self) {
        let mut usage = self.resource_usage.write().await;
        *usage = ResourceUsage::default();
        tracing::info!("Reset resource usage for plugin {}", self.plugin_name);
    }
}

/// 沙箱管理器
pub struct SandboxManager {
    sandboxes: Arc<RwLock<HashMap<String, Arc<PluginSandbox>>>>,
}

impl SandboxManager {
    /// 创建新的管理器
    pub fn new() -> Self {
        Self {
            sandboxes: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// 为插件创建沙箱
    pub async fn create_sandbox(&self, plugin_name: String) -> Arc<PluginSandbox> {
        let sandbox = Arc::new(PluginSandbox::new(plugin_name.clone()));
        
        let mut sandboxes = self.sandboxes.write().await;
        sandboxes.insert(plugin_name, sandbox.clone());
        
        sandbox
    }

    /// 获取插件沙箱
    pub async fn get_sandbox(&self, plugin_name: &str) -> Option<Arc<PluginSandbox>> {
        let sandboxes = self.sandboxes.read().await;
        sandboxes.get(plugin_name).cloned()
    }

    /// 移除插件沙箱
    pub async fn remove_sandbox(&self, plugin_name: &str) {
        let mut sandboxes = self.sandboxes.write().await;
        sandboxes.remove(plugin_name);
        tracing::info!("Removed sandbox for plugin {}", plugin_name);
    }

    /// 获取所有沙箱
    pub async fn list_sandboxes(&self) -> Vec<String> {
        let sandboxes = self.sandboxes.read().await;
        sandboxes.keys().cloned().collect()
    }
}

impl Default for SandboxManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_sandbox_permissions() {
        let sandbox = PluginSandbox::new("test_plugin".to_string());
        
        assert!(!sandbox.has_permission(&PluginPermission::Network).await);
        
        sandbox.grant_permission(PluginPermission::Network).await;
        assert!(sandbox.has_permission(&PluginPermission::Network).await);
        
        sandbox.revoke_permission(&PluginPermission::Network).await;
        assert!(!sandbox.has_permission(&PluginPermission::Network).await);
    }

    #[tokio::test]
    async fn test_sandbox_manager() {
        let manager = SandboxManager::new();
        
        let sandbox = manager.create_sandbox("test_plugin".to_string()).await;
        assert!(manager.get_sandbox("test_plugin").await.is_some());
        
        manager.remove_sandbox("test_plugin").await;
        assert!(manager.get_sandbox("test_plugin").await.is_none());
    }
}
