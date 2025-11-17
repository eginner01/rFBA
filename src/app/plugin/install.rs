//! 插件安装模块
//! 支持ZIP压缩包和Git仓库两种安装方式

use std::fs;
use std::io::Cursor;
use std::path::PathBuf;
use axum::extract::Multipart;
use tracing::{info, error};
use zip::ZipArchive;
use regex::Regex;

use crate::common::exception::{AppError, ErrorCode};
use crate::app::plugin::cache::PluginCacheManager;

/// 插件安装结果
#[derive(Debug, Clone)]
pub struct InstallResult {
    pub plugin_name: String,
    pub success: bool,
    pub message: String,
}

/// 从ZIP压缩包安装插件
/// 
/// 流程：
/// 1. 验证ZIP格式
/// 2. 检查必要文件（plugin.toml, README.md）
/// 3. 验证插件名称和是否已安装
/// 4. 解压到plugins目录
/// 5. 标记插件变更
pub async fn install_zip_plugin(mut multipart: Multipart) -> Result<InstallResult, AppError> {
    info!("Starting ZIP plugin installation...");
    
    // 读取上传的文件
    let mut zip_data: Option<Vec<u8>> = None;
    let mut filename = String::new();
    
    while let Some(field) = multipart.next_field().await.map_err(|e| {
        error!("Failed to read multipart field: {:?}", e);
        AppError::with_message(ErrorCode::InvalidInput, "Failed to read upload data")
    })? {
        let field_name = field.name().unwrap_or("").to_string();
        
        if field_name == "file" {
            let raw_filename = field.file_name().unwrap_or("unknown.zip").to_string();
            // 清理文件名：移除括号及其内容，如 "dict (1).zip" -> "dict.zip"
            filename = sanitize_filename(&raw_filename);
            info!("Original filename: {}, sanitized: {}", raw_filename, filename);
            
            let data = field.bytes().await.map_err(|e| {
                error!("Failed to read file bytes: {:?}", e);
                AppError::with_message(ErrorCode::InvalidInput, "Failed to read file content")
            })?;
            zip_data = Some(data.to_vec());
        }
    }
    
    let zip_data = zip_data.ok_or_else(|| {
        AppError::with_message(ErrorCode::InvalidInput, "No file uploaded")
    })?;
    
    info!("Received file: {} ({} bytes)", filename, zip_data.len());
    
    // 验证ZIP格式
    let cursor = Cursor::new(&zip_data);
    let mut archive = ZipArchive::new(cursor).map_err(|e| {
        error!("Invalid ZIP format: {:?}", e);
        AppError::with_message(ErrorCode::InvalidInput, "插件压缩包格式非法")
    })?;
    
    // 获取ZIP文件列表（统一使用正斜杠，兼容 Windows 和 Unix 路径）
    let file_list: Vec<String> = (0..archive.len())
        .filter_map(|i| {
            archive.by_index(i).ok().map(|f| {
                // 将所有反斜杠替换为正斜杠，确保路径统一
                f.name().replace('\\', "/")
            })
        })
        .collect();
    
    if file_list.is_empty() {
        return Err(AppError::with_message(ErrorCode::InvalidInput, "插件压缩包内容非法"));
    }
    
    // 获取插件目录名（第一个文件的顶级目录）
    let plugin_dir_name = file_list[0]
        .split('/')
        .next()
        .ok_or_else(|| AppError::with_message(ErrorCode::InvalidInput, "插件压缩包结构非法"))?
        .to_string();
    
    info!("Plugin directory name: {}", plugin_dir_name);
    info!("ZIP file list (first 5): {:?}", &file_list[..file_list.len().min(5)]);
    
    // 验证必要文件存在（只检查 plugin.toml）
    let plugin_toml = format!("{}/plugin.toml", plugin_dir_name);
    
    info!("Looking for required file: {}", plugin_toml);
    
    if file_list.len() <= 2 {
        return Err(AppError::with_message(ErrorCode::InvalidInput, "插件压缩包内容非法"));
    }
    
    if !file_list.contains(&plugin_toml) {
        return Err(AppError::with_message(
            ErrorCode::InvalidInput,
            "插件压缩包内缺少必要文件: plugin.toml"
        ));
    }
    
    // 提取插件名称（从文件名中）
    let re = Regex::new(r"^([a-zA-Z0-9_]+)").unwrap();
    let plugin_name = re.captures(&filename.replace(".zip", ""))
        .and_then(|caps| caps.get(1))
        .map(|m| m.as_str().to_string())
        .ok_or_else(|| {
            AppError::with_message(ErrorCode::InvalidInput, "无法从文件名提取插件名称")
        })?;
    
    info!("Extracted plugin name: {}", plugin_name);
    
    // 检查插件是否已安装
    let plugin_path = PathBuf::from("./plugins").join(&plugin_name);
    if plugin_path.exists() {
        return Err(AppError::with_message(ErrorCode::Conflict, "此插件已安装"));
    }
    
    // 创建插件目录
    fs::create_dir_all(&plugin_path).map_err(|e| {
        error!("Failed to create plugin directory: {:?}", e);
        AppError::with_message(ErrorCode::IOError, "创建插件目录失败")
    })?;
    
    // 解压文件
    info!("Extracting plugin files to {:?}", plugin_path);
    let cursor = Cursor::new(&zip_data);
    let mut archive = ZipArchive::new(cursor).unwrap();
    
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).map_err(|e| {
            error!("Failed to read ZIP entry: {:?}", e);
            AppError::with_message(ErrorCode::IOError, "读取压缩包文件失败")
        })?;
        
        // 统一路径分隔符为正斜杠
        let file_path = file.name().replace('\\', "/");
        
        // 只处理插件目录下的文件
        if !file_path.starts_with(&plugin_dir_name) {
            continue;
        }
        
        // 移除顶级目录前缀
        let relative_path = file_path
            .strip_prefix(&format!("{}/", plugin_dir_name))
            .unwrap_or("");
        
        if relative_path.is_empty() {
            continue;
        }
        
        let target_path = plugin_path.join(relative_path);
        
        if file.is_dir() {
            fs::create_dir_all(&target_path).map_err(|e| {
                error!("创建目录失败: {:?}", e);
                AppError::with_message(ErrorCode::IOError, "创建目录失败")
            })?;
        } else {
            // 确保父目录存在
            if let Some(parent) = target_path.parent() {
                fs::create_dir_all(parent).map_err(|e| {
                    error!("创建父目录失败: {:?}", e);
                    AppError::with_message(ErrorCode::IOError, "创建父目录失败")
                })?;
            }
            
            // 写入文件
            let mut outfile = fs::File::create(&target_path).map_err(|e| {
                error!("创建文件失败: {:?}", e);
                AppError::with_message(ErrorCode::IOError, "创建文件失败")
            })?;
            
            std::io::copy(&mut file, &mut outfile).map_err(|e| {
                error!("写入文件失败: {:?}", e);
                AppError::with_message(ErrorCode::IOError, "写入文件失败")
            })?;
        }
    }
    
    info!("插件文件解压成功");
    
    // 立即加载插件配置到Redis（这样前端可以立即看到新插件）
    info!("正在加载插件配置到 Redis...");
    match crate::app::plugin::init::load_single_plugin(&plugin_name).await {
        Ok(_) => info!("插件配置已加载到 Redis"),
        Err(e) => {
            error!("加载插件配置失败: {:?}", e);
            return Err(AppError::with_message(
                ErrorCode::OperationFailed,
                format!("插件文件已安装但配置加载失败: {}", e)
            ));
        }
    }
    
    // 标记插件变更
    PluginCacheManager::mark_plugin_changed().await?;
    
    info!("Plugin {} installed successfully", plugin_name);
    Ok(InstallResult {
        plugin_name: plugin_name.clone(),
        success: true,
        message: format!("插件 {} 安装成功", plugin_name),
    })
}

