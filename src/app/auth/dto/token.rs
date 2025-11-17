/// Token相关 DTO

use serde::{Deserialize, Serialize};

/// 刷新Token请求 DTO
#[derive(Debug, Deserialize)]
pub struct RefreshTokenRequest {
    /// 刷新令牌
    pub refresh_token: String,
}

/// 刷新Token响应 DTO - 匹配Python后端的GetNewToken
#[derive(Debug, Serialize)]
pub struct RefreshTokenResponse {
    /// 访问令牌
    pub access_token: String,
    /// 令牌过期时间
    pub access_token_expire_time: chrono::NaiveDateTime,
    /// 会话 UUID
    pub session_uuid: String,
}

/// 登出请求 DTO
#[derive(Debug, Deserialize)]
pub struct LogoutRequest {
    /// 访问令牌（可选，后端也可从请求头获取）
    pub access_token: Option<String>,
}
