//! 业务模型API处理器

use axum::{
    extract::{Json, Path, Query, State},
};
use validator::Validate;

use crate::dto::*;
use crate::error::CodeGenError;
use crate::service::{BusinessService, CodeGenService};

/// 获取所有业务
/// GET /codegen/businesses/all
pub async fn get_all_businesses(
    State(state): State<crate::api::AppState>,
) -> Result<Json<ApiResponse<Vec<GenBusinessDetail>>>, CodeGenError> {
    let data = BusinessService::get_all(&state.db).await?;
    Ok(Json(ApiResponse::success(data)))
}

/// 获取业务详情
/// GET /codegen/businesses/{id}
pub async fn get_business(
    State(state): State<crate::api::AppState>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<GenBusinessDetail>>, CodeGenError> {
    let data = BusinessService::get_by_id(&state.db, id).await?;
    Ok(Json(ApiResponse::success(data)))
}

/// 分页获取业务列表
/// GET /codegen/businesses
pub async fn get_businesses(
    State(state): State<crate::api::AppState>,
    Query(params): Query<std::collections::HashMap<String, String>>,
    Query(pagination): Query<PaginationQuery>,
) -> Result<Json<ApiResponse<PageData<GenBusinessDetail>>>, CodeGenError> {
    let table_name = params.get("table_name").map(|s| s.clone());
    
    let page_data = BusinessService::get_list(&state.db, table_name, pagination).await?;
    Ok(Json(ApiResponse::success(page_data)))
}

/// 获取业务的所有列
/// GET /codegen/businesses/{id}/columns
pub async fn get_business_columns(
    State(state): State<crate::api::AppState>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<Vec<GenColumnDetail>>>, CodeGenError> {
    let columns = BusinessService::get_columns(&state.db, id).await?;
    Ok(Json(ApiResponse::success(columns)))
}

/// 创建业务
/// POST /codegen/businesses
pub async fn create_business(
    State(state): State<crate::api::AppState>,
    Json(param): Json<CreateGenBusinessParam>,
) -> Result<Json<ApiResponse<GenBusinessDetail>>, CodeGenError> {
    param.validate()?;
    
    let data = BusinessService::create(&state.db, param).await?;
    Ok(Json(ApiResponse::success(data)))
}

/// 更新业务
/// PUT /codegen/businesses/{id}
pub async fn update_business(
    State(state): State<crate::api::AppState>,
    Path(id): Path<i64>,
    Json(param): Json<UpdateGenBusinessParam>,
) -> Result<Json<ApiResponse<()>>, CodeGenError> {
    param.validate()?;
    
    let count = BusinessService::update(&state.db, id, param).await?;
    if count > 0 {
        Ok(Json(ApiResponse::success_msg("更新成功")))
    } else {
        Err(CodeGenError::NotFound("业务不存在".to_string()))
    }
}

/// 删除业务
/// DELETE /codegen/businesses/{id}
pub async fn delete_business(
    State(state): State<crate::api::AppState>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<()>>, CodeGenError> {
    let count = BusinessService::delete(&state.db, id).await?;
    if count > 0 {
        Ok(Json(ApiResponse::success_msg("删除成功")))
    } else {
        Err(CodeGenError::NotFound("业务不存在".to_string()))
    }
}

/// 导入表结构
/// POST /codegen/businesses/import
pub async fn import_table(
    State(state): State<crate::api::AppState>,
    Json(param): Json<ImportTableParam>,
) -> Result<Json<ApiResponse<GenBusinessDetail>>, CodeGenError> {
    param.validate()?;
    
    let data = BusinessService::import_table(&state.db, param).await?;
    Ok(Json(ApiResponse::success(data)))
}

/// 获取业务代码生成路径
/// GET /codegen/businesses/{id}/paths
pub async fn get_business_paths(
    State(state): State<crate::api::AppState>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<Vec<String>>>, CodeGenError> {
    let paths = BusinessService::get_generate_paths(&state.db, id).await?;
    Ok(Json(ApiResponse::success(paths)))
}

/// 生成代码到本地文件系统
/// POST /codegen/businesses/{id}/generate
pub async fn generate_to_filesystem(
    State(state): State<crate::api::AppState>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<Vec<String>>>, CodeGenError> {
    let created_files = CodeGenService::generate_by_business_id(&state.db, id).await?;
    Ok(Json(ApiResponse::success(created_files)))
}

/// 创建业务模型路由
pub fn business_routes() -> axum::Router<crate::api::AppState> {
    use axum::routing::{delete, get, post, put};
    
    axum::Router::new()
        .route("/all", get(get_all_businesses))
        .route("/", get(get_businesses))
        .route("/", post(create_business))
        .route("/import", post(import_table))  // 导入表
        .route("/{id}", get(get_business))
        .route("/{id}", put(update_business))
        .route("/{id}", delete(delete_business))
        .route("/{id}/columns", get(get_business_columns))
        .route("/{id}/paths", get(get_business_paths))  // 获取生成路径
        .route("/{id}/generate", post(generate_to_filesystem))  // 生成到文件系统 ✨
}
