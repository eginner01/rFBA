use axum::{http::StatusCode, response::IntoResponse, Json}; use crate::common::response::api_response; pub async fn get_log_levels() -> Result<impl IntoResponse, AppError> { Ok((StatusCode::OK, Json(api_response("日志级别列表")))) }
use crate::common::exception::AppError;
