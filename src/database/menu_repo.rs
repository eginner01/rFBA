/// 菜单数据访问层
/// 提供菜单相关的数据库操作

use crate::database::entity::menu;
use crate::database::DatabaseConnection;
use sea_orm::{ActiveValue, ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter, QuerySelect, QueryOrder};

/// 菜单仓库
pub struct MenuRepository;

impl MenuRepository {
    /// 根据ID查找菜单
    /// 返回菜单模型
    pub async fn find_by_id(
        id: i64,
        db: &DatabaseConnection,
    ) -> Result<menu::Model, DbErr> {
        let menu = menu::Entity::find_by_id(id)
            .one(db)
            .await?;

        menu.ok_or(DbErr::RecordNotFound("Menu not found".to_string()))
    }

    /// 创建菜单
    /// 返回创建的菜单模型
    pub async fn create(
        menu: menu::ActiveModel,
        db: &DatabaseConnection,
    ) -> Result<menu::Model, DbErr> {
        let insert_result = menu::Entity::insert(menu).exec(db).await?;
        let menu_id = insert_result.last_insert_id;

        // 重新查询获取完整的菜单模型
        menu::Entity::find_by_id(menu_id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("Menu not found after creation".to_string()))
    }

    /// 更新菜单
    /// 返回更新后的菜单模型
    pub async fn update(
        id: i64,
        mut menu: menu::ActiveModel,
        db: &DatabaseConnection,
    ) -> Result<menu::Model, DbErr> {
        menu.id = ActiveValue::Set(id);
        let updated_menu = menu::Entity::update(menu).exec(db).await?;

        Ok(updated_menu)
    }

    /// 删除菜单（软删除）
    /// 返回操作结果
    pub async fn delete(
        id: i64,
        db: &DatabaseConnection,
    ) -> Result<(), DbErr> {
        let res = menu::Entity::delete_by_id(id)
            .exec(db)
            .await?;

        if res.rows_affected == 0 {
            return Err(DbErr::RecordNotFound("Menu not found".to_string()));
        }

        Ok(())
    }

    /// 检查菜单名在同级下是否已存在
    /// 返回是否存在
    pub async fn exists_by_name(
        name: &str,
        parent_id: Option<i64>,
        db: &DatabaseConnection,
    ) -> Result<bool, DbErr> {
        let mut select = menu::Entity::find()
            .filter(menu::Column::Name.eq(name));

        if let Some(parent_id) = parent_id {
            select = select.filter(menu::Column::ParentId.eq(parent_id));
        } else {
            select = select.filter(menu::Column::ParentId.is_null());
        }

        let count = select.count(db).await?;

        Ok(count > 0)
    }

    /// 分页查询菜单
    /// 返回菜单列表和总数
    pub async fn find_with_pagination(
        _repo: &MenuRepository,
        query: &crate::app::menu::dto::MenuPaginationQuery,
        db: &DatabaseConnection,
    ) -> Result<(Vec<menu::Model>, usize), DbErr> {
        let mut select = menu::Entity::find();

        // 菜单名搜索
        if let Some(name) = &query.name {
            select = select.filter(menu::Column::Name.like(format!("%{}%", name)));
        }

        // 菜单类型过滤
        if let Some(menu_type) = query.menu_type {
            select = select.filter(menu::Column::MenuType.eq(menu_type));
        }

        // 显示状态过滤
        if let Some(status) = query.status {
            let display = status == 1;
            select = select.filter(menu::Column::Display.eq(display));
        }

        // 排序
        select = select.order_by(menu::Column::Sort, sea_orm::Order::Asc);

        // 分页
        let size = query.size.unwrap_or(20);
        let page = query.page.unwrap_or(1);
        let offset = (page - 1) * size;

        // 查询总数
        let mut total_select = menu::Entity::find();

        if let Some(name) = &query.name {
            total_select = total_select.filter(menu::Column::Name.like(format!("%{}%", name)));
        }
        if let Some(menu_type) = query.menu_type {
            total_select = total_select.filter(menu::Column::MenuType.eq(menu_type));
        }
        if let Some(status) = query.status {
            let display = status == 1;
            total_select = total_select.filter(menu::Column::Display.eq(display));
        }

        let total = total_select.count(db).await?;

        // 查询列表
        let list = select
            .offset(offset as u64)
            .limit(size as u64)
            .all(db)
            .await?;

        Ok((list, total as usize))
    }

    /// 获取所有菜单（不分页，用于树形结构）
    /// 返回菜单列表
    pub async fn find_all(
        _repo: &MenuRepository,
        db: &DatabaseConnection,
    ) -> Result<Vec<menu::Model>, DbErr> {
        let menus = menu::Entity::find()
            .order_by(menu::Column::Sort, sea_orm::Order::Asc)
            .all(db)
            .await?;

        Ok(menus)
    }

    /// 检查菜单是否有子菜单
    /// 返回子菜单数量
    pub async fn count_children(
        id: i64,
        db: &DatabaseConnection,
    ) -> Result<u64, DbErr> {
        let count = menu::Entity::find()
            .filter(menu::Column::ParentId.eq(id))
            .count(db)
            .await?;

        Ok(count)
    }
}

// 为MenuRepository创建别名，以便在Service层中使用
pub use MenuRepository as MenuCrud;
