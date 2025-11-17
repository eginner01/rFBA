/// 插件管理API

use axum::{
    extract::{Path, Query},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use crate::common::exception::{AppError, ErrorCode};
use crate::common::response::ResponseModel;

/// 插件类型枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PluginType {
    Zip,
    Git,
}

#[derive(Debug, Deserialize)]
pub struct InstallPluginQuery {
    #[serde(rename = "type")]
    pub plugin_type: PluginType,
    pub repo_url: Option<String>,
}

/// 插件信息响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginResponse {
    pub plugin: PluginConfigResponse,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app: Option<AppConfigResponse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api: Option<serde_json::Value>,  // 使用Value因为api结构是动态的
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginConfigResponse {
    pub summary: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub enable: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfigResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extend: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub router: Option<Vec<String>>,
}

/// 获取所有插件
/// GET /api/v1/sys/plugin
pub async fn get_all_plugins() -> Result<impl IntoResponse, AppError> {
    // 直接从Redis读取插件信息
    use crate::database::redis::RedisManager;
    use crate::core::SETTINGS;
    use redis::AsyncCommands;
    
    let mut conn = RedisManager::get_connection().await
        .map_err(|e| AppError::with_message(ErrorCode::RedisError, format!("Failed to get Redis connection: {}", e)))?;
    
    let prefix = &SETTINGS.plugin_redis_prefix;
    let pattern = format!("{}:*", prefix);
    
    // 扫描所有插件键
    let keys: Vec<String> = conn.keys(&pattern).await
        .map_err(|e| AppError::with_message(ErrorCode::RedisError, format!("Failed to scan plugin keys: {}", e)))?;
    
    let mut plugins = Vec::new();
    
    for key in keys {
        // 跳过changed键
        if key.ends_with(":changed") {
            continue;
        }
        
        let plugin_data: Option<String> = conn.get(&key).await
            .map_err(|e| AppError::with_message(ErrorCode::RedisError, format!("Failed to get plugin data: {}", e)))?;
        
        if let Some(data) = plugin_data {
            match serde_json::from_str::<PluginResponse>(&data) {
                Ok(plugin) => plugins.push(plugin),
                Err(e) => {
                    tracing::warn!("Failed to parse plugin data for key {}: {}", key, e);
                    continue;
                }
            }
        }
    }
    
    Ok(Json(ResponseModel {
        code: 200,
        msg: "请求成功".to_string(),
        data: Some(plugins),
    }))
}

/// 检查插件是否变更
/// GET /api/v1/sys/plugin/changed
pub async fn plugin_changed() -> Result<impl IntoResponse, AppError> {
    use crate::app::plugin::PluginCacheManager;
    
    let changed = PluginCacheManager::is_plugin_changed().await?;
    
    Ok(Json(ResponseModel {
        code: 200,
        msg: "查询成功".to_string(),
        data: Some(changed),
    }))
}

/// 安装插件
/// POST /api/v1/sys/plugin
pub async fn install_plugin(
    Query(query): Query<InstallPluginQuery>,
    multipart: Option<axum::extract::Multipart>,
) -> Result<Json<ResponseModel<String>>, AppError> {
    match query.plugin_type {
        PluginType::Zip => {
            let multipart = multipart.ok_or_else(|| {
                AppError::with_message(ErrorCode::InvalidInput, "ZIP 压缩包不能为空")
            })?;
            
            // 调用ZIP安装函数
            let result = crate::app::plugin::install_zip_plugin(multipart).await?;
            
            Ok(Json(ResponseModel {
                code: 200,
                msg: result.message,
                data: Some(result.plugin_name),
            }))
        }
        PluginType::Git => {
            let repo_url = query.repo_url.ok_or_else(|| {
                AppError::with_message(ErrorCode::InvalidInput, "Git 仓库地址不能为空")
            })?;
            
            // 调用Git安装函数
            let result = crate::app::plugin::install_git_plugin(repo_url).await?;
            
            Ok(Json(ResponseModel {
                code: 200,
                msg: result.message,
                data: Some(result.plugin_name),
            }))
        }
    }
}

