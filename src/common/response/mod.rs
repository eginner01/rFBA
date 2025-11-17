/// API 响应模块
/// 统一的 API 响应格式

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};

/// 响应码
#[derive(Debug, Clone, PartialEq)]
pub enum ResponseCode {
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
    /// 服务器内部错误
    InternalServerError = 500,
}

/// 通用响应结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseModel<T> {
    /// 状态码
    pub code: u16,
    /// 响应消息
    pub msg: String,
    /// 响应数据
    pub data: Option<T>,
}

impl<T> ResponseModel<T> {
    /// 创建成功响应
    pub fn success(data: T) -> Self
    where
        T: Serialize,
    {
        Self {
            code: 200,
            msg: "success".to_string(),
            data: Some(data),
        }
    }

    /// 创建成功响应（无数据）
    pub fn success_msg(msg: impl Into<String>) -> ResponseModel<()> {
        ResponseModel {
            code: 200,
            msg: msg.into(),
            data: None,
        }
    }

    /// 创建错误响应
    pub fn error(code: u16, msg: impl Into<String>) -> ResponseModel<()> {
        ResponseModel {
            code,
            msg: msg.into(),
            data: None,
        }
    }

    /// 创建分页响应
    pub fn paginate(
        data: Vec<T>,
        total: u64,
        page: u64,
        size: u64,
    ) -> ResponseModel<PaginatedData<T>>
    where
        T: Serialize,
    {
        let total_pages = if size == 0 { 0 } else { total.div_ceil(size) };
        let paginated_data = PaginatedData {
            items: data,
            total,
            page,
            size,
            total_pages,
        };

        ResponseModel {
            code: 200,
            msg: "success".to_string(),
            data: Some(paginated_data),
        }
    }
}

/// 分页数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedData<T> {
    /// 数据列表
    pub items: Vec<T>,
    /// 总条数
    pub total: u64,
    /// 当前页
    pub page: u64,
    /// 每页数量
    pub size: u64,
    /// 总页数
    pub total_pages: u64,
}

/// API 响应结果类型
pub type ApiResult<T> = Result<T, crate::common::exception::AppError>;

/// 响应工具
pub mod api_response {
    use super::*;

    /// 成功响应（带数据）
    pub fn success<T>(data: T, msg: impl Into<String>) -> ResponseModel<T>
    where
        T: Serialize,
    {
        ResponseModel {
            code: 200,
            msg: msg.into(),
            data: Some(data),
        }
    }

    /// 成功响应（无数据）
    pub fn success_msg(msg: impl Into<String>) -> ResponseModel<()> {
        ResponseModel {
            code: 200,
            msg: msg.into(),
            data: None,
        }
    }

    /// 错误响应
    pub fn error(code: u16, msg: impl Into<String>) -> ResponseModel<()> {
        ResponseModel {
            code,
            msg: msg.into(),
            data: None,
        }
    }

    /// 分页响应
    pub fn paginate<T>(
        data: Vec<T>,
        total: u64,
        page: u64,
        size: u64,
    ) -> ResponseModel<PaginatedData<T>>
    where
        T: Serialize,
    {
        ResponseModel::paginate(data, total, page, size)
    }
}

/// 便捷函数：直接创建成功响应
/// 这个函数是为了兼容旧代码而保留的，推荐使用 ResponseModel::success()
pub fn api_response<T>(data: T) -> ResponseModel<T>
where
    T: Serialize,
{
    ResponseModel::success(data)
}

impl<T> IntoResponse for ResponseModel<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        let status_code = match self.code {
            200 => StatusCode::OK,
            400 => StatusCode::BAD_REQUEST,
            401 => StatusCode::UNAUTHORIZED,
            403 => StatusCode::FORBIDDEN,
            404 => StatusCode::NOT_FOUND,
            500 => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::OK,
        };

        (status_code, Json(self)).into_response()
    }
}
