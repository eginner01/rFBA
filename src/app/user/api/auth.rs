/// 用户认证相关API
/// 提供登录、注册、Token刷新、登出等接口

use axum::{
    extract::Json,
    http::StatusCode,
    response::IntoResponse,
};
use crate::common::response::api_response;
use crate::app::auth::dto::{LoginRequest, RefreshTokenRequest, LogoutRequest, CaptchaResponse};
use crate::app::auth::service::AuthService;
use crate::app::user::dto::CreateUserRequest;
use crate::common::exception::AppError;
use crate::core::SETTINGS;
use crate::database::DatabaseManager;
use captcha::{Captcha, filters::{Noise, Wave}};
use uuid::Uuid;
use base64::{Engine as _, engine::general_purpose};

/// 获取登录验证码（内部版本，用于无State的情况）
pub async fn get_captcha_internal() -> Result<impl IntoResponse, AppError> {
    // 生成UUID作为验证码标识
    let uuid = Uuid::new_v4().to_string();

    // 生成清新现代风格的验证码
    // 使用彩色字符、波浪干扰、浅色背景
    let mut captcha = Captcha::new();
    captcha
        .add_chars(4)
        .apply_filter(Wave::new(2.0, 10.0).horizontal())  // 水平波浪效果
        .apply_filter(Wave::new(1.5, 8.0).vertical())    // 垂直波浪效果  
        .apply_filter(Noise::new(0.01))                   // 极少噪声
        .view(200, 70);                                   // 更大尺寸

    let captcha_tuple = captcha.as_tuple();

    // 处理验证码生成结果
    let (captcha_text, image_bytes) = match captcha_tuple {
        Some((text, bytes)) => (text, bytes),
        None => {
            tracing::warn!("验证码生成失败，使用默认验证码");
            return Ok((StatusCode::OK, Json(api_response(CaptchaResponse {
                uuid,
                img_type: "base64".to_string(),
                image: "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8DwHwAFBQIAX8jx0gAAAABJRU5ErkJggg==".to_string(),
            }))));
        }
    };

    // 将图片编码为base64（已经是120x40尺寸，无需缩放）
    let image_data = general_purpose::STANDARD.encode(&image_bytes);

    // 创建Redis连接并存储验证码
    let redis_client = redis::Client::open(crate::core::SETTINGS.redis_url())
        .map_err(|e| {
            tracing::warn!("创建Redis客户端失败: {}", e);
        });

    if let Ok(client) = redis_client {
        if let Ok(mut redis_conn) = client.get_connection() {
            let redis_key = format!("captcha:{}", uuid);
            let _: () = redis::cmd("SETEX")
                .arg(&redis_key)
                .arg(300) // 5分钟过期
                .arg(&captcha_text)
                .query(&mut redis_conn)
                .map_err(|e| {
                    tracing::warn!("存储验证码到Redis失败: {}", e);
                })
                .unwrap_or(());

            tracing::info!("验证码已存储到Redis: {}, text: {}", uuid, captcha_text);
        } else {
            tracing::warn!("获取Redis连接失败");
        }
    }

    // 返回响应（返回原始base64数据，前端会添加前缀）
    let data = CaptchaResponse {
        uuid,
        img_type: "base64".to_string(),
        image: image_data,
    };

    Ok((StatusCode::OK, Json(api_response(data))))
}

/// 用户注册
/// POST /api/v1/auth/register
pub async fn register(
    Json(_request): Json<CreateUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    // TODO: 实现用户注册逻辑
    // 1. 验证用户数据
    // 2. 检查用户名、邮箱唯一性
    // 3. 加密密码
    // 4. 创建用户
    // 5. 返回成功响应

    Ok((StatusCode::NOT_IMPLEMENTED, Json(api_response("未实现".to_string()))))
}

/// 用户登录
/// POST /api/v1/auth/login
pub async fn login(
    Json(request): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建认证服务
    let auth_service = AuthService::new(SETTINGS.token_secret_key.clone());

    // 执行登录
    let result = auth_service.login(&request, db_conn).await?;

    Ok((StatusCode::OK, Json(api_response(result))))
}

/// 刷新Token
/// POST /api/v1/auth/refresh
pub async fn refresh_token(
    Json(request): Json<RefreshTokenRequest>,
) -> Result<impl IntoResponse, AppError> {
    // 创建认证服务
    let auth_service = AuthService::new(SETTINGS.token_secret_key.clone());

    // 刷新Token
    let result = auth_service.refresh_token(&request)?;

    Ok((StatusCode::OK, Json(api_response(result))))
}

/// 用户登出
/// POST /api/v1/auth/logout
pub async fn logout(
    Json(_request): Json<LogoutRequest>,
) -> Result<impl IntoResponse, AppError> {
    // TODO: 实现登出逻辑
    // 1. 可以将Token加入黑名单
    // 2. 或者标记用户为离线状态

    Ok((StatusCode::OK, Json(api_response("登出成功".to_string()))))
}