/// 从Git仓库安装插件
/// 
/// 流程：
/// 1. 验证Git URL格式
/// 2. 提取仓库名称
/// 3. 检查是否已安装
/// 4. 克隆仓库到plugins目录
/// 5. 标记插件变更
pub async fn install_git_plugin(repo_url: String) -> Result<InstallResult, AppError> {
    info!("Starting Git plugin installation from: {}", repo_url);
    
    // 验证Git URL格式
    let re = Regex::new(r"(?:https?://|git@)[\w\.\-]+(?:/|:)(?P<repo>[\w\-]+)(?:\.git)?$").unwrap();
    let captures = re.captures(&repo_url).ok_or_else(|| {
        AppError::with_message(ErrorCode::InvalidInput, "Git 仓库地址格式非法")
    })?;
    
    let repo_name = captures.name("repo")
        .map(|m| m.as_str().to_string())
        .ok_or_else(|| {
            AppError::with_message(ErrorCode::InvalidInput, "无法从URL提取仓库名称")
        })?;
    
    info!("Repository name: {}", repo_name);
    
    // 检查插件是否已安装
    let plugin_path = PathBuf::from("./plugins").join(&repo_name);
    if plugin_path.exists() {
        return Err(AppError::with_message(ErrorCode::Conflict, format!("{} 插件已安装", repo_name)));
    }
    
    // 使用git2克隆仓库
    info!("正在克隆仓库...");
    
    use git2::Repository;
    Repository::clone(&repo_url, &plugin_path).map_err(|e| {
        error!("Git 克隆失败: {:?}", e);
        AppError::with_message(ErrorCode::OperationFailed, "插件安装失败，请稍后重试")
    })?;
    
    info!("仓库克隆成功");
    
    // 验证必要文件
    let plugin_toml = plugin_path.join("plugin.toml");
    if !plugin_toml.exists() {
        // 清理已克隆的目录
        let _ = fs::remove_dir_all(&plugin_path);
        return Err(AppError::with_message(
            ErrorCode::InvalidInput,
            "插件仓库中缺少 plugin.toml 文件"
        ));
    }
    
    // 立即加载插件配置到Redis
    info!("正在加载插件配置到 Redis...");
    match crate::app::plugin::init::load_single_plugin(&repo_name).await {
        Ok(_) => info!("插件配置已加载到 Redis"),
        Err(e) => {
            error!("加载插件配置失败: {:?}", e);
            // 清理已克隆的目录
            let _ = fs::remove_dir_all(&plugin_path);
            return Err(AppError::with_message(
                ErrorCode::OperationFailed,
                format!("插件文件已安装但配置加载失败: {}", e)
            ));
        }
    }
    
    // 标记插件变更
    PluginCacheManager::mark_plugin_changed().await?;
    
    info!("插件 {} 安装成功", repo_name);
    
    Ok(InstallResult {
        plugin_name: repo_name.clone(),
        success: true,
        message: format!("插件 {} 安装成功", repo_name),
    })
}

