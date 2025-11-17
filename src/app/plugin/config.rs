/// 插件配置模块
/// 负责解析plugin.toml配置文件

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use crate::common::exception::{AppError, ErrorCode};

/// 插件配置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PluginConfig {
    /// 插件基本信息
    #[serde(default)]
    pub plugin: PluginInfo,
    /// 应用配置
    pub app: Option<AppConfig>,
    /// API配置
    pub api: Option<HashMap<String, ApiConfig>>,
}

/// 插件基本信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PluginInfo {
    /// 插件摘要
    #[serde(default)]
    pub summary: String,
    /// 版本号
    #[serde(default)]
    pub version: String,
    /// 详细描述
    #[serde(default)]
    pub description: String,
    /// 作者
    #[serde(default)]
    pub author: String,
    /// 是否启用 (1=启用, 0=禁用)
    #[serde(default = "default_enable")]
    pub enable: i32,
    /// 插件名称（动态添加，不在toml中）
    #[serde(skip)]
    pub name: Option<String>,
}

fn default_enable() -> i32 {
    1
}

/// 应用配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// 扩展级插件：扩展的目标app模块名称
    pub extend: Option<String>,
    /// 应用级插件：路由器列表
    pub router: Option<Vec<String>>,
}

/// API配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    /// 路由前缀
    pub prefix: String,
    /// 标签
    pub tags: Option<String>,
}

impl PluginConfig {
    /// 从toml文件加载配置
    pub fn load_from_file(path: &Path) -> Result<Self, AppError> {
        use std::fs;
        
        let content = fs::read_to_string(path).map_err(|e| {
            tracing::error!("Failed to read plugin config file: {:?}", e);
            AppError::with_message(
                ErrorCode::IOError,
                format!("Failed to read plugin config: {}", e)
            )
        })?;

        let mut config: PluginConfig = toml::from_str(&content).map_err(|e| {
            tracing::error!("Failed to parse plugin config: {:?}", e);
            AppError::with_message(
                ErrorCode::InvalidInput,
                format!("Invalid plugin config format: {}", e)
            )
        })?;

        // 从文件路径提取插件名称
        if let Some(parent) = path.parent() {
            if let Some(dir_name) = parent.file_name() {
                if let Some(name_str) = dir_name.to_str() {
                    config.plugin.name = Some(name_str.to_string());
                }
            }
        }

        Ok(config)
    }

    /// 验证配置有效性
    pub fn validate(&self) -> Result<(), AppError> {
        // 验证必填字段
        if self.plugin.summary.is_empty() {
            return Err(AppError::with_message(
                ErrorCode::InvalidInput,
                "Plugin summary is required"
            ));
        }

        if self.plugin.version.is_empty() {
            return Err(AppError::with_message(
                ErrorCode::InvalidInput,
                "Plugin version is required"
            ));
        }

        if self.plugin.description.is_empty() {
            return Err(AppError::with_message(
                ErrorCode::InvalidInput,
                "Plugin description is required"
            ));
        }

        if self.plugin.author.is_empty() {
            return Err(AppError::with_message(
                ErrorCode::InvalidInput,
                "Plugin author is required"
            ));
        }

        // 验证app配置
        if let Some(ref app) = self.app {
            if app.extend.is_none() && app.router.is_none() {
                return Err(AppError::with_message(
                    ErrorCode::InvalidInput,
                    "Plugin must specify either 'extend' or 'router' in app config"
                ));
            }

            // 扩展级插件必须有API配置
            if app.extend.is_some() && self.api.is_none() {
                return Err(AppError::with_message(
                    ErrorCode::InvalidInput,
                    "Extend-level plugin must have API configuration"
                ));
            }

            // 应用级插件必须有router列表
            if app.router.is_some() && app.extend.is_some() {
                return Err(AppError::with_message(
                    ErrorCode::InvalidInput,
                    "Plugin cannot be both extend-level and app-level"
                ));
            }
        } else {
            return Err(AppError::with_message(
                ErrorCode::InvalidInput,
                "Plugin app configuration is required"
            ));
        }

        Ok(())
    }

    /// 判断是否为扩展级插件
    pub fn is_extend_plugin(&self) -> bool {
        self.app.as_ref()
            .and_then(|app| app.extend.as_ref())
            .is_some()
    }

    /// 判断是否为应用级插件
    pub fn is_app_plugin(&self) -> bool {
        self.app.as_ref()
            .and_then(|app| app.router.as_ref())
            .is_some()
    }

    /// 获取扩展目标
    pub fn get_extend_target(&self) -> Option<&str> {
        self.app.as_ref()
            .and_then(|app| app.extend.as_ref())
            .map(|s| s.as_str())
    }

    /// 获取路由器列表
    pub fn get_routers(&self) -> Option<&[String]> {
        self.app.as_ref()
            .and_then(|app| app.router.as_ref())
            .map(|v| v.as_slice())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_extend_plugin_config() {
        let toml_str = r#"
[plugin]
summary = "数据字典"
version = "0.0.8"
description = "通常用于约束前端工程数据展示"
author = "wu-clan"

[app]
extend = "admin"

[api.dict_data]
prefix = "/dict-datas"
tags = "系统字典数据"

[api.dict_type]
prefix = "/dict-types"
tags = "系统字典类型"
        "#;

        let config: PluginConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(config.plugin.summary, "数据字典");
        assert_eq!(config.plugin.version, "0.0.8");
        assert!(config.is_extend_plugin());
        assert!(!config.is_app_plugin());
        assert_eq!(config.get_extend_target(), Some("admin"));
    }

    #[test]
    fn test_parse_app_plugin_config() {
        let toml_str = r#"
[plugin]
summary = "字典管理插件"
version = "1.0.0"
description = "字典管理插件，扩展系统字典功能"
author = "FastAPI Best Architecture"
enable = 1

[app]
router = ["dictionary"]
        "#;

        let config: PluginConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(config.plugin.summary, "字典管理插件");
        assert!(!config.is_extend_plugin());
        assert!(config.is_app_plugin());
        assert_eq!(config.get_routers(), Some(&["dictionary".to_string()][..]));
    }
}
