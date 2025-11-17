use axum::{routing::{get, post, put, delete}, Router};
use crate::app::dict_data::api::{
    get_all_dict_datas, get_dict_data, get_dict_data_by_type_code,
    get_dict_datas_paginated, create_dict_data, update_dict_data, delete_dict_datas,
};

pub fn dict_data_routes() -> Router {
    Router::new()
        // 获取所有字典数据
        .route("/all", get(get_all_dict_datas))
        // 分页获取字典数据列表
        .route("/", get(get_dict_datas_paginated))
        // 根据类型编码获取字典数据
        .route("/type-codes/{code}", get(get_dict_data_by_type_code))
        // 创建字典数据
        .route("/", post(create_dict_data))
        // 获取字典数据详情
        .route("/{dict_code}", get(get_dict_data))
        // 更新字典数据
        .route("/{dict_code}", put(update_dict_data))
        // 批量删除字典数据
        .route("/", delete(delete_dict_datas))
}
