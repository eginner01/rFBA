use axum::{http::StatusCode, response::IntoResponse, Json};
use crate::common::response::api_response;
use crate::common::exception::AppError;
use serde::Serialize;

#[derive(Serialize)]
pub struct MonitorItem {
    id: i64,
    name: String,
    status: String,
    value: f64,
}

pub async fn get_monitors() -> Result<impl IntoResponse, AppError> {
    let monitors = vec![MonitorItem {
        id: 1,
        name: "CPU使用率".to_string(),
        status: "正常".to_string(),
        value: 45.5,
    }];
    Ok((StatusCode::OK, Json(api_response(monitors))))
}
