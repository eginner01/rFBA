/// 数据权限中间件
/// 在请求处理过程中应用数据权限过滤

use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
};
use tracing::warn;
use crate::app::data_scope::service::DataScopeService;
use crate::app::data_scope::dto::{UserDataScope, DataScopeFilter};
use crate::common::exception::AppError;
use crate::middleware::permission_middleware::extract_user_from_request;
use crate::database::DatabaseManager;

/// 数据权限中间件处理器
/// 返回HTTP响应
pub async fn data_scope_middleware(
    request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let uri = request.uri().path().to_string();
    let _method = request.method().clone();

    // 跳过不需要数据权限验证的路径
    if is_skip_path(&uri) {
        return Ok(next.run(request).await);
    }

    // 从请求中提取用户信息
    let jwt_payload = match extract_user_from_request(&request) {
        Some(payload) => payload,
        None => {
            // 如果没有用户信息，跳过数据权限验证
            return Ok(next.run(request).await);
        }
    };

    // 解析用户ID
    let user_id = match jwt_payload.sub.parse::<i64>() {
        Ok(id) => id,
        Err(_) => {
            warn!("Invalid user ID in token: {}", jwt_payload.sub);
            return Ok(next.run(request).await);
        }
    };

    // 获取用户数据权限
    let db_conn = DatabaseManager::get_connection().await;
    let data_scope_service = DataScopeService::new(db_conn.clone());

    let user_data_scope = match data_scope_service.get_user_data_scope(user_id).await {
        Ok(scope) => scope,
        Err(_) => {
            // 如果获取数据权限失败，跳过验证
            return Ok(next.run(request).await);
        }
    };

    // 将用户数据权限添加到请求扩展中
    let mut request = request;
    request.extensions_mut().insert(user_data_scope);

    // 执行下一个处理器
    Ok(next.run(request).await)
}

/// 检查是否为跳过路径
/// 返回是否跳过
fn is_skip_path(path: &str) -> bool {
    // 跳过不需要数据权限验证的路径
    let skip_paths = [
        "/",
        "/health",
        "/api/v1/auth/",
        "/api/v1/data-scope/", // 数据权限管理相关API
    ];

    skip_paths.iter().any(|p| path.starts_with(p))
}

/// 从请求中提取数据权限
/// 返回用户数据权限
pub fn extract_data_scope_from_request(
    request: &Request,
) -> Option<&UserDataScope> {
    request.extensions().get::<UserDataScope>()
}

/// 从请求中提取数据权限过滤条件
/// 返回数据权限过滤条件
pub async fn extract_data_scope_filter_from_request(
    request: &Request,
) -> Option<DataScopeFilter> {
    if let Some(user_data_scope) = extract_data_scope_from_request(request) {
        let db_conn = DatabaseManager::get_connection().await;
        let data_scope_service = DataScopeService::new(db_conn.clone());
        data_scope_service.get_data_scope_filter(user_data_scope.user_id).await.ok()
    } else {
        None
    }
}
