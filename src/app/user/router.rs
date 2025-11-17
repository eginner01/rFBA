/// 用户管理路由
/// 组装所有用户相关的API路由

use axum::{routing::{get, post, put, delete, patch}, Router};
use axum::response::IntoResponse;
use axum::extract::Query;
use sea_orm::{EntityTrait, ColumnTrait, QueryFilter};
use crate::database::DatabaseManager;
use crate::app::user::dto::{
    UserPaginationQuery, CreateUserRequest, UpdateUserRequest,
    ChangePasswordRequest, ResetPasswordRequest,
    ImportUsersRequest, ExportUsersRequest, DownloadTemplateRequest, BatchImportUsersRequest,
};
use crate::app::auth::dto::{
    LoginRequest, RefreshTokenRequest, LogoutRequest,
};
use axum::{extract::Path, Json, http::StatusCode};
use crate::common::exception::AppError;
use crate::common::response::api_response;

/// 用户管理路由 - 路径规范: /api/v1/sys/users/*
pub fn user_routes() -> Router {
    Router::new()
        // ===== 当前用户相关路由 =====
        .route("/me", get(get_current_user_handler))  // GET /api/v1/sys/users/me
        .route("/me/password", put(update_current_user_password_handler))  // PUT /api/v1/sys/users/me/password
        .route("/me/nickname", put(update_current_user_nickname_handler))  // PUT /api/v1/sys/users/me/nickname
        .route("/me/phone", put(update_current_user_phone_handler))  // PUT /api/v1/sys/users/me/phone
        .route("/me/avatar", put(update_current_user_avatar_handler))  // PUT /api/v1/sys/users/me/avatar
        .route("/me/email", put(update_current_user_email_handler))  // PUT /api/v1/sys/users/me/email

        // ===== 用户管理路由 =====
        .route("/", get(get_users_handler))  // GET /api/v1/sys/users
        .route("/", post(create_user_handler))  // POST /api/v1/sys/users
        .route("/batch", delete(batch_delete_users_handler))  // DELETE /api/v1/sys/users/batch

        .route("/{id}", get(get_user_handler))  // GET /api/v1/sys/users/{id}
        .route("/{id}", put(update_user_handler))  // PUT /api/v1/sys/users/{id}
        .route("/{id}", delete(delete_user_handler))  // DELETE /api/v1/sys/users/{id}
        .route("/{id}/status", patch(update_user_status_handler))  // PATCH /api/v1/sys/users/{id}/status
        .route("/{id}/password", put(reset_user_password_handler))  // PUT /api/v1/sys/users/{id}/password
        .route("/{id}/roles", get(get_user_roles_handler))  // GET /api/v1/sys/users/{id}/roles
        .route("/{id}/permissions", put(update_user_permissions_handler))  // PUT /api/v1/sys/users/{id}/permissions

        // ===== 导入导出路由 =====
        .route("/import", post(import_users_handler))  // POST /api/v1/sys/users/import
        .route("/export", post(export_users_handler))  // POST /api/v1/sys/users/export
        .route("/download-template", post(download_template_handler))  // POST /api/v1/sys/users/download-template
        .route("/batch-import", post(batch_import_users_handler))  // POST /api/v1/sys/users/batch-import
}

/// 获取验证码
/// GET /api/v1/auth/captcha
async fn get_captcha_handler() -> Result<impl IntoResponse, AppError> {
    // 直接调用API函数，内部处理Redis连接
    crate::app::user::api::auth::get_captcha_internal().await
}

/// 获取当前用户信息
/// GET /api/v1/users/me
async fn get_current_user_handler(
    // 从请求扩展中提取认证上下文
    auth_context: axum::extract::Extension<crate::middleware::jwt_auth_middleware::AuthContext>,
) -> Result<impl IntoResponse, AppError> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 从认证上下文中获取用户ID
    let auth_context = auth_context.0;
    let user_id: i64 = auth_context.user_id.parse()
        .map_err(|_| AppError::new(crate::common::exception::ErrorCode::ValidationError))?;

    // 使用用户服务获取当前用户详情（包含部门名称和角色名称列表）
    let user_service = crate::app::user::service::UserService::new(db_conn.clone());
    let current_user = user_service.get_current_user(user_id).await?;

    Ok((StatusCode::OK, Json(api_response(current_user))))
}

