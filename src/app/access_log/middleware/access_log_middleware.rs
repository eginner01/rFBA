/// 访问日志中间件
/// 自动记录HTTP请求的访问日志

use axum::{
    extract::Request,
    http::{StatusCode, Method},
    middleware::Next,
    response::Response,
};
use tracing::{info, debug};
use std::time::Instant;
use crate::app::access_log::service::AccessLogService;
use crate::common::exception::AppError;
use crate::database::DatabaseManager;
use crate::middleware::permission_middleware::extract_user_from_request;

/// 访问日志中间件处理器
/// 返回HTTP响应
pub async fn access_log_middleware(
    request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let start_time = Instant::now();
    let method = request.method().clone();
    let uri = request.uri().path().to_string();
    let query = request.uri().query().unwrap_or("");

    // 跳过不需要记录访问日志的路径
    if is_skip_path(&uri) {
        return Ok(next.run(request).await);
    }

    // 获取客户端信息
    let client_ip = extract_client_ip(&request);
    let user_agent = extract_user_agent(&request);
    let (os, browser, device_type) = parse_user_agent(&user_agent);
    let referer = extract_referer(&request);

    // 获取请求ID（trace_id）
    let trace_id = generate_trace_id();

    // 执行请求
    let response = next.run(request).await?;

    // 记录访问日志
    let cost_time = start_time.elapsed().as_millis() as i64;
    let _ = record_access_log(
        &trace_id,
        &uri,
        &query,
        &method,
        &client_ip,
        &user_agent,
        &os,
        &browser,
        &device_type,
        &referer,
        &response,
        cost_time,
    ).await;

    Ok(response)
}

/// 记录访问日志
/// 返回操作结果
async fn record_access_log(
    trace_id: &str,
    uri: &str,
    query: &str,
    method: &Method,
    client_ip: &str,
    user_agent: &str,
    os: &str,
    browser: &str,
    device_type: &str,
    referer: &str,
    response: &Response,
    cost_time: i64,
) -> Result<(), AppError> {
    let db_conn = DatabaseManager::get_connection().await;
    let access_log_service = AccessLogService::new(db_conn.clone());

    // 获取用户信息
    let user_info = extract_user_from_request(response).or_else(|| {
        // 如果响应中没有，从请求中获取
        None
    });

    // 构建访问日志请求
    let log_request = crate::app::access_log::dto::CreateAccessLogRequest {
        user_id: user_info.and_then(|p| p.sub.parse::<i64>().ok()),
        user_name: user_info.map(|p| p.username.clone()),
        dept_id: None, // TODO: 从用户信息中获取
        dept_name: None, // TODO: 从用户信息中获取
        trace_id: trace_id.to_string(),
        parent_trace_id: None, // TODO: 从请求中获取
        method: method.as_str().to_string(),
        url: uri.to_string(),
        query_params: if !query.is_empty() { Some(query.to_string()) } else { None },
        request_body: None, // TODO: 从请求中获取
        status_code: response.status().as_u16(),
        response_body: None, // TODO: 从响应中获取
        client_ip: client_ip.to_string(),
        user_agent: if !user_agent.is_empty() { Some(user_agent.to_string()) } else { None },
        os: if !os.is_empty() { Some(os.to_string()) } else { None },
        browser: if !browser.is_empty() { Some(browser.to_string()) } else { None },
        device_type: if !device_type.is_empty() { Some(device_type.to_string()) } else { None },
        referer: if !referer.is_empty() { Some(referer.to_string()) } else { None },
        cost_time,
        is_error: response.status() >= StatusCode::BAD_REQUEST,
        error_msg: if response.status() >= StatusCode::BAD_REQUEST {
            Some(format!("HTTP error: {}", response.status()))
        } else {
            None
        },
    };

    // 记录访问日志
    let _ = access_log_service.create_access_log(&log_request).await;

    Ok(())
}

/// 检查是否为跳过路径
/// 返回是否跳过
fn is_skip_path(path: &str) -> bool {
    // 跳过不需要记录访问日志的路径
    let skip_paths = [
        "/",
        "/health",
        "/favicon.ico",
        "/api/v1/access-log/", // 访问日志管理相关API
        "/api/v1/opera-log/", // 操作日志管理相关API
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

/// 提取用户代理
/// 返回用户代理字符串
fn extract_user_agent(request: &Request) -> String {
    if let Some(user_agent) = request.headers().get("User-Agent") {
        if let Ok(user_agent_str) = user_agent.to_str() {
            return user_agent_str.to_string();
        }
    }
    "".to_string()
}

/// 提取访问来源
/// 返回访问来源
fn extract_referer(request: &Request) -> String {
    if let Some(referer) = request.headers().get("Referer") {
        if let Ok(referer_str) = referer.to_str() {
            return referer_str.to_string();
        }
    }
    "".to_string()
}

/// 解析用户代理
/// 返回(操作系统, 浏览器, 设备类型)元组
fn parse_user_agent(user_agent: &str) -> (String, String, String) {
    // TODO: 使用user-agents库解析用户代理
    // 这里可以集成user-agents crate来解析UA
    let os = detect_os(user_agent);
    let browser = detect_browser(user_agent);
    let device_type = detect_device_type(user_agent);

    (os, browser, device_type)
}

/// 检测操作系统
fn detect_os(user_agent: &str) -> String {
    if user_agent.contains("Windows") {
        "Windows".to_string()
    } else if user_agent.contains("Mac") {
        "macOS".to_string()
    } else if user_agent.contains("Linux") {
        "Linux".to_string()
    } else if user_agent.contains("Android") {
        "Android".to_string()
    } else if user_agent.contains("iOS") {
        "iOS".to_string()
    } else {
        "Unknown".to_string()
    }
}

/// 检测浏览器
fn detect_browser(user_agent: &str) -> String {
    if user_agent.contains("Chrome") {
        "Chrome".to_string()
    } else if user_agent.contains("Firefox") {
        "Firefox".to_string()
    } else if user_agent.contains("Safari") {
        "Safari".to_string()
    } else if user_agent.contains("Edge") {
        "Edge".to_string()
    } else if user_agent.contains("Opera") {
        "Opera".to_string()
    } else {
        "Unknown".to_string()
    }
}

/// 检测设备类型
fn detect_device_type(user_agent: &str) -> String {
    if user_agent.contains("Mobile") {
        "Mobile".to_string()
    } else if user_agent.contains("Tablet") {
        "Tablet".to_string()
    } else {
        "Desktop".to_string()
    }
}

/// 生成请求ID
/// 返回UUID字符串
fn generate_trace_id() -> String {
    uuid::Uuid::new_v4().to_string()
}
