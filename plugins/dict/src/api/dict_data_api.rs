//! 字典数据API处理器

use axum::{
    extract::{Path, Query, State},
    response::Json,
};
use sea_orm::DatabaseConnection;
use validator::Validate;

use crate::dto::*;
use crate::error::DictError;
use crate::service::DictDataService;

/// 获取所有字典数据
/// GET /api/v1/sys/dict-datas/all
pub async fn get_all_dict_datas(
    State(db): State<DatabaseConnection>,
) -> Result<Json<ApiResponse<Vec<DictDataDetail>>>, DictError> {
    let data = DictDataService::get_all(&db).await?;
    Ok(Json(ApiResponse::success(data)))
}

/// 获取字典数据详情
/// GET /api/v1/sys/dict-datas/{pk}
pub async fn get_dict_data(
    State(db): State<DatabaseConnection>,
    Path(pk): Path<i64>,
) -> Result<Json<ApiResponse<DictDataDetail>>, DictError> {
    let data = DictDataService::get_by_id(&db, pk).await?;
    Ok(Json(ApiResponse::success(data)))
}

/// 根据类型编码获取字典数据列表
/// GET /api/v1/sys/dict-datas/type-codes/{code}
pub async fn get_dict_data_by_type_code(
    State(db): State<DatabaseConnection>,
    Path(code): Path<String>,
) -> Result<Json<ApiResponse<Vec<DictDataDetail>>>, DictError> {
    let data = DictDataService::get_by_type_code(&db, &code).await?;
    Ok(Json(ApiResponse::success(data)))
}

/// 分页获取所有字典数据
/// GET /api/v1/sys/dict-datas
pub async fn get_dict_datas_paginated(
    State(db): State<DatabaseConnection>,
    Query(query): Query<DictDataQuery>,
    Query(pagination): Query<PaginationQuery>,
) -> Result<Json<ApiResponse<PageData<DictDataDetail>>>, DictError> {
    let page_data = DictDataService::get_list(&db, query, pagination).await?;
    Ok(Json(ApiResponse::success(page_data)))
}

/// 创建字典数据
/// POST /api/v1/sys/dict-datas
pub async fn create_dict_data(
    State(db): State<DatabaseConnection>,
    Json(param): Json<CreateDictDataParam>,
) -> Result<Json<ApiResponse<DictDataDetail>>, DictError> {
    // 验证参数
    param.validate()?;
    
    let data = DictDataService::create(&db, param).await?;
    Ok(Json(ApiResponse::success_with_msg("创建成功", data)))
}

/// 更新字典数据
/// PUT /api/v1/sys/dict-datas/{pk}
pub async fn update_dict_data(
    State(db): State<DatabaseConnection>,
    Path(pk): Path<i64>,
    Json(param): Json<UpdateDictDataParam>,
) -> Result<Json<ApiResponse<()>>, DictError> {
    // 验证参数
    param.validate()?;
    
    let count = DictDataService::update(&db, pk, param).await?;
    if count > 0 {
        Ok(Json(ApiResponse::success_msg("更新成功")))
    } else {
        Err(DictError::OperationFailed("更新失败".to_string()))
    }
}

/// 批量删除字典数据
/// DELETE /api/v1/sys/dict-datas
pub async fn delete_dict_datas(
    State(db): State<DatabaseConnection>,
    Json(param): Json<DeleteBatchParam>,
) -> Result<Json<ApiResponse<()>>, DictError> {
    let count = DictDataService::delete_batch(&db, param.ids).await?;
    if count > 0 {
        Ok(Json(ApiResponse::success_msg("删除成功")))
    } else {
        Err(DictError::OperationFailed("删除失败".to_string()))
    }
}

/// 创建字典数据路由
pub fn dict_data_routes() -> axum::Router<DatabaseConnection> {
    use axum::routing::{delete, get, post, put};
    
    axum::Router::new()
        .route("/all", get(get_all_dict_datas))
        .route("/{pk}", get(get_dict_data))
        .route("/type-codes/{code}", get(get_dict_data_by_type_code))
        .route("/", get(get_dict_datas_paginated))
        .route("/", post(create_dict_data))
        .route("/{pk}", put(update_dict_data))
        .route("/", delete(delete_dict_datas))
}
