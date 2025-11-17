use axum::Router; use crate::common::response::api_response; pub fn schedule_job_routes() -> Router { Router::new().route("/schedule-jobs", axum::routing::get(|| async { api_response("任务列表") })) }
