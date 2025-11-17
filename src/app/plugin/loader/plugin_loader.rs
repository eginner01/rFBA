/// 插件加载器
/// 负责插件文件的加载、解析和验证

use crate::common::exception::{AppError, ErrorCode};
use std::path::Path;
use std::fs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{error, info};

/// 插件加载器
pub struct PluginLoader {
    /// 插件目录
    plugin_dir: String,
    /// 已加载的插件文件
    loaded_files: HashMap<String, PluginFileInfo>,
}

/// 插件文件信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginFileInfo {
    /// 插件名称
    pub name: String,
    /// 插件编码
    pub code: String,
    /// 插件版本
    pub version: String,
    /// 插件类型
    pub plugin_type: i32,
    /// 插件描述
    pub description: Option<String>,
    /// 插件作者
    pub author: Option<String>,
    /// 插件主页
    pub homepage: Option<String>,
    /// 插件类名
    pub class_name: String,
    /// 插件配置
    pub config: Option<String>,
    /// 依赖插件
    pub dependencies: Option<String>,
    /// 文件路径
    pub file_path: String,
    /// 文件大小
    pub file_size: u64,
    /// 文件修改时间
    pub modified_time: chrono::NaiveDateTime,
    /// 文件校验和
    pub checksum: String,
}

/// 插件元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMetadata {
    /// 插件名称
    pub name: String,
    /// 插件编码
    pub code: String,
    /// 插件版本
    pub version: String,
    /// 插件类型
    pub plugin_type: i32,
    /// 插件描述
    pub description: Option<String>,
    /// 插件作者
    pub author: Option<String>,
    /// 插件主页
    pub homepage: Option<String>,
    /// 插件类名
    pub class_name: String,
    /// 插件配置
    pub config: Option<String>,
    /// 依赖插件
    pub dependencies: Option<String>,
    /// 插件权限
    pub permissions: Option<Vec<String>>,
    /// 插件入口点
    pub entry_point: Option<String>,
}

impl PluginLoader {
    /// 创建新的插件加载器
    pub fn new(plugin_dir: String) -> Self {
        Self {
            plugin_dir,
            loaded_files: HashMap::new(),
        }
    }

    /// 扫描插件目录
    pub async fn scan_plugin_directory(&mut self) -> Result<Vec<PluginFileInfo>, AppError> {
        info!("Scanning plugin directory: {}", self.plugin_dir);

        let plugin_dir_path = Path::new(&self.plugin_dir);
        if !plugin_dir_path.exists() {
            return Ok(Vec::new());
        }

        let mut plugin_files = Vec::new();

        // 扫描目录下的所有文件
        for entry in fs::read_dir(plugin_dir_path).map_err(|e| {
            error!("Failed to read plugin directory: {:?}", e);
            AppError::with_message(ErrorCode::IOError, "Failed to read plugin directory")
        })? {
            let entry = entry.map_err(|e| {
                error!("Failed to read directory entry: {:?}", e);
                AppError::with_message(ErrorCode::IOError, "Failed to read directory entry")
            })?;

            let path = entry.path();
            if path.is_file() {
                let file_extension = path.extension().and_then(|ext| ext.to_str());
                if file_extension == Some("zip") || file_extension == Some("jar") || file_extension == Some("so") {
                    if let Ok(plugin_info) = self.load_plugin_file(&path).await {
                        plugin_files.push(plugin_info);
                    }
                }
            }
        }

        info!("Found {} plugin files", plugin_files.len());
        Ok(plugin_files)
    }

    /// 加载插件文件
    pub async fn load_plugin_file(
        &mut self,
        file_path: &Path,
    ) -> Result<PluginFileInfo, AppError> {
        let file_path_str = file_path.to_string_lossy().to_string();
        let file_name = file_path
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| {
                AppError::with_message(ErrorCode::InvalidInput, "Invalid file path")
            })?;

        // 获取文件元数据
        let metadata = fs::metadata(file_path).map_err(|e| {
            error!("Failed to get file metadata: {:?}", e);
            AppError::with_message(ErrorCode::IOError, "Failed to get file metadata")
        })?;

        let file_size = metadata.len();
        let modified_time = {
            use std::time::UNIX_EPOCH;
            let system_time = metadata
                .modified()
                .map_err(|e| {
                    error!("Failed to get file modified time: {:?}", e);
                    AppError::with_message(ErrorCode::IOError, "Failed to get file modified time")
                })?;
            let duration = system_time
                .duration_since(UNIX_EPOCH)
                .map_err(|e| {
                    error!("Failed to convert system time: {:?}", e);
                    AppError::with_message(ErrorCode::IOError, "Failed to convert system time")
                })?;
            chrono::DateTime::from_timestamp(duration.as_secs() as i64, 0)
                .ok_or_else(|| {
                    AppError::with_message(ErrorCode::IOError, "Invalid timestamp")
                })?
                .naive_utc()
        };

        // 计算文件校验和
        let checksum = self.calculate_checksum(file_path).await?;

        // 解析插件元数据
        let metadata = self.parse_plugin_metadata(file_path).await?;

        // 保存用于日志的值
        let plugin_name = metadata.name.clone();
        let plugin_code = metadata.code.clone();

