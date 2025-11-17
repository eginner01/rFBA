use axum::{
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use crate::common::response::api_response;
use crate::common::exception::AppError;
use serde::Serialize;

#[derive(Serialize)]
pub struct SysConfigItem {
    id: i64,
    config_name: String,
    config_key: String,
    config_value: String,
    config_type: String,
}

pub async fn get_sys_configs() -> Result<impl IntoResponse, AppError> {
    let configs = vec![SysConfigItem {
        id: 1,
        config_name: "系统名称".to_string(),
        config_key: "sys.name".to_string(),
        config_value: "FastAPI架构".to_string(),
        config_type: "system".to_string(),
    }];
    Ok((StatusCode::OK, Json(api_response(configs))))
}

pub async fn get_sys_config() -> Result<impl IntoResponse, AppError> {
    Ok((StatusCode::OK, Json(api_response("查询成功".to_string()))))
}

pub async fn create_sys_config() -> Result<impl IntoResponse, AppError> {
    Ok((StatusCode::CREATED, Json(api_response("创建成功".to_string()))))
}

pub async fn update_sys_config() -> Result<impl IntoResponse, AppError> {
    Ok((StatusCode::OK, Json(api_response("更新成功".to_string()))))
}

pub async fn delete_sys_config() -> Result<impl IntoResponse, AppError> {
    Ok((StatusCode::NO_CONTENT, Json(api_response("删除成功".to_string()))))
}
