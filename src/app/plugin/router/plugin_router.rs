/// 插件路由
/// 路径：/api/v1/sys/plugin

use axum::{routing::{get, post, put, delete}, Router};
use crate::app::plugin::api::plugin_api;

/// 创建插件路由
/// 路径：/api/v1/sys/plugin
pub fn plugin_routes() -> Router {
    Router::new()
        // GET /api/v1/sys/plugin - 获取所有插件
        .route("/", get(plugin_api::get_all_plugins))
        // GET /api/v1/sys/plugin/changed - 检查插件变更
        .route("/changed", get(plugin_api::plugin_changed))
        // POST /api/v1/sys/plugin - 安装插件
        .route("/", post(plugin_api::install_plugin))
        // DELETE /api/v1/sys/plugin/{plugin} - 卸载插件
        .route("/{plugin}", delete(plugin_api::uninstall_plugin))
        // PUT /api/v1/sys/plugin/{plugin}/status - 更新插件状态
        .route("/{plugin}/status", put(plugin_api::update_plugin_status))
        // GET /api/v1/sys/plugin/{plugin} - 下载插件
        .route("/{plugin}", get(plugin_api::download_plugin))
}
