use axum::{routing::{get, post, delete}, Router};
use crate::app::login_log::api::{
    get_login_logs_paginated, get_login_log_detail,
    create_login_log, create_logout_log,
    delete_login_log, delete_login_logs_batch,
    clear_login_logs, get_login_log_statistics,
};

pub fn login_log_routes() -> Router {
    Router::new()
        .route("/", get(get_login_logs_paginated))
        .route("/{id}", get(get_login_log_detail))
        .route("/statistics", get(get_login_log_statistics))
        .route("/", post(create_login_log))
        .route("/logout", post(create_logout_log))
        .route("/{id}", delete(delete_login_log))
        .route("/batch", delete(delete_login_logs_batch))
        .route("/clear", delete(clear_login_logs))
}
