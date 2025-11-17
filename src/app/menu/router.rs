/// 组装所有菜单相关的API路由

use axum::{routing::{get, post, put, delete}, Router};
use crate::app::menu::api::{
    get_menu, create_menu, update_menu, delete_menu,
    get_menu_tree, get_menu_list, get_sidebar_menus,
};

/// 组合菜单管理相关的路由
pub fn menu_routes() -> Router {
    Router::new()
        // 注意：此 Router 会被挂载到前缀 `/api/v1/sys/menus` 下
        .route("/", get(get_menu_tree))  // GET '' 返回菜单树
        .route("/", post(create_menu))
        .route("/{id}", get(get_menu))
        .route("/{id}", put(update_menu))
        .route("/{id}", delete(delete_menu))
        .route("/tree", get(get_menu_tree))  // 保留此路由作为别名
        .route("/list", get(get_menu_list))
        .route("/sidebar", get(get_sidebar_menus))
}
