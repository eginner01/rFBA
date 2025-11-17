//! 邮件插件错误定义

use thiserror::Error;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

/// 邮件插件错误类型
#[derive(Error, Debug)]
pub enum EmailError {
    /// 数据库错误
    #[error("数据库错误: {0}")]
    DatabaseError(String),
    
    /// 资源未找到
    #[error("资源未找到: {0}")]
    NotFound(String),
    
    /// SMTP错误
    #[error("SMTP错误: {0}")]
    SmtpError(String),
    
    /// 模板错误
    #[error("模板错误: {0}")]
    TemplateError(String),
    
    /// 验证错误
    #[error("验证错误: {0}")]
    ValidationError(String),
    
    /// 操作失败
    #[error("操作失败: {0}")]
    OperationFailed(String),
}

impl IntoResponse for EmailError {
    fn into_response(self) -> Response {
        let (status, code, message) = match self {
            EmailError::DatabaseError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, 500, msg),
            EmailError::NotFound(msg) => (StatusCode::NOT_FOUND, 404, msg),
            EmailError::SmtpError(msg) => (StatusCode::BAD_REQUEST, 400, format!("SMTP错误: {}", msg)),
            EmailError::TemplateError(msg) => (StatusCode::BAD_REQUEST, 400, format!("模板错误: {}", msg)),
            EmailError::ValidationError(msg) => (StatusCode::UNPROCESSABLE_ENTITY, 422, msg),
            EmailError::OperationFailed(msg) => (StatusCode::BAD_REQUEST, 400, msg),
        };

        let body = Json(json!({
            "code": code,
            "msg": message,
        }));

        (status, body).into_response()
    }
}

/// 将validator错误转换为EmailError
impl From<validator::ValidationErrors> for EmailError {
    fn from(errors: validator::ValidationErrors) -> Self {
        let messages: Vec<String> = errors
            .field_errors()
            .iter()
            .flat_map(|(_, errs)| {
                errs.iter()
                    .filter_map(|e| e.message.as_ref().map(|m| m.to_string()))
            })
            .collect();
        
        EmailError::ValidationError(messages.join(", "))
    }
}