/// 获取用户权限码
/// GET /api/v1/auth/codes
async fn get_codes_handler(
    // 从请求扩展中提取认证上下文
    _auth_context: axum::extract::Extension<crate::middleware::jwt_auth_middleware::AuthContext>,
) -> Result<impl IntoResponse, AppError> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 从认证上下文中获取用户ID
    let auth_context = _auth_context.0;
    let user_id: i64 = auth_context.user_id.parse()
        .map_err(|_| AppError::new(crate::common::exception::ErrorCode::ValidationError))?;

    // 查询用户信息，检查是否为超级用户
    let user_model = crate::database::user::Entity::find_by_id(user_id)
        .one(db_conn)
        .await?
        .ok_or_else(|| AppError::new(crate::common::exception::ErrorCode::NotFound))?;

    // 用于存储权限码的集合
    let mut permission_codes = std::collections::HashSet::<String>::new();

    if user_model.is_superuser {
        // 超级用户：获取所有菜单的权限码
        let menus = crate::database::menu::Entity::find()
            .all(db_conn)
            .await?;

        for menu in menus {
            if let Some(perms) = menu.perms {
                // 权限码以逗号分隔
                for perm in perms.split(',').filter(|s| !s.is_empty()) {
                    permission_codes.insert(perm.trim().to_string());
                }
            }
        }
    } else {
        // 普通用户：获取用户角色下所有菜单的权限码
        // 1. 查询用户角色关联表
        let user_roles = crate::database::user_role::Entity::find()
            .filter(crate::database::user_role::Column::UserId.eq(user_id))
            .all(db_conn)
            .await?;

        if !user_roles.is_empty() {
            // 2. 查询所有有效的菜单（实际项目中应该根据角色权限过滤）
            let menus = crate::database::menu::Entity::find()
                .all(db_conn)
                .await?;

            // 简化处理：返回所有菜单的权限码（实际项目中应该根据角色权限过滤）
            for menu in menus {
                if let Some(perms) = menu.perms {
                    for perm in perms.split(',').filter(|s| !s.is_empty()) {
                        permission_codes.insert(perm.trim().to_string());
                    }
                }
            }
        }
    }

    // 转换为Vec<String>返回
    let codes: Vec<String> = permission_codes.into_iter().collect();

    Ok((StatusCode::OK, Json(api_response(codes))))
}

// ===== 认证相关处理器 =====
async fn login_handler(
    Json(request): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建认证服务
    let auth_service = crate::app::auth::service::AuthService::new(
        crate::core::SETTINGS.token_secret_key.clone()
    );

    // 调用认证服务进行登录
    let result = auth_service.login(&request, db_conn).await?;

    Ok((StatusCode::OK, Json(api_response(result))))
}

async fn refresh_token_handler(
    Json(request): Json<RefreshTokenRequest>,
) -> Result<impl IntoResponse, AppError> {
    // 创建认证服务
    let auth_service = crate::app::auth::service::AuthService::new(
        crate::core::SETTINGS.token_secret_key.clone()
    );

    // 刷新token（同步方法，不需要await）
    let result = auth_service.refresh_token(&request)?;

    Ok((StatusCode::OK, Json(api_response(result))))
}

async fn logout_handler(
    Json(_request): Json<LogoutRequest>,
) -> Result<impl IntoResponse, AppError> {
    // 对于JWT token，客户端删除token即可实现登出
    // 服务器端可以通过token的过期时间来自动失效
    // 这里可以添加清除Redis缓存的逻辑（如在线用户列表）
    // 暂时简化实现

    Ok((StatusCode::OK, Json(api_response("登出成功".to_string()))))
}

async fn register_handler(
    Json(_request): Json<CreateUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建用户服务（暂未使用）
    let _user_service = crate::app::user::service::UserService::new(db_conn.clone());

    // 注册用户（通常需要邮箱验证等，暂时返回未实现）
    // let result = user_service.register_user(&request).await?;
    // Ok((StatusCode::CREATED, Json(api_response(result))))

    Ok((StatusCode::NOT_IMPLEMENTED, Json(api_response("用户注册功能未开放".to_string()))))
}

// ===== 用户管理处理器 =====

/// 获取用户列表（分页）
/// GET /api/v1/users
async fn get_users_handler(
    Query(query): Query<UserPaginationQuery>,
) -> Result<impl IntoResponse, AppError> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建用户服务
    let user_service = crate::app::user::service::UserService::new(db_conn.clone());

    // 查询用户列表
    let result = user_service.get_users_paginated(&query).await?;

    Ok((StatusCode::OK, Json(api_response(result))))
}

/// 获取用户详情
/// GET /api/v1/users/{id}
async fn get_user_handler(
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建用户服务
    let user_service = crate::app::user::service::UserService::new(db_conn.clone());

    // 查询用户详情
    let result = user_service.get_user_detail(id).await?;

    Ok((StatusCode::OK, Json(api_response(result))))
}

/// 创建用户
/// POST /api/v1/users
async fn create_user_handler(
    Json(request): Json<CreateUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建用户服务
    let user_service = crate::app::user::service::UserService::new(db_conn.clone());

    // 创建用户
    let result = user_service.create_user(&request).await?;

    Ok((StatusCode::CREATED, Json(api_response(result))))
}

