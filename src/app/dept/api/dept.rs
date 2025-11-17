/// 部门管理 API 处理器

use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use crate::common::response::api_response;
use crate::common::exception::AppError;
use crate::database::DatabaseManager;
use crate::app::dept::dto::{
    CreateDeptRequest, UpdateDeptRequest,
    DeptTreeQuery, DeptListQuery,
};
use crate::app::dept::service::DeptService;

/// 状态更新请求 DTO
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusChangeRequest {
    pub status: i32,
}

/// 获取部门树
pub async fn get_dept_tree(
    Query(query): Query<DeptTreeQuery>,
    _auth_context: axum::extract::Extension<crate::middleware::jwt_auth_middleware::AuthContext>,
) -> Result<impl IntoResponse, AppError> {
    let db_conn = DatabaseManager::get_connection().await.clone();
    let service = DeptService::new(db_conn);

    let result = service.get_dept_tree(&query).await?;

    Ok((StatusCode::OK, Json(api_response(result))))
}

/// 获取部门列表（扁平列表，支持筛选）
pub async fn get_dept_list(
    Query(query): Query<DeptListQuery>,
    _auth_context: axum::extract::Extension<crate::middleware::jwt_auth_middleware::AuthContext>,
) -> Result<impl IntoResponse, AppError> {
    let db_conn = DatabaseManager::get_connection().await.clone();
    let service = DeptService::new(db_conn);

    let result = service.get_dept_list_with_filter(&query).await?;

    Ok((StatusCode::OK, Json(api_response(result))))
}

/// 获取部门详情
pub async fn get_dept(
    _auth_context: axum::extract::Extension<crate::middleware::jwt_auth_middleware::AuthContext>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let db_conn = DatabaseManager::get_connection().await.clone();
    let service = DeptService::new(db_conn);

    let result = service.get_dept_detail(id).await?;

    Ok((StatusCode::OK, Json(api_response(result))))
}

/// 创建部门
pub async fn create_dept(
    _auth_context: axum::extract::Extension<crate::middleware::jwt_auth_middleware::AuthContext>,
    Json(request): Json<CreateDeptRequest>,
) -> Result<impl IntoResponse, AppError> {
    let db_conn = DatabaseManager::get_connection().await.clone();
    let service = DeptService::new(db_conn);

    // TODO: 从当前登录用户获取创建人信息
    let create_by = "system";

    let result = service.create_dept(&request, create_by).await?;

    Ok((StatusCode::CREATED, Json(api_response(result))))
}

/// 更新部门
pub async fn update_dept(
    _auth_context: axum::extract::Extension<crate::middleware::jwt_auth_middleware::AuthContext>,
    Path(id): Path<i64>,
    raw_body: axum::body::Bytes,
) -> Result<impl IntoResponse, AppError> {
    use tracing::info;
    
    // 打印原始请求体用于调试
    let body_str = String::from_utf8_lossy(&raw_body);
    info!("收到更新部门请求，ID={}, 请求体: {}", id, body_str);
    
    // 手动反序列化以捕获更详细的错误
    let request: UpdateDeptRequest = serde_json::from_slice(&raw_body)
        .map_err(|e| {
            tracing::error!("JSON 反序列化失败: {:?}", e);
            AppError::with_message(
                crate::common::exception::ErrorCode::BadRequest,
                format!("Invalid request body: {}", e)
            )
        })?;
    
    info!("反序列化成功: name={}, parent_id={:?}, sort={}, status={}", 
          request.name, request.parent_id, request.sort, request.status);
    
    let db_conn = DatabaseManager::get_connection().await.clone();
    let service = DeptService::new(db_conn);

    // TODO: 从当前登录用户获取更新人信息
    let update_by = Some("system");

    let result = service.update_dept(id, &request, update_by).await?;

    Ok((StatusCode::OK, Json(api_response(result))))
}

/// 删除部门
pub async fn delete_dept(
    _auth_context: axum::extract::Extension<crate::middleware::jwt_auth_middleware::AuthContext>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let db_conn = DatabaseManager::get_connection().await.clone();
    let service = DeptService::new(db_conn);

    service.delete_dept(id).await?;

    Ok((StatusCode::NO_CONTENT, Json(api_response("部门删除成功".to_string()))))
}

/// 更改部门状态
pub async fn change_dept_status(
    _auth_context: axum::extract::Extension<crate::middleware::jwt_auth_middleware::AuthContext>,
    Path(id): Path<i64>,
    Json(request): Json<StatusChangeRequest>,
) -> Result<impl IntoResponse, AppError> {
    let db_conn = DatabaseManager::get_connection().await.clone();
    let service = DeptService::new(db_conn);

    service.change_status(id, request.status).await?;

    Ok((StatusCode::OK, Json(api_response("部门状态更新成功".to_string()))))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_change_request() {
        let request = StatusChangeRequest { status: 1 };
        assert_eq!(request.status, 1);
    }
}
