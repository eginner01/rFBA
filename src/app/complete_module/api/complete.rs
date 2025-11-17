/// 完整性检查 API
/// 提供系统完整性检查、功能模块清单、健康检查等接口

use axum::{
    extract::Query,
    response::Json,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::app::complete_module::service::CompleteService;
use crate::app::complete_module::dto::{
    SystemStatusResponse, ModuleInfoResponse, HealthCheckResponse,
};
use crate::common::response::ResponseModel;
use crate::common::exception::AppError;
use crate::database::DatabaseManager;

/// 系统状态查询参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStatusQuery {
    /// 是否包含详细信息
    pub include_details: Option<bool>,
}

/// 获取系统状态
pub async fn get_system_status(
    Query(_query): Query<SystemStatusQuery>,
) -> Result<Json<ResponseModel<SystemStatusResponse>>, AppError> {
    let db_conn = DatabaseManager::get_connection().await;
    let service = CompleteService::new(db_conn.clone());

    info!("Fetching system status...");

    let status = service.get_system_status().await?;

    Ok(Json(ResponseModel::success(status)))
}

/// 获取模块信息
pub async fn get_module_info(
) -> Result<Json<ResponseModel<ModuleInfoResponse>>, AppError> {
    let db_conn = DatabaseManager::get_connection().await;
    let service = CompleteService::new(db_conn.clone());

    info!("Fetching module information...");

    let info = service.get_module_info().await?;

    Ok(Json(ResponseModel::success(info)))
}

/// 健康检查
pub async fn health_check(
) -> Result<(StatusCode, Json<ResponseModel<HealthCheckResponse>>), AppError> {
    let db_conn = DatabaseManager::get_connection().await;
    let service = CompleteService::new(db_conn.clone());

    info!("Performing health check...");

    let health = service.health_check().await?;

    let status_code = if health.overall_status == "healthy" {
        axum::http::StatusCode::OK
    } else {
        axum::http::StatusCode::SERVICE_UNAVAILABLE
    };

    Ok((status_code, Json(ResponseModel::success(health))))
}

/// 获取系统摘要
pub async fn get_system_summary(
) -> Result<Json<ResponseModel<SystemSummary>>, AppError> {
    let db_conn = DatabaseManager::get_connection().await;
    let service = CompleteService::new(db_conn.clone());

    // 获取系统状态
    let status = service.get_system_status().await?;

    // 获取模块信息
    let module_info = service.get_module_info().await?;

    let summary = SystemSummary {
        system_name: "FastAPI Best Architecture - Rust".to_string(),
        system_version: env!("CARGO_PKG_VERSION").to_string(),
        status: status.status,
        total_modules: module_info.total_modules,
        implemented_modules: module_info.implemented_modules,
        health_percentage: status.health.health_percentage,
        database_status: if status.health.database_connection { "connected".to_string() } else { "disconnected".to_string() },
    };

    Ok(Json(ResponseModel::success(summary)))
}

/// 系统摘要 DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemSummary {
    pub system_name: String,
    pub system_version: String,
    pub status: String,
    pub total_modules: u32,
    pub implemented_modules: u32,
    pub health_percentage: u8,
    pub database_status: String,
}