        // 创建插件文件信息
        let plugin_info = PluginFileInfo {
            name: metadata.name,
            code: metadata.code,
            version: metadata.version,
            plugin_type: metadata.plugin_type,
            description: metadata.description,
            author: metadata.author,
            homepage: metadata.homepage,
            class_name: metadata.class_name,
            config: metadata.config,
            dependencies: metadata.dependencies,
            file_path: file_path_str,
            file_size,
            modified_time,
            checksum,
        };

        // 缓存已加载的文件
        self.loaded_files
            .insert(plugin_code.clone(), plugin_info.clone());

        info!("Loaded plugin file: {} ({})", plugin_name, file_name);
        Ok(plugin_info)
    }

    /// 解析插件元数据
    async fn parse_plugin_metadata(
        &self,
        file_path: &Path,
    ) -> Result<PluginMetadata, AppError> {
        // TODO: 实现插件元数据解析
        // 这里应该从插件文件中读取元数据信息
        // 目前返回示例数据

        let file_extension = file_path.extension().and_then(|ext| ext.to_str());

        match file_extension {
            Some("zip") | Some("jar") => {
                // 解析压缩文件中的元数据
                self.parse_archive_metadata(file_path).await
            }
            Some("so") | Some("dll") | Some("dylib") => {
                // 解析动态库中的元数据
                self.parse_library_metadata(file_path).await
            }
            _ => Err(AppError::with_message(
                ErrorCode::InvalidInput,
                "Unsupported plugin file format"
            )),
        }
    }

    /// 解析压缩文件元数据
    async fn parse_archive_metadata(
        &self,
        file_path: &Path,
    ) -> Result<PluginMetadata, AppError> {
        // TODO: 实现ZIP/JAR文件解析
        // 这里应该读取压缩文件中的plugin.json或其他元数据文件
        // 目前返回示例数据

        Ok(PluginMetadata {
            name: "示例插件".to_string(),
            code: "example-plugin".to_string(),
            version: "1.0.0".to_string(),
            plugin_type: 0,
            description: Some("这是一个示例插件".to_string()),
            author: Some("系统".to_string()),
            homepage: None,
            class_name: "ExamplePlugin".to_string(),
            config: None,
            dependencies: None,
            permissions: None,
            entry_point: None,
        })
    }

    /// 解析动态库元数据
    async fn parse_library_metadata(
        &self,
        file_path: &Path,
    ) -> Result<PluginMetadata, AppError> {
        // TODO: 实现动态库解析
        // 这里应该读取动态库中的符号信息或元数据
        // 目前返回示例数据

        Ok(PluginMetadata {
            name: "示例插件".to_string(),
            code: "example-plugin".to_string(),
            version: "1.0.0".to_string(),
            plugin_type: 0,
            description: Some("这是一个示例插件".to_string()),
            author: Some("系统".to_string()),
            homepage: None,
            class_name: "ExamplePlugin".to_string(),
            config: None,
            dependencies: None,
            permissions: None,
            entry_point: None,
        })
    }

    /// 计算文件校验和
    async fn calculate_checksum(
        &self,
        file_path: &Path,
    ) -> Result<String, AppError> {
        // TODO: 实现文件校验和计算
        // 这里应该使用SHA256或其他哈希算法计算文件校验和
        // 目前返回空字符串

        Ok("".to_string())
    }

    /// 验证插件文件
    pub async fn validate_plugin_file(
        &self,
        file_path: &Path,
    ) -> Result<PluginValidationResult, AppError> {
        info!("Validating plugin file: {:?}", file_path);

        // 检查文件是否存在
        if !file_path.exists() {
            return Ok(PluginValidationResult {
                is_valid: false,
                errors: vec!["Plugin file does not exist".to_string()],
                warnings: vec![],
            });
        }

        // 检查文件大小
        let file_metadata = fs::metadata(file_path).map_err(|e| {
            error!("Failed to get file metadata: {:?}", e);
            AppError::with_message(ErrorCode::IOError, "Failed to get file metadata")
        })?;

        let file_size = file_metadata.len();

        if file_size == 0 {
            return Ok(PluginValidationResult {
                is_valid: false,
                errors: vec!["Plugin file is empty".to_string()],
                warnings: vec![],
            });
        }

        // 解析插件元数据
        let plugin_metadata = self.parse_plugin_metadata(file_path).await?;

        // 验证必要字段
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        if plugin_metadata.name.is_empty() {
            errors.push("Plugin name is required".to_string());
        }

        if plugin_metadata.code.is_empty() {
            errors.push("Plugin code is required".to_string());
        }

        if plugin_metadata.version.is_empty() {
            errors.push("Plugin version is required".to_string());
        }

        if plugin_metadata.class_name.is_empty() {
            errors.push("Plugin class name is required".to_string());
        }

        // 检查文件大小（如果过大）
        if file_size > 100 * 1024 * 1024 {
            warnings.push("Plugin file is larger than 100MB".to_string());
        }

        let is_valid = errors.is_empty();

        Ok(PluginValidationResult {
            is_valid,
            errors,
            warnings,
        })
    }

    /// 获取已加载的插件文件
    pub fn get_loaded_files(&self) -> &HashMap<String, PluginFileInfo> {
        &self.loaded_files
    }

    /// 清除已加载的文件缓存
    pub fn clear_loaded_files(&mut self) {
        self.loaded_files.clear();
    }
}

/// 插件验证结果
#[derive(Debug, Clone)]
pub struct PluginValidationResult {
    /// 是否有效
    pub is_valid: bool,
    /// 错误列表
    pub errors: Vec<String>,
    /// 警告列表
    pub warnings: Vec<String>,
}