/// 更新用户
/// PUT /api/v1/users/{id}
async fn update_user_handler(
    Path(id): Path<i64>,
    Json(request): Json<UpdateUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建用户服务
    let user_service = crate::app::user::service::UserService::new(db_conn.clone());

    // 更新用户
    user_service.update_user(id, &request).await?;

    Ok((StatusCode::OK, Json(api_response("用户更新成功".to_string()))))
}

/// 删除用户
/// DELETE /api/v1/users/{id}
async fn delete_user_handler(
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建用户服务
    let user_service = crate::app::user::service::UserService::new(db_conn.clone());

    // 删除用户
    user_service.delete_user(id).await?;

    Ok((StatusCode::OK, Json(api_response("用户删除成功".to_string()))))
}

/// 批量删除用户
/// DELETE /api/v1/users/batch
async fn batch_delete_users_handler(
    Json(user_ids): Json<Vec<i64>>,
) -> Result<impl IntoResponse, AppError> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建用户服务
    let user_service = crate::app::user::service::UserService::new(db_conn.clone());

    // 批量删除用户
    user_service.batch_delete_users(&user_ids).await?;

    Ok((StatusCode::OK, Json(api_response("批量删除成功".to_string()))))
}

/// 修改密码
/// POST /api/v1/users/change-password
async fn change_password_handler(
    Json(request): Json<ChangePasswordRequest>,
) -> Result<impl IntoResponse, AppError> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建用户服务
    let user_service = crate::app::user::service::UserService::new(db_conn.clone());

    // 修改密码
    user_service.change_password(&request).await?;

    Ok((StatusCode::OK, Json(api_response("密码修改成功".to_string()))))
}

/// 重置密码
/// POST /api/v1/users/reset-password
async fn reset_password_handler(
    Json(request): Json<ResetPasswordRequest>,
) -> Result<impl IntoResponse, AppError> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建用户服务
    let user_service = crate::app::user::service::UserService::new(db_conn.clone());

    // 重置密码
    user_service.reset_password(&request).await?;

    Ok((StatusCode::OK, Json(api_response("密码重置成功".to_string()))))
}

/// 更新用户状态
/// PATCH /api/v1/users/{id}/status
async fn update_user_status_handler(
    Path(id): Path<i64>,
    Json(status): Json<i32>,
) -> Result<impl IntoResponse, AppError> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建用户服务
    let user_service = crate::app::user::service::UserService::new(db_conn.clone());

    // 更新用户状态
    user_service.update_user_status(id, status).await?;

    Ok((StatusCode::OK, Json(api_response("用户状态更新成功".to_string()))))
}

/// 导入用户
/// POST /api/v1/users/import
async fn import_users_handler(
    Json(request): Json<ImportUsersRequest>,
) -> Result<impl IntoResponse, AppError> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建用户服务
    let user_service = crate::app::user::service::UserService::new(db_conn.clone());

    // 导入用户
    let result = user_service.import_users(&request).await?;

    Ok((StatusCode::OK, Json(api_response(result))))
}

/// 导出用户
/// POST /api/v1/users/export
async fn export_users_handler(
    Json(request): Json<ExportUsersRequest>,
) -> Result<impl IntoResponse, AppError> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建用户服务
    let user_service = crate::app::user::service::UserService::new(db_conn.clone());

    // 导出用户
    let result = user_service.export_users(&request).await?;

    Ok((StatusCode::OK, Json(api_response(result))))
}

/// 下载用户导入模板
/// POST /api/v1/users/download-template
async fn download_template_handler(
    Json(request): Json<DownloadTemplateRequest>,
) -> Result<impl IntoResponse, AppError> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建用户服务
    let user_service = crate::app::user::service::UserService::new(db_conn.clone());

    // 下载模板
    let result = user_service.download_template(&request).await?;

    Ok((StatusCode::OK, Json(api_response(result))))
}

/// 批量导入用户
/// POST /api/v1/users/batch-import
async fn batch_import_users_handler(
    Json(request): Json<BatchImportUsersRequest>,
) -> Result<impl IntoResponse, AppError> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建用户服务
    let user_service = crate::app::user::service::UserService::new(db_conn.clone());

    // 批量导入用户
    let result = user_service.batch_import_users(&request).await?;

    Ok((StatusCode::OK, Json(api_response(result))))
}

// ===== Handler函数 =====

/// 更新当前用户密码
/// PUT /api/v1/sys/users/me/password
async fn update_current_user_password_handler(
    _auth_context: axum::extract::Extension<crate::middleware::jwt_auth_middleware::AuthContext>,
    Json(request): Json<ChangePasswordRequest>,
) -> Result<impl IntoResponse, AppError> {
    let db_conn = DatabaseManager::get_connection().await;
    let user_service = crate::app::user::service::UserService::new(db_conn.clone());

    // TODO: 从认证上下文中获取当前用户ID
    user_service.change_password(&request).await?;

    Ok((StatusCode::OK, Json(api_response("密码修改成功".to_string()))))
}