/// 卸载插件
/// DELETE /api/v1/sys/plugin/{plugin}
pub async fn uninstall_plugin(
    Path(plugin_name): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    use crate::app::plugin::PluginCacheManager;
    use std::fs;
    use chrono::Local;
    
    // 检查插件是否存在
    let plugin_dir = format!("./plugins/{}", plugin_name);
    if !std::path::Path::new(&plugin_dir).exists() {
        return Err(AppError::with_message(
            ErrorCode::NotFound,
            "插件不存在"
        ));
    }
    
    // 备份插件目录
    let timestamp = Local::now().format("%Y%m%d%H%M%S");
    let backup_dir = format!("./plugins/{}.{}.backup", plugin_name, timestamp);
    
    fs::rename(&plugin_dir, &backup_dir).map_err(|e| {
        tracing::error!("Failed to backup plugin: {:?}", e);
        AppError::with_message(ErrorCode::IOError, "插件备份失败")
    })?;
    
    // 删除Redis缓存
    PluginCacheManager::delete_plugin_cache(&plugin_name).await?;
    
    // 标记变更
    PluginCacheManager::mark_plugin_changed().await?;
    
    Ok(Json(ResponseModel {
        code: 200,
        msg: format!("插件 {} 卸载成功，请根据插件说明（README.md）移除相关配置并重启服务", plugin_name),
        data: Some(()),
    }))
}

/// 更新插件状态
/// PUT /api/v1/sys/plugin/{plugin}/status
pub async fn update_plugin_status(
    Path(plugin_name): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    use crate::app::plugin::PluginCacheManager;
    
    // 获取当前状态
    let current_enabled = PluginCacheManager::is_plugin_enabled(&plugin_name).await?;
    
    // 切换状态
    let new_status = !current_enabled;
    PluginCacheManager::update_plugin_status(&plugin_name, new_status).await?;
    
    Ok(Json(ResponseModel {
        code: 200,
        msg: "更新成功".to_string(),
        data: Some(()),
    }))
}

/// 下载插件
/// GET /api/v1/sys/plugin/{plugin}
pub async fn download_plugin(
    Path(plugin_name): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    use std::path::Path;
    use std::fs;
    use zip::ZipWriter;
    use zip::write::FileOptions;
    use std::io::{Write, Cursor};
    
    let plugin_dir = format!("./plugins/{}", plugin_name);
    let plugin_path = Path::new(&plugin_dir);
    
    if !plugin_path.exists() {
        return Err(AppError::with_message(
            ErrorCode::NotFound,
            "插件不存在"
        ));
    }
    
    // 创建ZIP
    let mut buffer = Cursor::new(Vec::new());
    {
        let mut zip = ZipWriter::new(&mut buffer);
        let options = FileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated);
        
        // 遍历插件目录，包含plugin.toml等所有文件
        for entry in walkdir::WalkDir::new(plugin_path)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            
            if path.is_dir() {
                continue;
            }
            
            let relative_path = path.strip_prefix("./plugins/")
                .or_else(|_| path.strip_prefix("plugins/"))
                .unwrap_or(path);
            
            let zip_path = relative_path.to_string_lossy().to_string();
            
            // 跳过隐藏文件和备份文件
            if zip_path.contains("/.") || zip_path.ends_with(".backup") {
                continue;
            }
            
            zip.start_file(zip_path.clone(), options)
                .map_err(|e| AppError::with_message(ErrorCode::IOError, format!("ZIP创建失败: {}", e)))?;
            
            let content = fs::read(path)
                .map_err(|e| AppError::with_message(ErrorCode::IOError, format!("文件读取失败: {}", e)))?;
            
            zip.write_all(&content)
                .map_err(|e| AppError::with_message(ErrorCode::IOError, format!("ZIP写入失败: {}", e)))?;
            
            tracing::debug!("Added to ZIP: {}", zip_path);
        }
        
        zip.finish()
            .map_err(|e| AppError::with_message(ErrorCode::IOError, format!("ZIP完成失败: {}", e)))?;
    }
    
    let bytes = buffer.into_inner();
    
    use axum::response::Response;
    use axum::body::Body;
    use axum::http::header;
    
    let mut response = Response::new(Body::from(bytes));
    response.headers_mut().insert(
        header::CONTENT_TYPE,
        "application/x-zip-compressed".parse().unwrap(),
    );
    response.headers_mut().insert(
        header::CONTENT_DISPOSITION,
        format!("attachment; filename={}.zip", plugin_name).parse().unwrap(),
    );
    
    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_type_serde() {
        let zip = PluginType::Zip;
        let json = serde_json::to_string(&zip).unwrap();
        assert_eq!(json, "\"zip\"");
    }
}
