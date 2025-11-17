//! 代码生成器API处理器

use axum::{
    extract::{Json, Path, Query, State},
    response::Response,
    http::{header, StatusCode},
};
use sea_orm::DatabaseConnection;
use validator::Validate;

use crate::dto::*;
use crate::error::CodeGenError;
use crate::service::CodeGenService;

/// 应用状态
#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

/// 获取所有表
/// GET /codegen/tables
pub async fn get_tables(
    State(state): State<AppState>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<ApiResponse<Vec<TableInfo>>>, CodeGenError> {
    let schema = params.get("schema").map(|s| s.as_str()).unwrap_or("fba");
    
    let tables = CodeGenService::get_tables(&state.db, schema).await?;
    Ok(Json(ApiResponse::success(tables)))
}

/// 获取表字段
/// GET /codegen/tables/:name/columns
pub async fn get_columns(
    State(state): State<AppState>,
    Path(table_name): Path<String>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<ApiResponse<Vec<ColumnInfo>>>, CodeGenError> {
    let schema = params.get("schema").map(|s| s.as_str()).unwrap_or("fba");
    
    let columns = CodeGenService::get_columns(&state.db, schema, &table_name).await?;
    Ok(Json(ApiResponse::success(columns)))
}

/// 获取可用模板
/// GET /codegen/templates
pub async fn get_templates(
    State(_state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<String>>>, CodeGenError> {
    let templates = CodeGenService::get_templates();
    Ok(Json(ApiResponse::success(templates)))
}

/// 生成代码预览
/// GET /codegen/preview
pub async fn preview_code(
    State(state): State<AppState>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<ApiResponse<CodePreview>>, CodeGenError> {
    let schema = params.get("schema").map(|s| s.as_str()).unwrap_or("fba");
    let table_name = params
        .get("table_name")
        .ok_or(CodeGenError::ValidationError("缺少table_name参数".to_string()))?;
    let module_name = params
        .get("module_name")
        .ok_or(CodeGenError::ValidationError("缺少module_name参数".to_string()))?;
    
    let preview = CodeGenService::preview_code(&state.db, schema, table_name, module_name).await?;
    Ok(Json(ApiResponse::success(preview)))
}

/// 生成代码
/// POST /codegen/generate
pub async fn generate_code(
    State(state): State<AppState>,
    Json(param): Json<GenerateCodeParam>,
) -> Result<Json<ApiResponse<String>>, CodeGenError> {
    param.validate()?;
    
    // 注意：这里只是生成预览，实际写入磁盘需要额外权限
    let preview = CodeGenService::preview_code(
        &state.db,
        "fba",
        &param.table_name,
        &param.module_name,
    ).await?;
    
    let msg = format!("生成{}个文件", preview.files.len());
    Ok(Json(ApiResponse::success(msg)))
}

/// 下载代码ZIP
/// GET /codegen/download
pub async fn download_code(
    State(state): State<AppState>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Response, CodeGenError> {
    let schema = params.get("schema").map(|s| s.as_str()).unwrap_or("fba");
    let table_name = params
        .get("table_name")
        .ok_or(CodeGenError::ValidationError("缺少table_name参数".to_string()))?;
    let module_name = params
        .get("module_name")
        .ok_or(CodeGenError::ValidationError("缺少module_name参数".to_string()))?;
    
    let zip_data = CodeGenService::download_code(&state.db, schema, table_name, module_name).await?;
    
    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/zip")
        .header(
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{}.zip\"", module_name),
        )
        .body(axum::body::Body::from(zip_data))
        .map_err(|e| CodeGenError::GenerateError(e.to_string()))?;
    
    Ok(response)
}

/// 创建代码生成器路由
pub fn codegen_routes() -> axum::Router<AppState> {
    use axum::routing::{get, post};
    
    axum::Router::new()
        .route("/tables", get(get_tables))
        .route("/tables/{name}/columns", get(get_columns))
        .route("/templates", get(get_templates))
        .route("/preview", get(preview_code))
        .route("/generate", post(generate_code))
        .route("/download", get(download_code))
}
