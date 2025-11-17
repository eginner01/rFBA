/// 操作日志中间件
/// 自动记录HTTP请求的操作日志

use axum::{
    extract::Request,
    http::{StatusCode, Method},
    middleware::Next,
    response::Response,
};
use std::time::Instant;
use crate::app::opera_log::service::OperaLogService;
use crate::common::exception::AppError;
use crate::database::DatabaseManager;
use crate::middleware::permission_middleware::extract_user_from_request;

/// 操作日志中间件处理器
/// 返回HTTP响应
pub async fn opera_log_middleware(
    request: Request,
    next: Next,
) -> Response {
    let start_time = Instant::now();
    let method = request.method().clone();
    let uri = request.uri().path().to_string();

    // 跳过不需要记录操作日志的路径
    if is_skip_path(&uri) {
        return next.run(request).await;
    }

    // 获取客户端IP和操作地点
    let client_ip = extract_client_ip(&request);
    let oper_location = extract_oper_location(&request);

    // 获取用户信息（在request被消费前），需要克隆以避免借用问题
    let user_info = extract_user_from_request(&request).cloned();

    // 执行请求
    let response = next.run(request).await;
    let cost_time = start_time.elapsed().as_millis() as i64;

    // 如果有用户信息，记录操作日志
    if let Some(payload) = user_info {
        let _ = record_opera_log(
            &uri,
            &method,
            client_ip.as_str(),
            oper_location,
            cost_time,
            &response,
            &payload,
        ).await;
    }

    response
}

/// 记录操作日志
/// 返回操作结果
async fn record_opera_log(
    uri: &str,
    method: &Method,
    client_ip: &str,
    oper_location: &str,
    cost_time: i64,
    response: &Response,
    payload: &crate::utils::encrypt::JwtPayload,
) -> Result<(), AppError> {
    let db_conn = DatabaseManager::get_connection().await;
    let opera_log_service = OperaLogService::new(db_conn.clone());

    // 解析请求体（如果存在）
    let request_body = extract_request_body(payload).await;

    // 解析响应体（如果存在）
    let response_body = extract_response_body(response).await;

    // 判断操作是否成功
    let status = if response.status() == StatusCode::OK {
        0
    } else {
        1
    };

    // 构建操作日志请求
    let log_request = crate::app::opera_log::dto::CreateOperaLogRequest {
        title: format!("{} {}", method.as_str(), uri),
        business_type: get_business_type_from_method(method),
        method: method.as_str().to_string(),
        request_method: method.as_str().to_string(),
        operator_type: 1, // 后台用户
        user_id: Some(payload.sub.parse::<i64>().unwrap_or(0)),
        user_name: payload.sub.clone(), // 使用 sub 作为用户标识
        dept_id: None, // TODO: 从用户信息中获取
        dept_name: None, // TODO: 从用户信息中获取
        oper_url: uri.to_string(),
        oper_ip: client_ip.to_string(),
        oper_location: Some(oper_location.to_string()),
        oper_param: request_body,
        json_result: response_body,
        status,
        error_msg: if status == 1 { Some("Operation failed".to_string()) } else { None },
        cost_time: Some(cost_time),
    };

    // 记录操作日志
    let _ = opera_log_service.create_opera_log(&log_request).await;

    Ok(())
}

/// 检查是否为跳过路径
/// 返回是否跳过
fn is_skip_path(path: &str) -> bool {
    // 跳过不需要记录操作日志的路径
    let skip_paths = [
        "/",
        "/health",
        "/api/v1/auth/",
        "/api/v1/opera-log/", // 操作日志管理相关API
        "/api/v1/access-log/", // 访问日志管理相关API
        "/api/v1/login-log/", // 登录日志管理相关API
    ];

    skip_paths.iter().any(|p| path.starts_with(p))
}

/// 提取客户端IP
/// 返回客户端IP
fn extract_client_ip(request: &Request) -> String {
    // 尝试从X-Forwarded-For头获取IP
    if let Some(ip) = request.headers().get("X-Forwarded-For") {
        if let Ok(ip_str) = ip.to_str() {
            return ip_str.split(',').next().unwrap_or(ip_str).trim().to_string();
        }
    }

    // 尝试从X-Real-IP头获取IP
    if let Some(ip) = request.headers().get("X-Real-IP") {
        if let Ok(ip_str) = ip.to_str() {
            return ip_str.to_string();
        }
    }

    // 从连接信息获取IP
    if let Some(remote_addr) = request.extensions().get::<axum::extract::ConnectInfo<std::net::SocketAddr>>() {
        return remote_addr.0.ip().to_string();
    }

    "0.0.0.0".to_string()
}

/// 提取操作地点
/// 返回操作地点
fn extract_oper_location(request: &Request) -> &'static str {
    // TODO: 根据IP地址获取地理位置
    // 这里可以集成第三方IP归属地查询服务
    "本地"
}

/// 提取请求体
/// 返回请求体JSON字符串
async fn extract_request_body(payload: &crate::utils::encrypt::JwtPayload) -> Option<String> {
    // TODO: 从请求中提取请求体
    // 这里可以集成请求体解析逻辑
    None
}

/// 提取响应体
/// 返回响应体JSON字符串
async fn extract_response_body(response: &Response) -> Option<String> {
    // TODO: 从响应中提取响应体
    // 这里可以集成响应体解析逻辑
    None
}

/// 根据HTTP方法获取业务类型
/// 返回业务类型
fn get_business_type_from_method(method: &Method) -> i32 {
    match method.as_str() {
        "POST" => 1, // 新增
        "PUT" => 2,  // 修改
        "DELETE" => 3, // 删除
        _ => 0, // 其它
    }
}
