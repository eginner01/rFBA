//! OAuth2 API处理器

use axum::{
    extract::{Json, Query, State},
    response::Redirect,
};
use sea_orm::DatabaseConnection;
use validator::Validate;
use std::collections::HashMap;

use crate::dto::*;
use crate::error::OAuth2Error;
use crate::service::{OAuth2Config, OAuth2Service};

/// 应用状态
#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub oauth2_config: OAuth2Config,
}

/// GitHub授权
/// GET /oauth2/github/authorize
pub async fn github_authorize(
    State(state): State<AppState>,
) -> Result<Redirect, OAuth2Error> {
    let auth_url = OAuth2Service::get_github_auth_url(&state.oauth2_config)?;
    Ok(Redirect::to(&auth_url))
}

/// GitHub回调
/// GET /oauth2/github/callback
pub async fn github_callback(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<ApiResponse<OAuthCallbackResponse>>, OAuth2Error> {
    let code = params
        .get("code")
        .ok_or(OAuth2Error::ValidationError("缺少code参数".to_string()))?
        .clone();

    let callback_data = OAuth2Service::github_callback(&state.oauth2_config, code).await?;
    
    Ok(Json(ApiResponse::success(callback_data)))
}

/// Google授权
/// GET /oauth2/google/authorize
pub async fn google_authorize(
    State(state): State<AppState>,
) -> Result<Redirect, OAuth2Error> {
    let auth_url = OAuth2Service::get_google_auth_url(&state.oauth2_config)?;
    Ok(Redirect::to(&auth_url))
}

/// Google回调
/// GET /oauth2/google/callback
pub async fn google_callback(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<ApiResponse<OAuthCallbackResponse>>, OAuth2Error> {
    let code = params
        .get("code")
        .ok_or(OAuth2Error::ValidationError("缺少code参数".to_string()))?
        .clone();

    let callback_data = OAuth2Service::google_callback(&state.oauth2_config, code).await?;
    
    Ok(Json(ApiResponse::success(callback_data)))
}

/// 绑定OAuth账号
/// POST /oauth2/bind
pub async fn bind_oauth(
    State(state): State<AppState>,
    Json(param): Json<BindOAuthParam>,
) -> Result<Json<ApiResponse<OAuthBindInfo>>, OAuth2Error> {
    param.validate()?;
    
    // 注意：这里需要从JWT中获取user_id
    // 暂时使用占位值，实际应用中需要实现JWT认证
    let user_id = 1i64; // TODO: 从JWT获取
    
    // 根据provider处理不同的OAuth流程
    let callback_data = match param.provider.as_str() {
        "github" => OAuth2Service::github_callback(&state.oauth2_config, param.code).await?,
        "google" => OAuth2Service::google_callback(&state.oauth2_config, param.code).await?,
        _ => return Err(OAuth2Error::ValidationError("不支持的OAuth提供商".to_string())),
    };
    
    let bind_info = OAuth2Service::bind_oauth(
        &state.db,
        user_id,
        &param.provider,
        &callback_data.user_info.provider_user_id,
        &callback_data.access_token,
    ).await?;
    
    Ok(Json(ApiResponse::success(bind_info)))
}

/// 解绑OAuth账号
/// DELETE /oauth2/unbind
pub async fn unbind_oauth(
    State(state): State<AppState>,
    Json(param): Json<UnbindOAuthParam>,
) -> Result<Json<ApiResponse<()>>, OAuth2Error> {
    param.validate()?;
    
    // 注意：这里需要从JWT中获取user_id
    let user_id = 1i64; // TODO: 从JWT获取
    
    OAuth2Service::unbind_oauth(&state.db, user_id, &param.provider).await?;
    
    Ok(Json(ApiResponse::success_msg("解绑成功")))
}

/// LinuxDo授权
/// GET /oauth2/linux-do/authorize
pub async fn linux_do_authorize(
    State(state): State<AppState>,
) -> Result<Redirect, OAuth2Error> {
    let auth_url = OAuth2Service::get_linux_do_auth_url(&state.oauth2_config)?;
    Ok(Redirect::to(&auth_url))
}

/// LinuxDo回调
/// GET /oauth2/linux-do/callback
pub async fn linux_do_callback(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<ApiResponse<OAuthCallbackResponse>>, OAuth2Error> {
    let code = params
        .get("code")
        .ok_or(OAuth2Error::ValidationError("缺少code参数".to_string()))?
        .clone();

    let callback_data = OAuth2Service::linux_do_callback(&state.oauth2_config, code).await?;
    
    Ok(Json(ApiResponse::success(callback_data)))
}

/// 创建OAuth2路由
pub fn oauth2_routes() -> axum::Router<AppState> {
    use axum::routing::{delete, get, post};
    
    axum::Router::new()
        .route("/github/authorize", get(github_authorize))
        .route("/github/callback", get(github_callback))
        .route("/google/authorize", get(google_authorize))
        .route("/google/callback", get(google_callback))
        .route("/linux-do/authorize", get(linux_do_authorize))
        .route("/linux-do/callback", get(linux_do_callback))
        .route("/bind", post(bind_oauth))
        .route("/unbind", delete(unbind_oauth))
}
