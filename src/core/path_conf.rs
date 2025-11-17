/// 路径配置
/// 用于定义项目中各种文件路径和目录

use std::path::PathBuf;

/// 项目根目录
/// 获取当前可执行文件的父目录
pub fn current_exe_dir() -> PathBuf {
    std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

/// 项目根目录 - 使用环境变量或当前执行目录
pub static BASE_PATH: once_cell::sync::Lazy<PathBuf> = once_cell::sync::Lazy::new(|| {
    std::env::var("FBA_BASE_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| current_exe_dir())
});

/// 日志目录
pub static LOG_DIR: once_cell::sync::Lazy<PathBuf> = once_cell::sync::Lazy::new(|| {
    let path = BASE_PATH.join("logs");
    if !path.exists() {
        std::fs::create_dir_all(&path).unwrap_or_default();
    }
    path
});

/// 静态文件目录
pub static STATIC_DIR: once_cell::sync::Lazy<PathBuf> = once_cell::sync::Lazy::new(|| {
    BASE_PATH.join("static")
});

/// 上传文件目录
pub static UPLOAD_DIR: once_cell::sync::Lazy<PathBuf> = once_cell::sync::Lazy::new(|| {
    let path = STATIC_DIR.join("upload");
    if !path.exists() {
        std::fs::create_dir_all(&path).unwrap_or_default();
    }
    path
});

/// 插件目录
pub static PLUGIN_DIR: once_cell::sync::Lazy<PathBuf> = once_cell::sync::Lazy::new(|| {
    BASE_PATH.join("plugin")
});

/// 国际化文件目录
pub static LOCALE_DIR: once_cell::sync::Lazy<PathBuf> = once_cell::sync::Lazy::new(|| {
    BASE_PATH.join("locale")
});

/// 配置文件目录
pub static CONFIG_DIR: once_cell::sync::Lazy<PathBuf> = once_cell::sync::Lazy::new(|| {
    BASE_PATH.join("config")
});
