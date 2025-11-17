use axum::{http::StatusCode, response::IntoResponse, Json}; use crate::common::response::api_response; pub async fn get_system_metrics() -> Result<impl IntoResponse, AppError> { Ok((StatusCode::OK, Json(api_response("指标列表")))) }
use crate::common::exception::AppError;
