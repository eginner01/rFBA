/// JWT 认证中间件
use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
    http::header,
};
use serde::{Serialize, Deserialize};
use tracing::{warn, info};

use crate::{
    common::exception::{AppError, ErrorCode},
    utils::encrypt::CryptoUtils,
    core::SETTINGS,
};

/// 认证上下文
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthContext {
    pub user_id: String,
    pub username: String,
    pub roles: Option<Vec<String>>,
    pub token_type: Option<String>,
}

/// 从请求头中提取 JWT Token
fn extract_token_from_headers(
    headers: &axum::http::HeaderMap,
) -> Result<String, AppError> {
    if let Some(auth_header) = headers.get(header::AUTHORIZATION) {
        let auth_str = auth_header.to_str()
            .map_err(|_| AppError::new(ErrorCode::TokenInvalid))?;

        if auth_str.starts_with("Bearer ") {
            let token = auth_str.trim_start_matches("Bearer ").to_string();
            if !token.is_empty() {
                return Ok(token);
            }
        } else if !auth_str.is_empty() {
            return Ok(auth_str.to_string());
        }
    }

    warn!("未找到认证信息");
    Err(AppError::new(ErrorCode::TokenInvalid))
}

/// JWT 认证中间件
pub async fn middleware(
    State(_app_state): State<crate::core::registrar::AppState>,
    request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let method = request.method();
    let path = request.uri().path();

    info!("JWT 中间件处理请求: {} {}", method, path);

    if method == axum::http::Method::OPTIONS {
        info!("OPTIONS 预检请求，直接放行");
        return Ok(next.run(request).await);
    }

    let token = match extract_token_from_headers(request.headers()) {
        Ok(token) => token,
        Err(_) => {
            if path.starts_with("/api/v1/auth/login")
                || path.starts_with("/api/v1/auth/captcha")
                || path == "/"
                || path.starts_with("/health")
                // WebSocket/Socket.IO 握手走的是 auth payload，而不是 HTTP 头，所以这里放行 /ws 下的请求
                || path.starts_with("/ws") {
                warn!("路径 {} 在白名单中，无需认证，直接放行", path);
                return Ok(next.run(request).await);
            }

            warn!("请求路径 {} 需要认证但未提供有效 token", path);
            return Err(AppError::new(ErrorCode::TokenInvalid));
        }
    };

    // 验证 JWT Token
    let payload = match CryptoUtils::verify_jwt(&token, &SETTINGS.token_secret_key) {
        Ok(payload) => payload,
        Err(e) => {
            warn!("JWT 验证失败: {:?}", e);
            return Err(AppError::new(ErrorCode::TokenExpired));
        }
    };

    // 验证payload包含必要字段
    let user_id = payload.sub.clone();
    if user_id.is_empty() {
        warn!("JWT payload 缺少必要字段: sub");
        return Err(AppError::new(ErrorCode::TokenInvalid));
    }

    // 创建认证上下文
    let auth_context = AuthContext {
        user_id: user_id.clone(),                // 用户ID（字符串格式）
        username: user_id,                       // 使用sub作为username临时值
        roles: None,
        token_type: None,
    };

    info!("用户认证成功: {}, Token类型: {:?}",
          auth_context.username,
          auth_context.token_type);

    // 将认证上下文注入到请求扩展中
    let mut request = request;
    request.extensions_mut().insert(auth_context);

    Ok(next.run(request).await)
}
