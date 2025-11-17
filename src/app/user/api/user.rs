/// 用户管理相关API
/// 提供用户CRUD、分页查询、密码管理等接口

use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use crate::common::response::{api_response, ApiResult};
use crate::app::user::dto::{
    CreateUserRequest, UpdateUserRequest, UserPaginationQuery,
    ChangePasswordRequest, ResetPasswordRequest,
    ImportUsersRequest, ExportUsersRequest, DownloadTemplateRequest, BatchImportUsersRequest,
};
use crate::app::user::service::UserService;
use crate::database::DatabaseManager;

/// 获取用户列表（分页）
/// GET /api/v1/users
pub async fn get_users(
    Query(query): Query<UserPaginationQuery>,
) -> ApiResult<impl IntoResponse> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建用户服务
    let user_service = UserService::new(db_conn.clone());

    // 查询用户列表
    let result = user_service.get_users_paginated(&query).await?;

    Ok((StatusCode::OK, Json(api_response(result))))
}

/// 获取用户详情
/// GET /api/v1/users/{id}
pub async fn get_user(
    Path(id): Path<i64>,
) -> ApiResult<impl IntoResponse> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建用户服务
    let user_service = UserService::new(db_conn.clone());

    // 查询用户详情
    let result = user_service.get_user_detail(id).await?;

    Ok((StatusCode::OK, Json(api_response(result))))
}

/// 创建用户
/// POST /api/v1/users
pub async fn create_user(
    Json(request): Json<CreateUserRequest>,
) -> ApiResult<impl IntoResponse> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建用户服务
    let user_service = UserService::new(db_conn.clone());

    // 创建用户
    let result = user_service.create_user(&request).await?;

    Ok((StatusCode::CREATED, Json(api_response(result))))
}

/// 更新用户
/// PUT /api/v1/users/{id}
pub async fn update_user(
    Path(id): Path<i64>,
    Json(request): Json<UpdateUserRequest>,
) -> ApiResult<impl IntoResponse> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建用户服务
    let user_service = UserService::new(db_conn.clone());

    // 更新用户
    user_service.update_user(id, &request).await?;

    Ok((StatusCode::OK, Json(api_response("用户更新成功".to_string()))))
}

/// 删除用户
/// DELETE /api/v1/users/{id}
pub async fn delete_user(
    Path(id): Path<i64>,
) -> ApiResult<impl IntoResponse> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建用户服务
    let user_service = UserService::new(db_conn.clone());

    // 删除用户
    user_service.delete_user(id).await?;

    Ok((StatusCode::OK, Json(api_response("用户删除成功".to_string()))))
}

/// 批量删除用户
/// DELETE /api/v1/users/batch
pub async fn batch_delete_users(
    Json(user_ids): Json<Vec<i64>>,
) -> ApiResult<impl IntoResponse> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建用户服务
    let user_service = UserService::new(db_conn.clone());

    // 批量删除用户
    user_service.batch_delete_users(&user_ids).await?;

    Ok((StatusCode::OK, Json(api_response("批量删除成功".to_string()))))
}

/// 修改密码
/// POST /api/v1/users/change-password
pub async fn change_password(
    Json(request): Json<ChangePasswordRequest>,
) -> ApiResult<impl IntoResponse> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建用户服务
    let user_service = UserService::new(db_conn.clone());

    // 修改密码
    user_service.change_password(&request).await?;

    Ok((StatusCode::OK, Json(api_response("密码修改成功".to_string()))))
}

/// 重置密码
/// POST /api/v1/users/reset-password
pub async fn reset_password(
    Json(request): Json<ResetPasswordRequest>,
) -> ApiResult<impl IntoResponse> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建用户服务
    let user_service = UserService::new(db_conn.clone());

    // 重置密码
    user_service.reset_password(&request).await?;

    Ok((StatusCode::OK, Json(api_response("密码重置成功".to_string()))))
}

/// 更新用户状态
/// PATCH /api/v1/users/{id}/status
pub async fn update_user_status(
    Path(id): Path<i64>,
    Json(status): Json<i32>,
) -> ApiResult<impl IntoResponse> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建用户服务
    let user_service = UserService::new(db_conn.clone());

    // 更新用户状态
    user_service.update_user_status(id, status).await?;

    Ok((StatusCode::OK, Json(api_response("用户状态更新成功".to_string()))))
}

/// 导入用户
/// POST /api/v1/users/import
pub async fn import_users(
    Json(request): Json<ImportUsersRequest>,
) -> ApiResult<impl IntoResponse> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建用户服务
    let user_service = UserService::new(db_conn.clone());

    // 导入用户
    let result = user_service.import_users(&request).await?;

    Ok((StatusCode::OK, Json(api_response(result))))
}

/// 导出用户
/// POST /api/v1/users/export
pub async fn export_users(
    Json(request): Json<ExportUsersRequest>,
) -> ApiResult<impl IntoResponse> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建用户服务
    let user_service = UserService::new(db_conn.clone());

    // 导出用户
    let result = user_service.export_users(&request).await?;

    Ok((StatusCode::OK, Json(api_response(result))))
}

/// 下载用户导入模板
/// POST /api/v1/users/download-template
pub async fn download_template(
    Json(request): Json<DownloadTemplateRequest>,
) -> ApiResult<impl IntoResponse> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建用户服务
    let user_service = UserService::new(db_conn.clone());

    // 下载模板
    let result = user_service.download_template(&request).await?;

    Ok((StatusCode::OK, Json(api_response(result))))
}

/// 批量导入用户
/// POST /api/v1/users/batch-import
pub async fn batch_import_users(
    Json(request): Json<BatchImportUsersRequest>,
) -> ApiResult<impl IntoResponse> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建用户服务
    let user_service = UserService::new(db_conn.clone());

    // 批量导入用户
    let result = user_service.batch_import_users(&request).await?;

    Ok((StatusCode::OK, Json(api_response(result))))
}
