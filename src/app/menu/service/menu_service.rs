use tracing::{info, error};

/// 菜单管理服务实现
/// 提供菜单CRUD、菜单树、角色-菜单关联等功能

use crate::app::menu::dto::{
    CreateMenuRequest, CreateMenuResponse, UpdateMenuRequest,
    MenuDetailResponse, MenuListItem, MenuPaginationQuery,
    MenuPaginationResponse, MenuTreeNode,
};
use crate::common::exception::{AppError, ErrorCode};
use crate::database::entity::menu;
use crate::database::menu_repo::MenuRepository as MenuRepo;
use sea_orm::{DatabaseConnection, ActiveValue};

/// 菜单管理服务
pub struct MenuService {
    db: DatabaseConnection,
}

#[derive(serde::Serialize, Clone, Debug)]
pub struct VbenMenuMeta {
    title: String,
    icon: Option<String>,
    #[serde(rename = "iframeSrc")]
    iframe_src: String,
    link: String,
    #[serde(rename = "keepAlive")]
    keep_alive: bool,
    #[serde(rename = "hideInMenu")]
    hide_in_menu: bool,
    #[serde(rename = "menuVisibleWithForbidden")]
    menu_visible_with_forbidden: bool,
}

#[derive(serde::Serialize, Clone, Debug)]
pub struct VbenMenuNode {
    id: i64,
    parent_id: Option<i64>,
    name: String,
    path: Option<String>,
    component: Option<String>,
    meta: VbenMenuMeta,
    children: Vec<VbenMenuNode>,
}

