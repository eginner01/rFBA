/// 请求日志中间件
/// 在 Debug 模式下记录详细的请求信息

use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
};
use tracing::{info, debug};
use crate::core::SETTINGS;

/// 请求日志中间件
/// 在 debug 模式下显示详细的请求信息，包括完整 URL、方法、查询参数等
pub async fn middleware(request: Request, next: Next) -> Response {
    if SETTINGS.debug_mode {
        // 提取请求信息
        let method = request.method().clone();
        let uri = request.uri().clone();
        let path = uri.path();
        let query = uri.query().unwrap_or("");
        
        // 构建完整 URL
        let full_url = if query.is_empty() {
            path.to_string()
        } else {
            format!("{}?{}", path, query)
        };

        // 获取客户端 IP（如果有）
        let client_ip = request
            .headers()
            .get("x-forwarded-for")
            .and_then(|v| v.to_str().ok())
            .or_else(|| {
                request.headers()
                    .get("x-real-ip")
                    .and_then(|v| v.to_str().ok())
            })
            .unwrap_or("unknown");

        // 记录请求开始
        info!(
            "📥 {} {} | 客户端: {}",
            method,
            full_url,
            client_ip
        );

        // 如果有 Content-Type，也显示
        if let Some(content_type) = request.headers().get("content-type") {
            if let Ok(ct) = content_type.to_str() {
                debug!("   Content-Type: {}", ct);
            }
        }

        // 执行请求
        let start = std::time::Instant::now();
        let response = next.run(request).await;
        let elapsed = start.elapsed();

        // 记录响应
        let status = response.status();
        let status_emoji = if status.is_success() {
            "✅"
        } else if status.is_client_error() {
            "⚠️"
        } else if status.is_server_error() {
            "❌"
        } else {
            "ℹ️"
        };

        info!(
            "{} {} {} | 状态: {} | 耗时: {}ms",
            status_emoji,
            method,
            full_url,
            status.as_u16(),
            elapsed.as_millis()
        );

        response
    } else {
        // 普通模式：直接执行，不记录
        next.run(request).await
    }
}
