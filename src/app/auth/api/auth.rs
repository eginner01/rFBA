/// 认证 API - 与Python版本逻辑完全一致
use axum::{
    extract::{State, Json},
    http::StatusCode,
    response::{IntoResponse, Json as AxumJson},
    routing::{get, post},
    Router,
};
use uuid::Uuid;

use crate::app::auth::dto::{LoginRequest, LoginResponse, RefreshTokenRequest, RefreshTokenResponse, LogoutRequest};
use crate::common::exception::{AppError, ErrorCode};
use crate::database::entity::user;
use crate::database::DatabaseConnection;
use crate::utils::encrypt::{CryptoUtils, JwtPayload};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

const ACCESS_TOKEN_EXPIRE_SECONDS: i64 = 3600 * 24; // 24小时 - 与Python版本一致

/// 获取验证码
pub async fn get_captcha() -> Result<impl IntoResponse, AppError> {
    // 调用实际的验证码生成函数
    crate::app::user::api::auth::get_captcha_internal().await
}

/// 登录 - 与Python版本create_access_token逻辑一致
pub async fn login(
    State(db): State<DatabaseConnection>,
    Json(request): Json<LoginRequest>,
) -> Result<(StatusCode, AxumJson<crate::common::response::ResponseModel<LoginResponse>>), AppError> {
    // 查找用户
    let user = user::Entity::find()
        .filter(user::Column::Username.eq(&request.username))
        .filter(user::Column::DelFlag.eq(0))
        .one(&db)
        .await?
        .ok_or(AppError::with_message(
            ErrorCode::UserNotFound,
            "用户名或密码错误",
        ))?;

    // 验证密码
    let password = user.password.as_ref().ok_or_else(|| {
        AppError::with_message(
            ErrorCode::PasswordError,
            "用户密码不存在",
        )
    })?;
    let is_valid = CryptoUtils::verify_password(&request.password, password).await?;
    if !is_valid || user.status != 1 {
        return Err(AppError::with_message(
            ErrorCode::PasswordError,
            "用户名或密码错误",
        ));
    }

    // 生成session UUID - 与Python版本一致
    let session_uuid = Uuid::new_v4().to_string();

    // 生成 access token - 只包含与Python版本相同的字段
    let access_payload = JwtPayload::new(
        user.id.to_string(),      // sub: 用户ID（字符串）
        session_uuid.clone(),     // session_uuid: 会话UUID
        ACCESS_TOKEN_EXPIRE_SECONDS, // exp: 过期时间
    );
    let access_token = CryptoUtils::generate_jwt(&access_payload, &crate::core::SETTINGS.token_secret_key)?;

    // 计算access token过期时间
    let expire_time = chrono::Utc::now() + chrono::Duration::seconds(ACCESS_TOKEN_EXPIRE_SECONDS);
    let access_token_expire_time = expire_time.naive_local();

    // 构建完整的用户信息（匹配Python后端的GetUserInfoDetail）
    let user_info = crate::app::auth::dto::UserInfo {
        id: user.id,
        uuid: user.uuid,
        username: user.username.clone(),
        nickname: user.nickname.clone(),
        email: user.email.clone(),
        phone: user.phone.clone(),
        avatar: user.avatar.clone(),
        dept_id: user.dept_id,
        status: user.status,
        is_superuser: user.is_superuser,
        is_staff: user.is_staff,
        is_multi_login: user.is_multi_login,
        join_time: user.join_time,
        last_login_time: user.last_login_time,
        dept: None, // TODO: 从部门表获取部门名称
        roles: vec![], // TODO: 从角色表获取角色名称列表
    };

    // 存储 token 到 Redis - 与 Python 版本一致
    use crate::core::conf::SETTINGS;
    
    if let Ok(redis) = redis::Client::open(SETTINGS.redis_url()) {
        if let Ok(mut conn) = redis.get_connection() {
            // 1. 存储 access token
            let token_key = format!("{}:{}:{}", SETTINGS.token_redis_prefix, user.id, session_uuid);
            let _: Result<(), _> = redis::cmd("SETEX")
                .arg(&token_key)
                .arg(SETTINGS.token_expire_seconds)
                .arg(&access_token)
                .query(&mut conn);

            // 2. 存储 token 额外信息（用于在线用户列表展示）
            let extra_info = serde_json::json!({
                "username": user.username,
                "nickname": user.nickname,
                "ip": "", // TODO: 从请求中获取IP
                "os": "", // TODO: 从User-Agent解析
                "browser": "", // TODO: 从User-Agent解析
                "device": "", // TODO: 从User-Agent解析
                "last_login_time": chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            });
            
            let extra_info_key = format!("{}:{}:{}", SETTINGS.token_extra_info_redis_prefix, user.id, session_uuid);
            let _: Result<(), _> = redis::cmd("SETEX")
                .arg(&extra_info_key)
                .arg(SETTINGS.token_expire_seconds)
                .arg(serde_json::to_string(&extra_info).unwrap_or_default())
                .query(&mut conn);

            // 3. 添加到在线用户集合（如果需要）
            let _: Result<(), _> = redis::cmd("SADD")
                .arg(&SETTINGS.token_online_redis_prefix)
                .arg(&session_uuid)
                .query(&mut conn);
            
            tracing::info!("Token 已存储到 Redis: {}", token_key);
        }
    }

    let response = LoginResponse {
        access_token,
        access_token_expire_time,
        session_uuid,
        user: user_info,
    };
    
    Ok((StatusCode::OK, AxumJson(crate::common::response::api_response(response))))
}

