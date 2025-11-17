use axum::Router;
use crate::common::response::api_response;

pub fn notice_routes() -> Router {
    Router::new()
        .route("/", axum::routing::get(|| async { api_response("Notice List - TODO") }))
}