impl MenuService {
    /// 创建新的菜单服务
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// 创建菜单
    pub async fn create_menu(
        &self,
        request: &CreateMenuRequest,
    ) -> Result<CreateMenuResponse, AppError> {
        info!("Creating menu: {}", request.name);
        // 构建菜单数据
        let menu_model = menu::ActiveModel {
            id: ActiveValue::NotSet,
            title: ActiveValue::Set(request.title.clone()),
            name: ActiveValue::Set(request.name.clone()),
            parent_id: ActiveValue::Set(request.parent_id),
            sort: ActiveValue::Set(request.sort.unwrap_or(0)),
            path: ActiveValue::Set(request.path.clone()),
            component: ActiveValue::Set(request.component.clone()),
            menu_type: ActiveValue::Set(request.menu_type),
            perms: ActiveValue::Set(request.perms.clone()),
            icon: ActiveValue::Set(request.icon.clone()),
            status: ActiveValue::Set(request.status.unwrap_or(1)),
            display: ActiveValue::Set(request.display.unwrap_or(true)),
            cache: ActiveValue::Set(request.cache.unwrap_or(false)),
            link: ActiveValue::Set(request.link.clone()),
            remark: ActiveValue::Set(request.remark.clone()),
            created_time: ActiveValue::NotSet,
            updated_time: ActiveValue::NotSet,
        };

        // 保存到数据库
        let saved_menu = MenuRepo::create(menu_model, &self.db)
            .await
            .map_err(|e| {
                error!("Failed to create menu: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, "Failed to create menu")
            })?;

        info!("Menu created successfully: {}", saved_menu.id);

        Ok(CreateMenuResponse {
            id: saved_menu.id,
            title: saved_menu.title,
            name: saved_menu.name,
            menu_type: saved_menu.menu_type,
            status: saved_menu.status,
            created_time: saved_menu.created_time.and_utc(),
        })
    }

    /// 更新菜单
    pub async fn update_menu(
        &self,
        menu_id: i64,
        request: &UpdateMenuRequest,
    ) -> Result<(), AppError> {
        info!("Updating menu: {}", menu_id);

        // 检查菜单是否存在
        let _ = MenuRepo::find_by_id(menu_id, &self.db)
            .await
            .map_err(|_| AppError::with_message(ErrorCode::NotFound, "Menu not found"))?;

        // 构建更新数据
        let mut update_data = menu::ActiveModel::default();

        if let Some(title) = &request.title {
            update_data.title = ActiveValue::Set(title.clone());
        }

        if let Some(name) = &request.name {
            update_data.name = ActiveValue::Set(name.clone());
        }

        if let Some(parent_id) = request.parent_id {
            // 更新父菜单ID（不再使用 level 字段）
            update_data.parent_id = ActiveValue::Set(Some(parent_id));
        }


        if let Some(sort) = request.sort {
            update_data.sort = ActiveValue::Set(sort);
        }

        if let Some(path) = &request.path {
            update_data.path = ActiveValue::Set(Some(path.clone()));
        }

        if let Some(component) = &request.component {
            update_data.component = ActiveValue::Set(Some(component.clone()));
        }

        if let Some(menu_type) = request.menu_type {
            update_data.menu_type = ActiveValue::Set(menu_type);
        }

        if let Some(perms) = &request.perms {
            update_data.perms = ActiveValue::Set(Some(perms.clone()));
        }

        if let Some(icon) = &request.icon {
            update_data.icon = ActiveValue::Set(Some(icon.clone()));
        }

        if let Some(display) = request.display {
            update_data.display = ActiveValue::Set(display);
        }

        if let Some(cache) = request.cache {
            update_data.cache = ActiveValue::Set(cache);
        }

        if let Some(status) = request.status {
            update_data.status = ActiveValue::Set(status);
        }

        if let Some(link) = &request.link {
            update_data.link = ActiveValue::Set(Some(link.clone()));
        }

        if let Some(remark) = &request.remark {
            update_data.remark = ActiveValue::Set(Some(remark.clone()));
        }

        // 执行更新
        MenuRepo::update(menu_id, update_data, &self.db)
            .await
            .map_err(|e| {
                error!("Failed to update menu: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, "Failed to update menu")
            })?;

        info!("Menu updated successfully: {}", menu_id);

        Ok(())
    }

    /// 删除菜单
    pub async fn delete_menu(&self, menu_id: i64) -> Result<(), AppError> {
        info!("Deleting menu: {}", menu_id);

        // 检查菜单是否存在
        let _ = MenuRepo::find_by_id(menu_id, &self.db)
            .await
            .map_err(|_| AppError::with_message(ErrorCode::NotFound, "Menu not found"))?;

        // 检查是否有子菜单
        let child_count = MenuRepo::count_children(menu_id, &self.db)
            .await
            .map_err(|e| {
                error!("Failed to count child menus: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, "Failed to count child menus")
            })?;

        if child_count > 0 {
            return Err(AppError::with_message(
                ErrorCode::BadRequest,
                "Cannot delete menu with child menus"
            ));
        }

        // 执行软删除
        MenuRepo::delete(menu_id, &self.db)
            .await
            .map_err(|e| {
                error!("Failed to delete menu: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, "Failed to delete menu")
            })?;

        info!("Menu deleted successfully: {}", menu_id);

        Ok(())
    }

    /// 获取菜单详情
    pub async fn get_menu_detail(
        &self,
        menu_id: i64,
    ) -> Result<MenuDetailResponse, AppError> {
        let menu = MenuRepo::find_by_id(menu_id, &self.db)
            .await
            .map_err(|e| {
                error!("Failed to find menu: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, "Failed to find menu")
            })?;

        Ok(MenuDetailResponse {
            id: menu.id,
            title: menu.title,
            name: menu.name,
            parent_id: menu.parent_id,
            sort: menu.sort,
            path: menu.path,
            component: menu.component,
            menu_type: menu.menu_type,
            perms: menu.perms,
            icon: menu.icon,
            display: menu.display,
            cache: menu.cache,
            status: menu.status,
            link: menu.link,
            remark: menu.remark,
            created_time: menu.created_time.and_utc(),
            updated_time: menu.updated_time.map(|t| t.and_utc()),
        })
    }

    /// 分页查询菜单
    pub async fn get_menus_paginated(
        &self,
        query: &MenuPaginationQuery,
    ) -> Result<MenuPaginationResponse, AppError> {
        let (menus, total) = MenuRepo::find_with_pagination(&MenuRepo, query, &self.db)
            .await
            .map_err(|e| {
                error!("Failed to query menus: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, "Failed to query menus")
            })?;

        let size = query.size.unwrap_or(20);
        let page = query.page.unwrap_or(1);

        let list = menus.into_iter()
            .map(|m| MenuListItem {
                id: m.id,
                title: m.title,
                name: m.name,
                parent_id: m.parent_id,
                sort: m.sort,
                path: m.path,
                component: m.component,
                menu_type: m.menu_type,
                perms: m.perms,
                icon: m.icon,
                display: m.display,
                cache: m.cache,
                status: m.status,
                link: m.link,
                remark: m.remark,
                created_time: m.created_time.and_utc(),
            })
            .collect();

        Ok(MenuPaginationResponse {
            list,
            total,
            page,
            size,
            pages: (total as f64 / size as f64).ceil() as usize,
        })
    }

    /// 获取菜单树
    pub async fn get_menu_tree(&self) -> Result<Vec<MenuTreeNode>, AppError> {
        let menus = MenuRepo::find_all(&MenuRepo, &self.db)
            .await
            .map_err(|e| {
                error!("Failed to query menus: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, "Failed to query menus")
            })?;

        let mut menu_map: std::collections::HashMap<i64, MenuTreeNode> = std::collections::HashMap::new();
        let mut root_nodes: Vec<MenuTreeNode> = Vec::new();

        // 先构建所有节点
        for menu in &menus {
            let node = MenuTreeNode {
                id: menu.id,
                title: menu.title.clone(),
                name: menu.name.clone(),
                parent_id: menu.parent_id,
                sort: menu.sort,
                path: menu.path.clone(),
                component: menu.component.clone(),
                menu_type: menu.menu_type,
                perms: menu.perms.clone(),
                icon: menu.icon.clone(),
                display: menu.display,
                cache: menu.cache,
                status: menu.status,
                link: menu.link.clone(),
                remark: menu.remark.clone(),
                children: Vec::new(),
            };
            menu_map.insert(menu.id, node);
        }

        // 构建树形结构
        for menu in &menus {
            if let Some(node) = menu_map.remove(&menu.id) {
                if let Some(parent_id) = menu.parent_id {
                    if let Some(parent) = menu_map.get_mut(&parent_id) {
                        parent.children.push(node);
                    } else if let Some(index) = root_nodes.iter().position(|n| n.id == parent_id) {
                        root_nodes[index].children.push(node);
                    }
                } else {
                    root_nodes.push(node);
                }
            }
        }

        Ok(root_nodes)
    }

    /// 获取菜单列表（平铺）
    pub async fn get_menu_list(&self) -> Result<Vec<MenuListItem>, AppError> {
        let menus = MenuRepo::find_all(&MenuRepo, &self.db)
            .await
            .map_err(|e| {
                error!("Failed to query menus: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, "Failed to query menus")
            })?;

        let list = menus.into_iter()
            .map(|m| MenuListItem {
                id: m.id,
                title: m.title,
                name: m.name,
                parent_id: m.parent_id,
                sort: m.sort,
                path: m.path,
                component: m.component,
                menu_type: m.menu_type,
                perms: m.perms,
                icon: m.icon,
                display: m.display,
                cache: m.cache,
                status: m.status,
                link: m.link,
                remark: m.remark,
                created_time: m.created_time.and_utc(),
            })
            .collect();

        Ok(list)
    }

    /// 检查菜单是否可以删除
    pub async fn can_delete_menu(&self, menu_id: i64) -> Result<bool, AppError> {
        let child_count = MenuRepo::count_children(menu_id, &self.db)
            .await
            .map_err(|e| {
                error!("Failed to count child menus: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, "Failed to count child menus")
            })?;

        Ok(child_count == 0)
    }

    /// 获取侧边栏菜单
    pub async fn get_sidebar_menus(&self) -> Result<Vec<VbenMenuNode>, AppError> {
        let menus = MenuRepo::find_all(&MenuRepo, &self.db)
            .await
            .map_err(|e| {
                error!("Failed to query sidebar menus: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, "Failed to query sidebar menus")
            })?;

        let mut filtered_menus: Vec<_> = menus
            .into_iter()
            .filter(|m| matches!(m.menu_type, 0 | 1 | 3 | 4) && m.display)
            .collect();

        filtered_menus.sort_by(|a, b| a.sort.cmp(&b.sort));

        let all_nodes: Vec<VbenMenuNode> = filtered_menus
            .into_iter()
            .map(|m| {
                let iframe_src = if m.menu_type == 3 {
                    m.link.clone().unwrap_or_default()
                } else {
                    String::new()
                };

                let link = if m.menu_type == 4 {
                    m.link.clone().unwrap_or_default()
                } else {
                    String::new()
                };

                let meta = VbenMenuMeta {
                    title: m.title.clone(),
                    icon: m.icon.clone(),
                    iframe_src,
                    link,
                    keep_alive: m.cache,
                    hide_in_menu: !m.display,
                    menu_visible_with_forbidden: m.status == 0,
                };

                VbenMenuNode {
                    id: m.id,
                    parent_id: m.parent_id,
                    name: m.name.clone(),
                    path: m.path.clone(),
                    component: m.component.clone(),
                    meta,
                    children: Vec::new(),
                }
            })
            .collect();

        // 使用稳定的遍历算法构建树
        let tree = Self::build_tree_stable(all_nodes);

        Ok(tree)
    }

    /// 使用稳定的遍历算法构建树形结构
    fn build_tree_stable(nodes: Vec<VbenMenuNode>) -> Vec<VbenMenuNode> {
        use std::collections::HashMap;
        
        // 创建 id -> node 的映射
        let mut node_map: HashMap<i64, VbenMenuNode> = HashMap::new();
        for node in nodes {
            node_map.insert(node.id, node);
        }

        let mut tree: Vec<VbenMenuNode> = Vec::new();
        
        let mut ids: Vec<i64> = node_map.keys().copied().collect();
        ids.sort();

        for id in ids {
            if let Some(node) = node_map.remove(&id) {
                match node.parent_id {
                    None => {
        
                        tree.push(node);
                    }
                    Some(parent_id) => {        
                        if let Some(parent) = node_map.get_mut(&parent_id) {
                  
                            parent.children.push(node);
                        } else if !Self::add_to_parent(&mut tree, parent_id, node.clone()) {
                            // 找不到父节点，作为根节点
                            tree.push(node);
                        }
                    }
                }
            }
        }

        tree
    }

    /// 递归查找父节点并添加子节点
    fn add_to_parent(nodes: &mut Vec<VbenMenuNode>, parent_id: i64, child: VbenMenuNode) -> bool {
        for node in nodes.iter_mut() {
            if node.id == parent_id {
                node.children.push(child);
                return true;
            }
            // 递归查找子节点
            if Self::add_to_parent(&mut node.children, parent_id, child.clone()) {
                return true;
            }
        }
        false
    }
}
