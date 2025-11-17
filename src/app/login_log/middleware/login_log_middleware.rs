/// 登录日志中间件
/// 自动记录用户登录/注销日志

use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use std::time::Instant;
use crate::app::login_log::service::LoginLogService;
use crate::common::exception::AppError;
use crate::database::DatabaseManager;

/// 登录日志中间件处理器
/// 返回HTTP响应
pub async fn login_log_middleware(
    request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let start_time = Instant::now();
    let _method = request.method().clone();
    let uri = request.uri().path().to_string();

    // 跳过不需要记录登录日志的路径
    if is_skip_path(&uri) {
        return Ok(next.run(request).await);
    }

    // 检查是否是登录请求
    if uri.contains("/api/v1/auth/login") {
        return handle_login(request, next, start_time).await;
    }

    // 检查是否是注销请求
    if uri.contains("/api/v1/auth/logout") {
        return handle_logout(request, next, start_time).await;
    }

    // 其他请求直接跳过
    Ok(next.run(request).await)
}

/// 处理登录请求
/// 返回HTTP响应
async fn handle_login(
    request: Request,
    next: Next,
    start_time: Instant,
) -> Result<Response, AppError> {
    // 记录登录日志
    let login_time = start_time.elapsed().as_millis() as i64;
    let client_ip = extract_client_ip(&request);
    let (os, browser, device_type) = parse_client_info(&request);
    // 在移动 request 之前先从请求中提取用户名
    let username = extract_username_from_request(&request);

    // 执行登录请求
    let response = next.run(request).await;

    // 解析登录结果
    let (status, msg) = parse_login_result(&response);

    // 记录登录日志
    if let Some(username) = username {
        let _ = record_login_log(
            &username,
            &client_ip,
            &os,
            &browser,
            &device_type,
            status,
            msg.as_deref(),
            login_time,
        ).await;
    }

    Ok(response)
}

/// 处理注销请求
/// 返回HTTP响应
async fn handle_logout(
    request: Request,
    next: Next,
    start_time: Instant,
) -> Result<Response, AppError> {
    // 记录注销日志
    let logout_time = start_time.elapsed().as_millis() as i64;
    let client_ip = extract_client_ip(&request);

    // 执行注销请求
    let response = next.run(request).await;

    // 解析注销结果
    let username = extract_username_from_response(&response);

    // 记录注销日志
    if let Some(username) = username {
        let _ = record_logout_log(
            &username,
            &client_ip,
            logout_time,
        ).await;
    }

    Ok(response)
}

/// 记录登录日志
/// 返回操作结果
async fn record_login_log(
    username: &str,
    client_ip: &str,
    os: &str,
    browser: &str,
    device_type: &str,
    status: i32,
    msg: Option<&str>,
    login_time: i64,
) -> Result<(), AppError> {
    let db_conn = DatabaseManager::get_connection().await;
    let login_log_service = LoginLogService::new(db_conn.clone());

    // 获取登录地点
    let login_location = get_login_location(client_ip);

    // 构建登录日志请求
    let log_request = crate::app::login_log::dto::CreateLoginLogRequest {
        user_id: None, // TODO: 从用户信息中获取
        username: username.to_string(),
        dept_id: None, // TODO: 从用户信息中获取
        dept_name: None, // TODO: 从用户信息中获取
        ipaddr: client_ip.to_string(),
        login_location: Some(login_location),
        browser: if !browser.is_empty() { Some(browser.to_string()) } else { None },
        os: if !os.is_empty() { Some(os.to_string()) } else { None },
        dev_type: if !device_type.is_empty() { Some(device_type.to_string()) } else { None },
        status,
        msg: msg.map(|m| m.to_string()),
        login_time: Some(login_time),
    };

    // 记录登录日志
    let _ = login_log_service.create_login_log(&log_request).await;

    Ok(())
}

/// 记录注销日志
/// 返回操作结果
async fn record_logout_log(
    username: &str,
    client_ip: &str,
    logout_time: i64,
) -> Result<(), AppError> {
    let db_conn = DatabaseManager::get_connection().await;
    let login_log_service = LoginLogService::new(db_conn.clone());

    // 获取登录地点
    let login_location = get_login_location(client_ip);

    // 构建注销日志请求
    let log_request = crate::app::login_log::dto::CreateLogoutLogRequest {
        user_id: None, // TODO: 从用户信息中获取
        username: username.to_string(),
        login_id: None, // TODO: 从会话中获取
        logout_time: Some(logout_time),
    };

    // 记录注销日志
    let _ = login_log_service.create_logout_log(&log_request).await;

    Ok(())
}

/// 检查是否为跳过路径
/// 返回是否跳过
fn is_skip_path(path: &str) -> bool {
    // 跳过不需要记录登录日志的路径
    let skip_paths = [
        "/api/v1/login-log/", // 登录日志管理相关API
    ];

    skip_paths.iter().any(|p| path.starts_with(p))
}

/// 提取客户端IP
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

/// 解析客户端信息
fn parse_client_info(request: &Request) -> (String, String, String) {
    let user_agent = extract_user_agent(request);
    let os = detect_os(&user_agent);
    let browser = detect_browser(&user_agent);
    let device_type = detect_device_type(&user_agent);

    (os, browser, device_type)
}

/// 提取用户代理
fn extract_user_agent(request: &Request) -> String {
    if let Some(user_agent) = request.headers().get("User-Agent") {
        if let Ok(user_agent_str) = user_agent.to_str() {
            return user_agent_str.to_string();
        }
    }
    "".to_string()
}

/// 解析登录结果
fn parse_login_result(response: &Response) -> (i32, Option<String>) {
    if response.status() == StatusCode::OK {
        (1, Some("登录成功".to_string()))
    } else {
        (0, Some("登录失败".to_string()))
    }
}

/// 从请求中提取用户名（暂未实现）
fn extract_username_from_request(_request: &Request) -> Option<String> {
    // TODO: 根据需要从请求中提取用户名
    None
}

/// 从响应中提取用户名（暂未实现）
fn extract_username_from_response(_response: &Response) -> Option<String> {
    // TODO: 从响应中提取用户名
    None
}

/// 获取登录地点
fn get_login_location(_ip: &str) -> String {
    // TODO: 根据IP地址获取地理位置
    // 这里可以集成第三方IP归属地查询服务
    "本地".to_string()
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
