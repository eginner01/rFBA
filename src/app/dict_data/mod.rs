// pub mod api; // TODO: Replace with real implementation
pub mod router;

// Stub implementations - TODO: Replace with real implementation
pub mod api {
    use axum::{
        http::StatusCode,
        response::IntoResponse,
        Json,
    };
    use crate::common::response::api_response;

    pub async fn get_all_dict_datas() -> Result<impl IntoResponse, crate::common::exception::AppError> {
        Ok((StatusCode::OK, Json(api_response(Vec::<serde_json::Value>::new()))))
    }

    pub async fn get_dict_data() -> Result<impl IntoResponse, crate::common::exception::AppError> {
        Ok((StatusCode::OK, Json(api_response(serde_json::Value::Null))))
    }

    pub async fn get_dict_data_by_type_code() -> Result<impl IntoResponse, crate::common::exception::AppError> {
        Ok((StatusCode::OK, Json(api_response(Vec::<serde_json::Value>::new()))))
    }

    pub async fn get_dict_datas_paginated() -> Result<impl IntoResponse, crate::common::exception::AppError> {
        let data = serde_json::json!({
            "list": Vec::<serde_json::Value>::new(),
            "total": 0,
            "page": 1,
            "size": 20,
            "pages": 0
        });
        Ok((StatusCode::OK, Json(api_response(data))))
    }

    pub async fn create_dict_data() -> Result<impl IntoResponse, crate::common::exception::AppError> {
        Ok((StatusCode::CREATED, Json(api_response("创建成功".to_string()))))
    }

    pub async fn update_dict_data() -> Result<impl IntoResponse, crate::common::exception::AppError> {
        Ok((StatusCode::OK, Json(api_response("更新成功".to_string()))))
    }

    pub async fn delete_dict_datas() -> Result<impl IntoResponse, crate::common::exception::AppError> {
        Ok((StatusCode::OK, Json(api_response("删除成功".to_string()))))
    }
}

pub use router::*;
