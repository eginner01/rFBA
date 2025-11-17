//! 系统配置API处理器

use axum::{
    extract::{Path, Query, State},
    response::Json,
};
use sea_orm::DatabaseConnection;
use validator::Validate;

use crate::dto::*;
use crate::error::ConfigError;
use crate::service::ConfigService;

/// 应用状态（包含DB和Redis连接）
#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub redis: redis::aio::ConnectionManager,
}

/// 获取所有配置
/// GET /api/v1/sys/configs/all?type=EMAIL
pub async fn get_all_configs(
    State(state): State<AppState>,
    Query(query): Query<GetAllConfigQuery>,
) -> Result<Json<ApiResponse<Vec<ConfigDetail>>>, ConfigError> {
    let data = ConfigService::get_all(&state.db, query.type_filter).await?;
    Ok(Json(ApiResponse::success(data)))
}

/// 获取配置详情
/// GET /api/v1/sys/configs/{pk}
pub async fn get_config(
    State(state): State<AppState>,
    Path(pk): Path<i64>,
) -> Result<Json<ApiResponse<ConfigDetail>>, ConfigError> {
    let data = ConfigService::get_by_id(&state.db, pk).await?;
    Ok(Json(ApiResponse::success(data)))
}

/// 根据key获取配置（带缓存）
/// GET /api/v1/sys/configs/key/{key}
pub async fn get_config_by_key(
    State(mut state): State<AppState>,
    Path(key): Path<String>,
) -> Result<Json<ApiResponse<ConfigDetail>>, ConfigError> {
    let data = ConfigService::get_by_key(&state.db, &mut state.redis, &key).await?;
    Ok(Json(ApiResponse::success(data)))
}

/// 分页获取所有配置
/// GET /api/v1/sys/configs
pub async fn get_configs_paginated(
    State(state): State<AppState>,
    Query(query): Query<ConfigQuery>,
    Query(pagination): Query<PaginationQuery>,
) -> Result<Json<ApiResponse<PageData<ConfigDetail>>>, ConfigError> {
    let page_data = ConfigService::get_list(&state.db, query, pagination).await?;
    Ok(Json(ApiResponse::success(page_data)))
}

/// 创建配置
/// POST /api/v1/sys/configs
pub async fn create_config(
    State(state): State<AppState>,
    Json(param): Json<CreateConfigParam>,
) -> Result<Json<ApiResponse<ConfigDetail>>, ConfigError> {
    // 验证参数
    param.validate()?;
    
    let data = ConfigService::create(&state.db, param).await?;
    Ok(Json(ApiResponse::success_with_msg("创建成功", data)))
}

/// 更新配置
/// PUT /api/v1/sys/configs/{pk}
pub async fn update_config(
    State(mut state): State<AppState>,
    Path(pk): Path<i64>,
    Json(param): Json<UpdateConfigParam>,
) -> Result<Json<ApiResponse<()>>, ConfigError> {
    // 验证参数
    param.validate()?;
    
    let count = ConfigService::update(&state.db, &mut state.redis, pk, param).await?;
    if count > 0 {
        Ok(Json(ApiResponse::success_msg("更新成功")))
    } else {
        Err(ConfigError::OperationFailed("更新失败".to_string()))
    }
}

/// 批量删除配置
/// DELETE /api/v1/sys/configs
pub async fn delete_configs(
    State(mut state): State<AppState>,
    Json(param): Json<DeleteBatchParam>,
) -> Result<Json<ApiResponse<()>>, ConfigError> {
    let count = ConfigService::delete_batch(&state.db, &mut state.redis, param.ids).await?;
    if count > 0 {
        Ok(Json(ApiResponse::success_msg("删除成功")))
    } else {
        Err(ConfigError::OperationFailed("删除失败".to_string()))
    }
}

/// 刷新配置缓存
/// POST /api/v1/sys/configs/refresh
pub async fn refresh_config_cache(
    State(mut state): State<AppState>,
) -> Result<Json<ApiResponse<()>>, ConfigError> {
    ConfigService::refresh_cache(&state.db, &mut state.redis).await?;
    Ok(Json(ApiResponse::success_msg("缓存刷新成功")))
}

/// 创建配置路由
pub fn config_routes() -> axum::Router<AppState> {
    use axum::routing::{delete, get, post, put};
    
    axum::Router::new()
        .route("/all", get(get_all_configs))
        .route("/{pk}", get(get_config))
        .route("/key/{key}", get(get_config_by_key))
        .route("/", get(get_configs_paginated))
        .route("/", post(create_config))
        .route("/{pk}", put(update_config))
        .route("/", delete(delete_configs))
        .route("/refresh", post(refresh_config_cache))
}