/// 更新当前用户昵称
/// PUT /api/v1/sys/users/me/nickname
async fn update_current_user_nickname_handler(
    _auth_context: axum::extract::Extension<crate::middleware::jwt_auth_middleware::AuthContext>,
    Json(_request): Json<serde_json::Value>,
) -> Result<impl IntoResponse, AppError> {
    // TODO: 从认证上下文中获取当前用户ID
    // TODO: 实现昵称更新逻辑
    Ok((StatusCode::OK, Json(api_response("昵称更新成功".to_string()))))
}

/// 更新当前用户头像
/// PUT /api/v1/sys/users/me/avatar
async fn update_current_user_avatar_handler(
    _auth_context: axum::extract::Extension<crate::middleware::jwt_auth_middleware::AuthContext>,
    Json(_request): Json<serde_json::Value>,
) -> Result<impl IntoResponse, AppError> {
    // TODO: 从认证上下文中获取当前用户ID
    // TODO: 实现头像更新逻辑
    Ok((StatusCode::OK, Json(api_response("头像更新成功".to_string()))))
}

/// 更新当前用户邮箱
/// PUT /api/v1/sys/users/me/email
async fn update_current_user_email_handler(
    _auth_context: axum::extract::Extension<crate::middleware::jwt_auth_middleware::AuthContext>,
    Json(_request): Json<serde_json::Value>,
) -> Result<impl IntoResponse, AppError> {
    // TODO: 从认证上下文中获取当前用户ID
    // TODO: 实现邮箱更新逻辑
    Ok((StatusCode::OK, Json(api_response("邮箱更新成功".to_string()))))
}

/// 更新当前用户手机号
/// PUT /api/v1/sys/users/me/phone
async fn update_current_user_phone_handler(
    _auth_context: axum::extract::Extension<crate::middleware::jwt_auth_middleware::AuthContext>,
    Json(_request): Json<serde_json::Value>,
) -> Result<impl IntoResponse, AppError> {
    // TODO: 从认证上下文中获取当前用户ID
    // TODO: 实现手机号更新逻辑
    Ok((StatusCode::OK, Json(api_response("手机号更新成功".to_string()))))
}

/// 重置用户密码（管理员操作）
/// PUT /api/v1/sys/users/{id}/password
async fn reset_user_password_handler(
    Path(id): Path<i64>,
    Json(request): Json<serde_json::Value>,
) -> Result<impl IntoResponse, AppError> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建用户服务
    let user_service = crate::app::user::service::UserService::new(db_conn.clone());

    // 从请求中提取密码
    let password = request.get("password")
        .and_then(|v| v.as_str())
        .ok_or_else(|| AppError::new(crate::common::exception::ErrorCode::ValidationError))?;

    // 重置密码
    user_service.admin_reset_password(id, password).await?;

    Ok((StatusCode::OK, Json(api_response("密码重置成功".to_string()))))
}

/// 获取用户角色列表
/// GET /api/v1/sys/users/{id}/roles
async fn get_user_roles_handler(
    Path(_id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    // TODO: 实现获取用户角色逻辑
    let roles: Vec<String> = vec![];
    Ok((StatusCode::OK, Json(api_response(roles))))
}

/// 更新用户权限（切换状态）
/// PUT /api/v1/sys/users/{id}/permissions?type=status
async fn update_user_permissions_handler(
    Path(id): Path<i64>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<impl IntoResponse, AppError> {
    use tracing::info;
    
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建用户服务
    let user_service = crate::app::user::service::UserService::new(db_conn.clone());

    // 从查询参数中提取权限类型
    let permission_type_str = params.get("type")
        .ok_or_else(|| {
            tracing::error!("缺少必需的查询参数: type");
            AppError::with_message(
                crate::common::exception::ErrorCode::ValidationError,
                "Missing required parameter: type"
            )
        })?;

    info!("更新用户权限请求: user_id={}, type={}", id, permission_type_str);

    let permission_type = serde_json::from_str::<crate::common::enums::UserPermissionType>(&format!("\"{}\"", permission_type_str))
        .map_err(|e| {
            tracing::error!("无效的权限类型: {}, 错误: {:?}", permission_type_str, e);
            AppError::with_message(
                crate::common::exception::ErrorCode::ValidationError,
                format!("Invalid permission type: {}", permission_type_str)
            )
        })?;

    // 更新权限（自动切换状态）
    user_service.update_permission_toggle(id, permission_type).await?;

    Ok((StatusCode::OK, Json(api_response("权限更新成功".to_string()))))
}
