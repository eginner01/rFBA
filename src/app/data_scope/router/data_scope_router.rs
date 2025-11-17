/// 数据权限路由实现
/// 路径规范: /api/v1/sys/data-scopes/*

use axum::{Router, routing::{get, post, put, delete}, extract::Path, extract::Query, Json};
use axum::response::IntoResponse;
use crate::common::exception::AppError;
use crate::common::response::api_response;
use crate::database::DatabaseManager;
use crate::app::data_scope::dto::{
    DataScopeQueryParams,
    CreateDataScopeRequest,
    UpdateDataScopeRequest,
    UpdateDataScopeRuleRequest,
    DeleteDataScopeRequest,
};
use crate::app::data_scope::service::DataScopeService;

/// 数据权限路由
/// 注意：此 Router 会被挂载到 `/api/v1/sys/data-scopes` 下，所以这里的路径应为相对路径
pub fn data_scope_routes() -> Router {
    Router::new()
        // ===== 数据权限相关路由 =====
        // 注意：更具体的路由（/all, /batch）要放在带参数的路由（/{id}）之前
        .route("/all", get(get_all_data_scopes_handler))  // GET /api/v1/sys/data-scopes/all
        
        .route("/", get(get_data_scopes_handler))  // GET /api/v1/sys/data-scopes
        .route("/", post(create_data_scope_handler))  // POST /api/v1/sys/data-scopes
        .route("/", delete(batch_delete_data_scopes_handler))  // DELETE /api/v1/sys/data-scopes (批量删除，JSON body)

        .route("/{id}", get(get_data_scope_handler))  // GET /api/v1/sys/data-scopes/{id}
        .route("/{id}", put(update_data_scope_handler))  // PUT /api/v1/sys/data-scopes/{id}

        .route("/{id}/rules", get(get_data_scope_rules_handler))  // GET /api/v1/sys/data-scopes/{id}/rules
        .route("/{id}/rules", put(update_data_scope_rules_handler))  // PUT /api/v1/sys/data-scopes/{id}/rules
}

/// 获取数据权限列表（分页）
/// GET /api/v1/sys/data-scopes
async fn get_data_scopes_handler(
    Query(query): Query<DataScopeQueryParams>,
) -> Result<impl IntoResponse, AppError> {
    use tracing::{info, error};
    
    info!("接收到数据权限列表请求");
    
    let db_conn = DatabaseManager::get_connection().await;
    let data_scope_service = DataScopeService::new(db_conn.clone());

    let result = data_scope_service.get_data_scope_list(&query).await
        .map_err(|e| {
            error!("服务层返回错误: {:?}", e);
            e
        })?;

    info!("服务层成功返回结果，准备序列化响应");
    
    let response = api_response(result);
    
    info!("响应序列化成功，准备返回");

    Ok((axum::http::StatusCode::OK, Json(response)))
}

/// 获取所有数据权限
/// GET /api/v1/sys/data-scopes/all
async fn get_all_data_scopes_handler(
    Query(_query): Query<DataScopeQueryParams>,
) -> Result<impl IntoResponse, AppError> {
    let db_conn = DatabaseManager::get_connection().await;
    let data_scope_service = DataScopeService::new(db_conn.clone());

    let result = data_scope_service.get_data_scope_list(&DataScopeQueryParams::default()).await?;

    Ok((axum::http::StatusCode::OK, Json(api_response(result))))
}

/// 获取数据权限详情
/// GET /api/v1/sys/data-scopes/{role_id}
async fn get_data_scope_handler(
    Path(role_id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let db_conn = DatabaseManager::get_connection().await;
    let data_scope_service = DataScopeService::new(db_conn.clone());

    let result = data_scope_service.get_data_scope_detail(role_id).await?;

    Ok((axum::http::StatusCode::OK, Json(api_response(result))))
}

/// 创建数据权限
/// POST /api/v1/sys/data-scopes
async fn create_data_scope_handler(
    Json(request): Json<CreateDataScopeRequest>,
) -> Result<impl IntoResponse, AppError> {
    use tracing::info;
    
    info!("接收到创建数据范围请求");
    
    let db_conn = DatabaseManager::get_connection().await;
    let data_scope_service = DataScopeService::new(db_conn.clone());

    let result = data_scope_service.create_data_scope(&request).await?;

    Ok((axum::http::StatusCode::CREATED, Json(api_response(result))))
}

/// 更新数据权限
/// PUT /api/v1/sys/data-scopes/{id}
async fn update_data_scope_handler(
    Path(id): Path<i64>,
    Json(request): Json<UpdateDataScopeRequest>,
) -> Result<impl IntoResponse, AppError> {
    use tracing::info;
    
    info!("接收到更新数据范围请求: id={}", id);
    
    let db_conn = DatabaseManager::get_connection().await;
    let data_scope_service = DataScopeService::new(db_conn.clone());

    let result = data_scope_service.update_data_scope(id, &request).await?;

    Ok((axum::http::StatusCode::OK, Json(api_response(result))))
}

/// 获取数据权限规则
/// GET /api/v1/sys/data-scopes/{role_id}/rules
async fn get_data_scope_rules_handler(
    Path(role_id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let db_conn = DatabaseManager::get_connection().await;
    let data_scope_service = DataScopeService::new(db_conn.clone());

    let result = data_scope_service.get_data_scope_tree(role_id).await?;

    Ok((axum::http::StatusCode::OK, Json(api_response(result))))
}

/// 更新数据权限规则
/// PUT /api/v1/sys/data-scopes/{id}/rules
async fn update_data_scope_rules_handler(
    Path(id): Path<i64>,
    Json(request): Json<UpdateDataScopeRuleRequest>,
) -> Result<impl IntoResponse, AppError> {
    use tracing::info;
    
    info!("接收到更新数据范围规则请求: id={}", id);
    
    let db_conn = DatabaseManager::get_connection().await;
    let data_scope_service = DataScopeService::new(db_conn.clone());

    let count = data_scope_service.update_data_scope_rules(id, &request).await?;

    let message = format!("成功更新 {} 条规则", count);
    Ok((axum::http::StatusCode::OK, Json(api_response(message))))
}

/// 批量删除数据权限
/// DELETE /api/v1/sys/data-scopes
async fn batch_delete_data_scopes_handler(
    Json(request): Json<DeleteDataScopeRequest>,
) -> Result<impl IntoResponse, AppError> {
    use tracing::info;
    
    info!("接收到批量删除数据范围请求: pks={:?}", request.pks);
    
    let db_conn = DatabaseManager::get_connection().await;
    let data_scope_service = DataScopeService::new(db_conn.clone());

    let deleted_count = data_scope_service.batch_delete_data_scopes(&request.pks).await?;
    let total = request.pks.len();

    let result = format!(
        "批量删除完成：成功 {} 条，失败 {} 条",
        deleted_count,
        total - deleted_count
    );

    Ok((axum::http::StatusCode::OK, Json(api_response(result))))
}
