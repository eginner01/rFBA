/// 菜单管理相关API

use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use crate::common::response::ApiResult;
use crate::app::menu::dto::{
    CreateMenuRequest, UpdateMenuRequest, MenuPaginationQuery,
};
use crate::app::menu::service::MenuService;
use crate::database::DatabaseManager;
use crate::common::response::api_response;

/// 获取菜单列表（分页）
/// GET /api/v1/menus
pub async fn get_menus(
    Query(query): Query<MenuPaginationQuery>,
) -> ApiResult<impl IntoResponse> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建菜单服务
    let menu_service = MenuService::new(db_conn.clone());

    // 查询菜单列表
    let result = menu_service.get_menus_paginated(&query).await?;

    Ok((StatusCode::OK, Json(api_response(result))))
}

/// 获取菜单详情
/// GET /api/v1/menus/{id}
pub async fn get_menu(
    Path(id): Path<i64>,
) -> ApiResult<impl IntoResponse> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建菜单服务
    let menu_service = MenuService::new(db_conn.clone());

    // 查询菜单详情
    let result = menu_service.get_menu_detail(id).await?;

    Ok((StatusCode::OK, Json(api_response(result))))
}

/// 创建菜单
/// POST /api/v1/menus
pub async fn create_menu(
    Json(request): Json<CreateMenuRequest>,
) -> ApiResult<impl IntoResponse> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建菜单服务
    let menu_service = MenuService::new(db_conn.clone());

    // 创建菜单
    let result = menu_service.create_menu(&request).await?;

    Ok((StatusCode::CREATED, Json(api_response(result))))
}

/// 更新菜单
/// PUT /api/v1/menus/{id}
pub async fn update_menu(
    Path(id): Path<i64>,
    Json(request): Json<UpdateMenuRequest>,
) -> ApiResult<impl IntoResponse> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建菜单服务
    let menu_service = MenuService::new(db_conn.clone());

    // 更新菜单
    menu_service.update_menu(id, &request).await?;

    Ok((StatusCode::OK, Json(api_response("菜单更新成功".to_string()))))
}

/// 删除菜单
/// DELETE /api/v1/menus/{id}
pub async fn delete_menu(
    Path(id): Path<i64>,
) -> ApiResult<impl IntoResponse> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建菜单服务
    let menu_service = MenuService::new(db_conn.clone());

    // 删除菜单
    menu_service.delete_menu(id).await?;

    Ok((StatusCode::OK, Json(api_response("菜单删除成功".to_string()))))
}

/// 获取菜单树
/// GET /api/v1/menus 或 GET /api/v1/menus/tree
pub async fn get_menu_tree(
    Query(_query): Query<MenuPaginationQuery>,
) -> ApiResult<impl IntoResponse> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建菜单服务
    let menu_service = MenuService::new(db_conn.clone());

    // 获取菜单树（暂不过滤，返回所有菜单）
    let result = menu_service.get_menu_tree().await?;

    Ok((StatusCode::OK, Json(api_response(result))))
}

/// 获取菜单列表（平铺）
/// GET /api/v1/menus/list
pub async fn get_menu_list() -> ApiResult<impl IntoResponse> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建菜单服务
    let menu_service = MenuService::new(db_conn.clone());

    // 获取菜单列表
    let result = menu_service.get_menu_list().await?;

    Ok((StatusCode::OK, Json(api_response(result))))
}

/// 获取侧边栏菜单
/// GET /api/v1/menus/sidebar
pub async fn get_sidebar_menus(
    _auth_context: axum::extract::Extension<crate::middleware::jwt_auth_middleware::AuthContext>,
) -> ApiResult<impl IntoResponse> {
    // 获取数据库连接
    let db_conn = DatabaseManager::get_connection().await;

    // 创建菜单服务
    let menu_service = MenuService::new(db_conn.clone());

    // 获取侧边栏菜单（只获取菜单类型的菜单）
    let result = menu_service.get_sidebar_menus().await?;

    Ok((StatusCode::OK, Json(api_response(result))))
}
