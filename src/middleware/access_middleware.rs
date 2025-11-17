/// 中间件占位符
/// TODO: 实现具体功能
use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
};

pub async fn middleware(request: Request, next: Next) -> Response {
    next.run(request).await
}