/// 清理文件名，移除特殊字符
/// 
/// 规则：
/// - 移除括号及其内容：`dict (1).zip` -> `dict.zip`
/// - 移除多余空格
/// - 保留扩展名
fn sanitize_filename(filename: &str) -> String {
    let parts: Vec<&str> = filename.rsplitn(2, '.').collect();
    let (basename, extension) = if parts.len() == 2 {
        (parts[1], parts[0])
    } else {
        (filename, "")
    };
    
    let re = Regex::new(r"\s*\([^)]*\)\s*").unwrap();
    let cleaned = re.replace_all(basename, "").trim().to_string();
    
    let re_special = Regex::new(r"[^\w\-_]").unwrap();
    let cleaned = re_special.replace_all(&cleaned, "").to_string();
    
    if extension.is_empty() {
        cleaned
    } else {
        format!("{}.{}", cleaned, extension)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_git_url_regex() {
        let re = Regex::new(r"(?:https?://|git@)[\w\.\-]+(?:/|:)(?P<repo>[\w\-]+)(?:\.git)?$").unwrap();
        
        let urls = vec![
            ("https://github.com/user/my-plugin.git", Some("my-plugin")),
            ("https://github.com/user/my-plugin", Some("my-plugin")),
            ("git@github.com:user/my-plugin.git", Some("my-plugin")),
            ("invalid-url", None),
        ];
        
        for (url, expected) in urls {
            let result = re.captures(url).and_then(|c| c.name("repo")).map(|m| m.as_str());
            assert_eq!(result, expected, "Failed for URL: {}", url);
        }
    }
    
    #[test]
    fn test_plugin_name_extraction() {
        let re = Regex::new(r"^([a-zA-Z0-9_]+)").unwrap();
        
        let filenames = vec![
            ("my_plugin.zip", Some("my_plugin")),
            ("test-plugin-v1.0.zip", Some("test")),
            ("plugin123.zip", Some("plugin123")),
        ];
        
        for (filename, expected) in filenames {
            let name = filename.replace(".zip", "");
            let result = re.captures(&name).and_then(|c| c.get(1)).map(|m| m.as_str());
            assert_eq!(result, expected, "Failed for filename: {}", filename);
        }
    }
    
    #[test]
    fn test_sanitize_filename() {
        assert_eq!(sanitize_filename("dict (1).zip"), "dict.zip");
        assert_eq!(sanitize_filename("plugin (copy).zip"), "plugin.zip");
        assert_eq!(sanitize_filename("test(2)(3).zip"), "test.zip");
        assert_eq!(sanitize_filename("my-plugin.zip"), "my-plugin.zip");
    }
}
