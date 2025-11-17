//! 字典类型API处理器

use axum::{
    extract::{Path, Query, State},
    response::Json,
};
use sea_orm::DatabaseConnection;
use validator::Validate;

use crate::dto::*;
use crate::error::DictError;
use crate::service::DictTypeService;

/// 获取所有字典类型
/// GET /api/v1/sys/dict-types/all
pub async fn get_all_dict_types(
    State(db): State<DatabaseConnection>,
) -> Result<Json<ApiResponse<Vec<DictTypeDetail>>>, DictError> {
    let data = DictTypeService::get_all(&db).await?;
    Ok(Json(ApiResponse::success(data)))
}

/// 获取字典类型详情
/// GET /api/v1/sys/dict-types/{pk}
pub async fn get_dict_type(
    State(db): State<DatabaseConnection>,
    Path(pk): Path<i64>,
) -> Result<Json<ApiResponse<DictTypeDetail>>, DictError> {
    let data = DictTypeService::get_by_id(&db, pk).await?;
    Ok(Json(ApiResponse::success(data)))
}

/// 分页获取所有字典类型
/// GET /api/v1/sys/dict-types
pub async fn get_dict_types_paginated(
    State(db): State<DatabaseConnection>,
    Query(query): Query<DictTypeQuery>,
    Query(pagination): Query<PaginationQuery>,
) -> Result<Json<ApiResponse<PageData<DictTypeDetail>>>, DictError> {
    let page_data = DictTypeService::get_list(&db, query, pagination).await?;
    Ok(Json(ApiResponse::success(page_data)))
}

/// 创建字典类型
/// POST /api/v1/sys/dict-types
pub async fn create_dict_type(
    State(db): State<DatabaseConnection>,
    Json(param): Json<CreateDictTypeParam>,
) -> Result<Json<ApiResponse<DictTypeDetail>>, DictError> {
    // 验证参数
    param.validate()?;
    
    let data = DictTypeService::create(&db, param).await?;
    Ok(Json(ApiResponse::success_with_msg("创建成功", data)))
}

/// 更新字典类型
/// PUT /api/v1/sys/dict-types/{pk}
pub async fn update_dict_type(
    State(db): State<DatabaseConnection>,
    Path(pk): Path<i64>,
    Json(param): Json<UpdateDictTypeParam>,
) -> Result<Json<ApiResponse<()>>, DictError> {
    // 验证参数
    param.validate()?;
    
    let count = DictTypeService::update(&db, pk, param).await?;
    if count > 0 {
        Ok(Json(ApiResponse::success_msg("更新成功")))
    } else {
        Err(DictError::OperationFailed("更新失败".to_string()))
    }
}

/// 批量删除字典类型
/// DELETE /api/v1/sys/dict-types
pub async fn delete_dict_types(
    State(db): State<DatabaseConnection>,
    Json(param): Json<DeleteBatchParam>,
) -> Result<Json<ApiResponse<()>>, DictError> {
    let count = DictTypeService::delete_batch(&db, param.ids).await?;
    if count > 0 {
        Ok(Json(ApiResponse::success_msg("删除成功")))
    } else {
        Err(DictError::OperationFailed("删除失败".to_string()))
    }
}

/// 创建字典类型路由
pub fn dict_type_routes() -> axum::Router<DatabaseConnection> {
    use axum::routing::{delete, get, post, put};
    
    axum::Router::new()
        .route("/all", get(get_all_dict_types))
        .route("/{pk}", get(get_dict_type))
        .route("/", get(get_dict_types_paginated))
        .route("/", post(create_dict_type))
        .route("/{pk}", put(update_dict_type))
        .route("/", delete(delete_dict_types))
}
