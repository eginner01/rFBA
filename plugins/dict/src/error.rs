//! 字典插件错误定义

use thiserror::Error;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

/// 字典插件错误类型
#[derive(Error, Debug)]
pub enum DictError {
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
    
    /// 权限不足
    #[error("权限不足")]
    PermissionDenied,
}

impl IntoResponse for DictError {
    fn into_response(self) -> Response {
        let (status, code, message) = match self {
            DictError::DatabaseError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, 500, msg),
            DictError::NotFound(msg) => (StatusCode::NOT_FOUND, 404, msg),
            DictError::AlreadyExists(msg) => (StatusCode::CONFLICT, 409, msg),
            DictError::OperationFailed(msg) => (StatusCode::BAD_REQUEST, 400, msg),
            DictError::ValidationError(msg) => (StatusCode::UNPROCESSABLE_ENTITY, 422, msg),
            DictError::PermissionDenied => (
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

/// 将validator错误转换为DictError
impl From<validator::ValidationErrors> for DictError {
    fn from(errors: validator::ValidationErrors) -> Self {
        let messages: Vec<String> = errors
            .field_errors()
            .iter()
            .flat_map(|(_, errs)| {
                errs.iter()
                    .filter_map(|e| e.message.as_ref().map(|m| m.to_string()))
            })
            .collect();
        
        DictError::ValidationError(messages.join(", "))
    }
}
