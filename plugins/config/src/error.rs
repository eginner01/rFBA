//! 配置插件错误定义

use thiserror::Error;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

/// 配置插件错误类型
#[derive(Error, Debug)]
pub enum ConfigError {
    /// 数据库错误
    #[error("数据库错误: {0}")]
    DatabaseError(String),
    
    /// 资源未找到
    #[error("资源未找到: {0}")]
    NotFound(String),
    
    /// 资源已存在
    #[error("资源已存在: {0}")]
    AlreadyExists(String),
    
    /// 操作失败
    #[error("操作失败: {0}")]
    OperationFailed(String),
    
    /// 验证错误
    #[error("验证错误: {0}")]
    ValidationError(String),
    
    /// Redis错误
    #[error("缓存错误: {0}")]
    RedisError(String),
    
    /// 权限不足
    #[error("权限不足")]
    PermissionDenied,
}

impl IntoResponse for ConfigError {
    fn into_response(self) -> Response {
        let (status, code, message) = match self {
            ConfigError::DatabaseError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, 500, msg),
            ConfigError::NotFound(msg) => (StatusCode::NOT_FOUND, 404, msg),
            ConfigError::AlreadyExists(msg) => (StatusCode::CONFLICT, 409, msg),
            ConfigError::OperationFailed(msg) => (StatusCode::BAD_REQUEST, 400, msg),
            ConfigError::ValidationError(msg) => (StatusCode::UNPROCESSABLE_ENTITY, 422, msg),
            ConfigError::RedisError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, 500, msg),
            ConfigError::PermissionDenied => (
                StatusCode::FORBIDDEN,
                403,
                "权限不足".to_string(),
            ),
        };

        let body = Json(json!({
            "code": code,
            "msg": message,
        }));

        (status, body).into_response()
    }
}

/// 将validator错误转换为ConfigError
impl From<validator::ValidationErrors> for ConfigError {
    fn from(errors: validator::ValidationErrors) -> Self {
        let messages: Vec<String> = errors
            .field_errors()
            .iter()
            .flat_map(|(_, errs)| {
                errs.iter()
                    .filter_map(|e| e.message.as_ref().map(|m| m.to_string()))
            })
            .collect();
        
        ConfigError::ValidationError(messages.join(", "))
    }
}
