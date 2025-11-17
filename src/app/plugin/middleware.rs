/// 插件中间件模块
/// 提供插件状态检查和权限验证

use axum::{
    extract::Request,
    middleware::Next,
    response::{Response, IntoResponse},
    http::StatusCode,
    Json,
};
use crate::common::exception::ErrorCode;
use crate::app::plugin::cache::PluginCacheManager;

/// 插件状态检查中间件
/// 验证插件是否启用，如果未启用则拒绝请求
pub async fn check_plugin_status(
    plugin_name: &str,
    req: Request,
    next: Next,
) -> Result<Response, Response> {
    // 检查插件是否启用
    match PluginCacheManager::is_plugin_enabled(plugin_name).await {
        Ok(enabled) => {
            if !enabled {
                tracing::warn!("Plugin {} is not enabled", plugin_name);
                let error_response = crate::common::response::ResponseModel::<()> {
                    code: ErrorCode::Forbidden.code(),
                    msg: format!("插件 {} 未启用，请联系系统管理员", plugin_name),
                    data: None,
                };
                return Err((StatusCode::FORBIDDEN, Json(error_response)).into_response());
            }
            // 插件已启用，继续处理请求
            Ok(next.run(req).await)
        }
        Err(e) => {
            tracing::error!("Failed to check plugin status: {:?}", e);
            let error_response = crate::common::response::ResponseModel::<()> {
                code: ErrorCode::InternalServerError.code(),
                msg: "检查插件状态失败".to_string(),
                data: None,
            };
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)).into_response())
        }
    }
}

/// 创建插件状态检查中间件
/// 使用宏方式创建闭包来避免Clone问题
#[macro_export]
macro_rules! plugin_middleware {
    ($plugin_name:expr) => {
        |req: Request, next: Next| async move {
            $crate::app::plugin::middleware::check_plugin_status($plugin_name, req, next).await
        }
    };
}

#[cfg(test)]
mod tests {
    

    #[tokio::test]
    async fn test_plugin_status_check() {
        // 这里需要mock Redis连接进行测试
        // 实际测试需要Redis环境
    }
}
