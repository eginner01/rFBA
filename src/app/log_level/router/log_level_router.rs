use axum::Router; use crate::common::response::api_response; pub fn log_level_routes() -> Router { Router::new().route("/log-levels", axum::routing::get(|| async { api_response("日志级别列表") })) }
