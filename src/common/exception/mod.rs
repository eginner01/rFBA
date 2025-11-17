/// 异常处理模块
/// 统一的错误处理和异常定义

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use std::fmt;

/// 错误码枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorCode {
    // ===== 通用错误 =====
    /// 成功
    Success = 200,
    /// 客户端错误
    BadRequest = 400,
    /// 未授权
    Unauthorized = 401,
    /// 禁止访问
    Forbidden = 403,
    /// 资源未找到
    NotFound = 404,
    /// 资源冲突
    Conflict = 409,
    /// 参数验证失败
    ValidationError = 422,
    /// 服务器内部错误
    InternalServerError = 500,

    // ===== 认证相关错误 =====
    /// 认证失败
    AuthenticationFailed = 10001,
    /// 登录失败
    LoginFailed = 10002,
    /// Token 过期
    TokenExpired = 10003,
    /// Token 无效
    TokenInvalid = 10004,
    /// 用户不存在
    UserNotFound = 10005,
    /// 用户已禁用
    UserDisabled = 10006,
    /// 密码错误
    PasswordError = 10007,

    // ===== 权限相关错误 =====
    /// 权限不足
    PermissionDenied = 20001,
    /// 角色不存在
    RoleNotFound = 20002,
    /// 权限不存在
    PermissionNotFound = 20003,

    // ===== 资源相关错误 =====
    /// 资源已存在
    ResourceExists = 30001,
    /// 资源不存在
    ResourceNotFound = 30002,

    // ===== 数据库相关错误 =====
    /// 数据库错误
    DatabaseError = 40001,
    /// 事务错误
    TransactionError = 40002,

    // ===== 业务相关错误 =====
    /// 业务逻辑错误
    BusinessError = 50001,
    /// 操作失败
    OperationFailed = 50002,
    
    // ===== IO和系统错误 =====
    /// IO错误
    IOError = 60001,
    /// 无效输入
    InvalidInput = 60002,
    /// 序列化错误
    SerializationError = 60003,
    
    // ===== 外部服务错误 =====
    /// Redis错误
    RedisError = 70001,
}

impl ErrorCode {
    pub fn code(&self) -> u16 {
        *self as u16
    }

    pub fn message(&self) -> &'static str {
        match self {
            ErrorCode::Success => "操作成功",
            ErrorCode::BadRequest => "请求参数错误",
            ErrorCode::Unauthorized => "未授权访问",
            ErrorCode::Forbidden => "禁止访问",
            ErrorCode::NotFound => "资源未找到",
            ErrorCode::Conflict => "资源冲突",
            ErrorCode::ValidationError => "参数验证失败",
            ErrorCode::InternalServerError => "服务器内部错误",

            ErrorCode::AuthenticationFailed => "认证失败",
            ErrorCode::LoginFailed => "登录失败",
            ErrorCode::TokenExpired => "Token 已过期",
            ErrorCode::TokenInvalid => "Token 无效",
            ErrorCode::UserNotFound => "用户不存在",
            ErrorCode::UserDisabled => "用户已被禁用",
            ErrorCode::PasswordError => "密码错误",

            ErrorCode::PermissionDenied => "权限不足",
            ErrorCode::RoleNotFound => "角色不存在",
            ErrorCode::PermissionNotFound => "权限不存在",

            ErrorCode::ResourceExists => "资源已存在",
            ErrorCode::ResourceNotFound => "资源不存在",

            ErrorCode::DatabaseError => "数据库操作失败",
            ErrorCode::TransactionError => "事务执行失败",

            ErrorCode::BusinessError => "业务逻辑错误",
            ErrorCode::OperationFailed => "操作失败",
            
            ErrorCode::IOError => "IO操作失败",
            ErrorCode::InvalidInput => "无效的输入参数",
            ErrorCode::SerializationError => "序列化失败",
            
            ErrorCode::RedisError => "Redis操作失败",
        }
    }
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message())
    }
}

/// 应用异常
#[derive(Debug)]
pub struct AppError {
    pub code: ErrorCode,
    pub message: String,
    pub details: Option<String>,
}

impl AppError {
    /// 创建新的异常
    pub fn new(code: ErrorCode) -> Self {
        Self {
            code,
            message: code.message().to_string(),
            details: None,
        }
    }

    /// 创建带自定义消息的异常
    pub fn with_message(code: ErrorCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            details: None,
        }
    }

    /// 创建带详细信息的异常
    pub fn with_details(code: ErrorCode, message: impl Into<String>, details: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            details: Some(details.into()),
        }
    }

    /// 获取状态码
    pub fn status_code(&self) -> StatusCode {
        match self.code {
            ErrorCode::Success => StatusCode::OK,
            ErrorCode::BadRequest | ErrorCode::ValidationError => StatusCode::BAD_REQUEST,
            ErrorCode::Unauthorized => StatusCode::UNAUTHORIZED,
            ErrorCode::Forbidden => StatusCode::FORBIDDEN,
            ErrorCode::NotFound => StatusCode::NOT_FOUND,
            ErrorCode::Conflict => StatusCode::CONFLICT,
            ErrorCode::InternalServerError | ErrorCode::DatabaseError | ErrorCode::TransactionError | ErrorCode::RedisError => StatusCode::INTERNAL_SERVER_ERROR,
            
            // IO和系统错误返回 400
            ErrorCode::IOError | ErrorCode::InvalidInput | ErrorCode::SerializationError => StatusCode::BAD_REQUEST,

            // 业务错误返回 400
            ErrorCode::AuthenticationFailed | ErrorCode::LoginFailed | ErrorCode::TokenExpired | ErrorCode::TokenInvalid | ErrorCode::UserNotFound | ErrorCode::UserDisabled | ErrorCode::PasswordError | ErrorCode::PermissionDenied | ErrorCode::RoleNotFound | ErrorCode::PermissionNotFound | ErrorCode::ResourceExists | ErrorCode::ResourceNotFound | ErrorCode::BusinessError | ErrorCode::OperationFailed => StatusCode::BAD_REQUEST,
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(details) = &self.details {
            write!(f, "Error {}: {} - {}", self.code.code(), self.message, details)
        } else {
            write!(f, "Error {}: {}", self.code.code(), self.message)
        }
    }
}

impl std::error::Error for AppError {}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status_code = self.status_code();

        let response: crate::common::response::ResponseModel<()> = crate::common::response::ResponseModel {
            code: self.code.code(),
            msg: self.message,
            data: None,
        };

        (status_code, Json(response)).into_response()
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        Self::with_details(
            ErrorCode::InternalServerError,
            "HTTP 请求失败",
            err.to_string(),
        )
    }
}

impl From<redis::RedisError> for AppError {
    fn from(err: redis::RedisError) -> Self {
        Self::with_details(
            ErrorCode::DatabaseError,
            "Redis 操作失败",
            err.to_string(),
        )
    }
}

impl From<sea_orm::DbErr> for AppError {
    fn from(err: sea_orm::DbErr) -> Self {
        Self::with_details(
            ErrorCode::DatabaseError,
            "数据库操作失败",
            err.to_string(),
        )
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        Self::with_details(
            ErrorCode::InternalServerError,
            "内部错误",
            err.to_string(),
        )
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        Self::with_details(
            ErrorCode::ValidationError,
            "JSON 序列化失败",
            err.to_string(),
        )
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        Self::with_details(
            ErrorCode::TokenInvalid,
            "Token 验证失败",
            err.to_string(),
        )
    }
}

/// 错误处理中间件
pub async fn error_handler(
    request: axum::extract::Request,
    next: axum::middleware::Next,
) -> Response {
    let result = next.run(request).await;

    result
}