/// 刷新 Token - 与Python版本create_new_token逻辑一致
pub async fn refresh_token(
    Json(request): Json<RefreshTokenRequest>,
) -> Result<AxumJson<RefreshTokenResponse>, AppError> {
    // 验证refresh token - 与Python版本jwt_decode一致
    let payload = match CryptoUtils::verify_jwt(&request.refresh_token, &crate::core::SETTINGS.token_secret_key) {
        Ok(payload) => payload,
        Err(e) => {
            return Err(AppError::with_message(
                ErrorCode::TokenExpired,
                "Refresh token无效或已过期",
            ));
        }
    };

    // 验证payload包含必要字段 - 与Python版本一致
    if payload.session_uuid.is_empty() || payload.sub.is_empty() {
        return Err(AppError::with_message(
            ErrorCode::TokenInvalid,
            "Refresh token无效",
        ));
    }

    // 重新生成access token - 使用相同的session_uuid（与Python版本一致）
    let new_payload = JwtPayload::new(
        payload.sub.clone(),           // 保持用户ID
        payload.session_uuid.clone(),  // 使用原有session_uuid
        ACCESS_TOKEN_EXPIRE_SECONDS,
    );
    let access_token = CryptoUtils::generate_jwt(&new_payload, &crate::core::SETTINGS.token_secret_key)?;

    // 计算access token过期时间
    let expire_time = chrono::Utc::now() + chrono::Duration::seconds(ACCESS_TOKEN_EXPIRE_SECONDS);
    let access_token_expire_time = expire_time.naive_local();

    Ok(AxumJson(RefreshTokenResponse {
        access_token,
        access_token_expire_time,
        session_uuid: payload.session_uuid.clone(),
    }))
}

/// 退出登录
pub async fn logout(
    Json(_request): Json<LogoutRequest>,
) -> Result<AxumJson<serde_json::Value>, AppError> {
    // TODO: 实现Token黑名单机制 - 可选功能

    Ok(AxumJson(serde_json::json!({
        "code": 200,
        "message": "退出成功",
        "data": null
    })))
}

/// 获取当前用户信息
pub async fn get_current_user(
    State(db): State<DatabaseConnection>,
    request: axum::http::Request<axum::body::Body>,
) -> Result<AxumJson<user::Model>, AppError> {
    // 从请求扩展中获取认证上下文
    let auth_context = request
        .extensions()
        .get::<crate::middleware::jwt_auth_middleware::AuthContext>()
        .ok_or(AppError::with_message(
            ErrorCode::Unauthorized,
            "未认证的用户",
        ))?;

    // 解析用户ID
    let user_id = auth_context.user_id.parse::<i64>()
        .map_err(|_| AppError::with_message(
            ErrorCode::TokenInvalid,
            "无效的用户ID",
        ))?;

    // 查找用户
    let user = user::Entity::find_by_id(user_id)
        .filter(user::Column::DelFlag.eq(0))
        .one(&db)
        .await?
        .ok_or(AppError::with_message(
            ErrorCode::UserNotFound,
            "用户不存在",
        ))?;

    Ok(AxumJson(user))
}

/// 认证路由
pub fn auth_router() -> Router<DatabaseConnection> {
    Router::new()
        .route("/captcha", get(get_captcha))
        .route("/login", post(login))
        .route("/refresh", post(refresh_token))
        .route("/logout", post(logout))
        .route("/me", get(get_current_user))
}
