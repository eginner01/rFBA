use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
    routing::{get, post},
    Router,
};
use axum::extract::Query;
use crate::app::auth::dto::{
    LoginRequest,
    RefreshTokenRequest, LogoutRequest,
};
use crate::common::exception::AppError;
use crate::common::response::api_response;

#[derive(serde::Deserialize)]
pub struct CaptchaQuery {
    pub key: Option<String>,
}

pub fn auth_routes() -> Router {
    Router::new()
        .route("/captcha", get(get_captcha_handler))
        .route("/login", post(login_handler))
        .route("/refresh", post(refresh_token_handler))
        .route("/logout", post(logout_handler))
        .route("/codes", get(get_codes_handler))
}

/// GET /api/v1/auth/captcha
async fn get_captcha_handler(
    Query(_query): Query<CaptchaQuery>,
) -> Result<impl IntoResponse, AppError> {
    crate::app::user::api::auth::get_captcha_internal().await
}

/// POST /api/v1/auth/login
async fn login_handler(
    Json(request): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    let db_conn = crate::database::DatabaseManager::get_connection().await;
    let (status, json_response) = crate::app::auth::api::auth::login(
        State(db_conn.clone()),
        Json(request),
    ).await?;
    Ok((status, json_response))
}

/// POST /api/v1/auth/refresh
async fn refresh_token_handler(
    Json(request): Json<RefreshTokenRequest>,
) -> Result<impl IntoResponse, AppError> {
    let auth_service = crate::app::auth::service::AuthService::new(
        crate::core::SETTINGS.token_secret_key.clone()
    );
    let result = auth_service.refresh_token(&request)?;
    Ok((StatusCode::OK, Json(api_response(result))))
}

/// POST /api/v1/auth/logout
async fn logout_handler(
    Json(request): Json<LogoutRequest>,
) -> Result<impl IntoResponse, AppError> {
    let response = crate::app::auth::api::auth::logout(Json(request)).await?;
    Ok(response)
}

/// GET /api/v1/auth/codes
async fn get_codes_handler(
    _auth_context: axum::extract::Extension<crate::middleware::jwt_auth_middleware::AuthContext>,
) -> Result<impl IntoResponse, AppError> {
    // TODO: 实现获取权限码逻辑
    let codes: Vec<String> = vec![];
    Ok((StatusCode::OK, Json(crate::common::response::api_response(codes))))
}
