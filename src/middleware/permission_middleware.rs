/// 权限相关中间件
/// 提供从请求中提取用户信息等功能

use axum::extract::Request;
use crate::utils::encrypt::JwtPayload;

/// 从请求中提取用户信息
/// 返回JWT负载信息
pub fn extract_user_from_request(request: &Request) -> Option<&JwtPayload> {
    request.extensions().get::<JwtPayload>()
}
