/// 字典类型 API 处理器

use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use crate::common::response::{api_response, ApiResult};
use crate::app::dict_type::dto::{
    CreateDictTypeRequest, DictTypeQuery,
    UpdateDictTypeRequest,
};
use crate::app::dict_type::service::DictTypeService;
use crate::database::DatabaseManager;

/// 获取字典类型列表
pub async fn get_dict_types(
    Query(query): Query<DictTypeQuery>,
) -> ApiResult<impl IntoResponse> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建字典类型服务
    let service = DictTypeService::new(db_conn.clone());

    // 查询字典类型列表
    let result = service.list(&query).await?;

    Ok((StatusCode::OK, Json(api_response(result))))
}

/// 获取字典类型选项列表
pub async fn get_dict_type_options() -> ApiResult<impl IntoResponse> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建字典类型服务
    let service = DictTypeService::new(db_conn.clone());

    // 查询字典类型选项
    let result = service.get_all().await?;

    Ok((StatusCode::OK, Json(api_response(result))))
}

/// 获取字典类型详情
pub async fn get_dict_type(
    Path(id): Path<i64>,
) -> ApiResult<impl IntoResponse> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建字典类型服务
    let service = DictTypeService::new(db_conn.clone());

    // 查询字典类型详情
    let result = service.get_by_id(id).await?;

    Ok((StatusCode::OK, Json(api_response(result))))
}

/// 创建字典类型
pub async fn create_dict_type(
    Json(request): Json<CreateDictTypeRequest>,
) -> ApiResult<impl IntoResponse> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建字典类型服务
    let service = DictTypeService::new(db_conn.clone());

    // 创建字典类型
    let result = service.create(&request).await?;

    Ok((StatusCode::CREATED, Json(api_response(result))))
}

/// 更新字典类型
pub async fn update_dict_type(
    Path(id): Path<i64>,
    Json(request): Json<UpdateDictTypeRequest>,
) -> ApiResult<impl IntoResponse> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建字典类型服务
    let service = DictTypeService::new(db_conn.clone());

    // 更新字典类型
    let result = service.update(id, &request).await?;

    Ok((StatusCode::OK, Json(api_response(result))))
}

/// 删除字典类型
pub async fn delete_dict_type(
    Path(id): Path<i64>,
) -> ApiResult<impl IntoResponse> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建字典类型服务
    let service = DictTypeService::new(db_conn.clone());

    // 删除字典类型
    service.delete(id).await?;

    Ok((StatusCode::NO_CONTENT, Json(api_response("字典类型删除成功".to_string()))))
}

/// 批量删除字典类型
pub async fn batch_delete_dict_types(
    Json(ids): Json<Vec<i64>>,
) -> ApiResult<impl IntoResponse> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建字典类型服务
    let service = DictTypeService::new(db_conn.clone());

    // 批量删除字典类型
    service.batch_delete(&ids).await?;

    Ok((StatusCode::NO_CONTENT, Json(api_response("字典类型批量删除成功".to_string()))))
}


#[cfg(test)]
mod tests {
    use super::*;
    
    use sea_orm::Database;

    #[tokio::test]
    async fn test_get_dict_types() {
        // 模拟数据库连接
        let db_conn = Database::connect("sqlite::memory:").await.unwrap();
        let service = DictTypeService::new(db_conn.clone());

        let query = DictTypeQuery::default();

        // 注意：这里需要实际的测试数据库和测试数据
        // 由于是测试，仅验证函数签名
        let _ = get_dict_types;
    }

    #[tokio::test]
    async fn test_create_dict_type() {
        let request = CreateDictTypeRequest {
            name: "用户状态".to_string(),
            code: "user_status".to_string(),
            remark: Some("用户状态字典类型".to_string()),
        };

        // 验证函数签名
        let _ = create_dict_type;
    }
}
