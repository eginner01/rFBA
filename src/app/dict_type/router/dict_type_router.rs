/// 字典类型路由配置
/// 与Python版本对齐

use axum::{routing::{get, post, put, delete}, Router};

pub fn create_dict_type_router() -> Router {
    Router::new()
        // GET /all - 获取所有字典类型
        .route("/all", get(crate::app::dict_type::api::get_dict_type_options))
        // GET /{id} - 获取字典类型详情
        .route("/{id}", get(crate::app::dict_type::api::get_dict_type))
        // GET / - 分页获取字典类型列表
        .route("/", get(crate::app::dict_type::api::get_dict_types))
        // POST / - 创建字典类型
        .route("/", post(crate::app::dict_type::api::create_dict_type))
        // PUT /{id} - 更新字典类型
        .route("/{id}", put(crate::app::dict_type::api::update_dict_type))
        // DELETE / - 批量删除字典类型
        .route("/", delete(crate::app::dict_type::api::batch_delete_dict_types))
}

/// 获取字典类型路由列表（用于文档）
pub fn get_dict_type_routes() -> Vec<(String, String)> {
    vec![
        ("GET".to_string(), "/api/v1/sys/dict-types/all".to_string()),
        ("GET".to_string(), "/api/v1/sys/dict-types/{id}".to_string()),
        ("GET".to_string(), "/api/v1/sys/dict-types".to_string()),
        ("POST".to_string(), "/api/v1/sys/dict-types".to_string()),
        ("PUT".to_string(), "/api/v1/sys/dict-types/{id}".to_string()),
        ("DELETE".to_string(), "/api/v1/sys/dict-types".to_string()),
    ]
}
