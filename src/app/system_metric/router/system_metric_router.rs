use axum::Router; use crate::common::response::api_response; pub fn system_metric_routes() -> Router { Router::new().route("/system-metrics", axum::routing::get(|| async { api_response("指标列表") })) }
