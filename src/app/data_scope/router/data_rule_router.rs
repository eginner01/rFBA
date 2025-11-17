/// 数据规则路由实现
/// 按照Python后端路径规范: /api/v1/sys/data-rules/*

use axum::{Router, routing::{get, post, put, delete}, extract::Path, extract::Query, Json};
use axum::response::IntoResponse;
use crate::common::exception::AppError;
use crate::common::response::api_response;
use crate::database::DatabaseManager;
use crate::app::data_scope::dto::{
    CreateDataRuleRequest, UpdateDataRuleRequest, DataRulePaginationQuery, DataRuleQueryParams,
};
use crate::app::data_scope::service::DataRuleService;

/// 数据规则路由
pub fn data_rule_routes() -> Router {
    Router::new()
        // ===== 数据规则相关路由 =====
        .route("/", get(get_data_rules_handler))  // GET /api/v1/sys/data-rules
        .route("/all", get(get_all_data_rules_handler))  // GET /api/v1/sys/data-rules/all
        .route("/models", get(get_data_rule_models_handler))  // GET /api/v1/sys/data-rules/models
        .route("/", post(create_data_rule_handler))  // POST /api/v1/sys/data-rules

        .route("/{id}", get(get_data_rule_handler))  // GET /api/v1/sys/data-rules/{id}
        .route("/{id}", put(update_data_rule_handler))  // PUT /api/v1/sys/data-rules/{id}
        .route("/{id}", delete(delete_data_rule_handler))  // DELETE /api/v1/sys/data-rules/{id}

        .route("/models/{model}/columns", get(get_model_columns_handler))  // GET /api/v1/sys/data-rules/models/{model}/columns

        .route("/batch", delete(batch_delete_data_rules_handler))  // DELETE /api/v1/sys/data-rules/batch
}

/// 获取数据规则列表（分页）
/// GET /api/v1/sys/data-rules
async fn get_data_rules_handler(
    Query(query): Query<DataRulePaginationQuery>,
) -> Result<impl IntoResponse, AppError> {
    use tracing::{info, error};
    
    info!("获取数据规则列表，查询参数: page={:?}, size={:?}", query.page, query.size);
    
    let db_conn = DatabaseManager::get_connection().await;
    let data_rule_service = DataRuleService::new(db_conn.clone());

    let result = data_rule_service.get_data_rule_list(&query).await
        .map_err(|e| {
            error!("获取数据规则列表失败: {:?}", e);
            e
        })?;

    info!("成功获取数据规则列表，返回 {} 条记录", result.total);

    Ok((axum::http::StatusCode::OK, Json(api_response(result))))
}

/// 获取所有数据规则
/// GET /api/v1/sys/data-rules/all
async fn get_all_data_rules_handler(
    Query(query): Query<DataRuleQueryParams>,
) -> Result<impl IntoResponse, AppError> {
    let db_conn = DatabaseManager::get_connection().await;
    let data_rule_service = DataRuleService::new(db_conn.clone());

    let result = data_rule_service.get_all_data_rules(&query).await?;

    Ok((axum::http::StatusCode::OK, Json(api_response(result))))
}

/// 获取数据规则模型列表
/// GET /api/v1/sys/data-rules/models
async fn get_data_rule_models_handler(
    Query(query): Query<serde_json::Value>,
) -> Result<impl IntoResponse, AppError> {
    let db_conn = DatabaseManager::get_connection().await;
    let data_rule_service = DataRuleService::new(db_conn.clone());

    let keyword = query.get("keyword").and_then(|v| v.as_str());
    let result = data_rule_service.get_data_rule_models(keyword).await?;

    Ok((axum::http::StatusCode::OK, Json(api_response(result))))
}

/// 获取模型列信息
/// GET /api/v1/sys/data-rules/models/{model}/columns
async fn get_model_columns_handler(
    Path(model): Path<String>,
    Query(query): Query<serde_json::Value>,
) -> Result<impl IntoResponse, AppError> {
    let db_conn = DatabaseManager::get_connection().await;
    let data_rule_service = DataRuleService::new(db_conn.clone());

    let include_system = query.get("include_system").and_then(|v| v.as_bool());
    let result = data_rule_service.get_model_columns(&model, include_system).await?;

    Ok((axum::http::StatusCode::OK, Json(api_response(result))))
}

/// 创建数据规则
/// POST /api/v1/sys/data-rules
async fn create_data_rule_handler(
    Json(request): Json<CreateDataRuleRequest>,
) -> Result<impl IntoResponse, AppError> {
    let db_conn = DatabaseManager::get_connection().await;
    let data_rule_service = DataRuleService::new(db_conn.clone());

    let result = data_rule_service.create_data_rule(&request).await?;

    Ok((axum::http::StatusCode::CREATED, Json(api_response(result))))
}

/// 获取数据规则详情
/// GET /api/v1/sys/data-rules/{id}
async fn get_data_rule_handler(
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let db_conn = DatabaseManager::get_connection().await;
    let data_rule_service = DataRuleService::new(db_conn.clone());

    let result = data_rule_service.get_data_rule_detail(id).await?;

    Ok((axum::http::StatusCode::OK, Json(api_response(result))))
}

/// 更新数据规则
/// PUT /api/v1/sys/data-rules/{id}
async fn update_data_rule_handler(
    Path(id): Path<i64>,
    Json(request): Json<UpdateDataRuleRequest>,
) -> Result<impl IntoResponse, AppError> {
    let db_conn = DatabaseManager::get_connection().await;
    let data_rule_service = DataRuleService::new(db_conn.clone());

    let result = data_rule_service.update_data_rule(id, &request).await?;

    Ok((axum::http::StatusCode::OK, Json(api_response(result))))
}

/// 删除数据规则
/// DELETE /api/v1/sys/data-rules/{id}
async fn delete_data_rule_handler(
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let db_conn = DatabaseManager::get_connection().await;
    let data_rule_service = DataRuleService::new(db_conn.clone());

    data_rule_service.delete_data_rule(id).await?;

    Ok((axum::http::StatusCode::OK, Json(api_response("数据规则删除成功".to_string()))))
}

/// 批量删除数据规则
/// DELETE /api/v1/sys/data-rules/batch
async fn batch_delete_data_rules_handler(
    Json(ids): Json<Vec<i64>>,
) -> Result<impl IntoResponse, AppError> {
    let db_conn = DatabaseManager::get_connection().await;
    let data_rule_service = DataRuleService::new(db_conn.clone());

    let result = data_rule_service.batch_delete_data_rules(&ids).await?;

    Ok((axum::http::StatusCode::OK, Json(api_response(result))))
}
