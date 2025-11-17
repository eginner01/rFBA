//! 代码生成器错误定义

use thiserror::Error;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

/// 代码生成器错误类型
#[derive(Error, Debug)]
pub enum CodeGenError {
    /// 数据库错误
    #[error("数据库错误: {0}")]
    DatabaseError(String),
    
    /// 生成错误
    #[error("代码生成失败: {0}")]
    GenerateError(String),
    
    /// 模板错误
    #[error("模板错误: {0}")]
    TemplateError(String),
    
    /// 验证错误
    #[error("验证错误: {0}")]
    ValidationError(String),
    
    /// 资源未找到
    #[error("资源未找到: {0}")]
    NotFound(String),
}

impl IntoResponse for CodeGenError {
    fn into_response(self) -> Response {
        let (status, code, message) = match self {
            CodeGenError::DatabaseError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, 500, msg),
            CodeGenError::GenerateError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, 500, msg),
            CodeGenError::TemplateError(msg) => (StatusCode::BAD_REQUEST, 400, msg),
            CodeGenError::ValidationError(msg) => (StatusCode::UNPROCESSABLE_ENTITY, 422, msg),
            CodeGenError::NotFound(msg) => (StatusCode::NOT_FOUND, 404, msg),
        };

        let body = Json(json!({
            "code": code,
            "msg": message,
        }));

        (status, body).into_response()
    }
}

impl From<validator::ValidationErrors> for CodeGenError {
    fn from(errors: validator::ValidationErrors) -> Self {
        let messages: Vec<String> = errors
            .field_errors()
            .iter()
            .flat_map(|(_, errs)| {
                errs.iter()
                    .filter_map(|e| e.message.as_ref().map(|m| m.to_string()))
            })
            .collect();
        
        CodeGenError::ValidationError(messages.join(", "))
    }
}
