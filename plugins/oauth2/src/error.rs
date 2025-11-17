//! OAuth2错误定义

use thiserror::Error;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

/// OAuth2错误类型
#[derive(Error, Debug)]
pub enum OAuth2Error {
    /// 配置错误
    #[error("配置错误: {0}")]
    ConfigError(String),
    
    /// Token交换错误
    #[error("Token交换失败: {0}")]
    TokenExchangeError(String),
    
    /// API调用错误
    #[error("API调用失败: {0}")]
    ApiError(String),
    
    /// 数据库错误
    #[error("数据库错误: {0}")]
    DatabaseError(String),
    
    /// 资源未找到
    #[error("资源未找到: {0}")]
    NotFound(String),
    
    /// 验证错误
    #[error("验证错误: {0}")]
    ValidationError(String),
}

impl IntoResponse for OAuth2Error {
    fn into_response(self) -> Response {
        let (status, code, message) = match self {
            OAuth2Error::ConfigError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, 500, msg),
            OAuth2Error::TokenExchangeError(msg) => (StatusCode::BAD_REQUEST, 400, msg),
            OAuth2Error::ApiError(msg) => (StatusCode::BAD_GATEWAY, 502, msg),
            OAuth2Error::DatabaseError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, 500, msg),
            OAuth2Error::NotFound(msg) => (StatusCode::NOT_FOUND, 404, msg),
            OAuth2Error::ValidationError(msg) => (StatusCode::UNPROCESSABLE_ENTITY, 422, msg),
        };

        let body = Json(json!({
            "code": code,
            "msg": message,
        }));

        (status, body).into_response()
    }
}

impl From<validator::ValidationErrors> for OAuth2Error {
    fn from(errors: validator::ValidationErrors) -> Self {
        let messages: Vec<String> = errors
            .field_errors()
            .iter()
            .flat_map(|(_, errs)| {
                errs.iter()
                    .filter_map(|e| e.message.as_ref().map(|m| m.to_string()))
            })
            .collect();
        
        OAuth2Error::ValidationError(messages.join(", "))
    }
}
