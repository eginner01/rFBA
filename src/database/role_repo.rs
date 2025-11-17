/// 角色数据访问层
/// 提供角色相关的数据库操作

use crate::database::entity::role;
use crate::database::DatabaseConnection;
use sea_orm::{ActiveValue, ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter, QuerySelect, QueryOrder};

/// 角色仓库
pub struct RoleRepository;

impl RoleRepository {
    /// 根据ID查找角色
    /// 返回角色模型
    pub async fn find_by_id(
        id: i64,
        db: &DatabaseConnection,
    ) -> Result<role::Model, DbErr> {
        let role = role::Entity::find_by_id(id)
            .one(db)
            .await?;

        role.ok_or(DbErr::RecordNotFound("Role not found".to_string()))
    }

    /// 根据角色编码查找角色（数据库表中没有code字段，此方法已禁用）
    /// 返回角色模型
    pub async fn find_by_code(
        _code: &str,
        _db: &DatabaseConnection,
    ) -> Result<role::Model, DbErr> {
        // 数据库表中没有code字段
        Err(DbErr::RecordNotFound("Code field not exist in table".to_string()))
    }

    /// 创建角色
    /// 返回创建的角色模型
    pub async fn create(
        role: role::ActiveModel,
        db: &DatabaseConnection,
    ) -> Result<role::Model, DbErr> {
        let insert_result = role::Entity::insert(role).exec(db).await?;
        let role_id = insert_result.last_insert_id;

        // 重新查询获取完整的角色模型
        role::Entity::find_by_id(role_id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("Role not found after creation".to_string()))
    }

    /// 更新角色
    /// 返回更新后的角色模型
    pub async fn update(
        id: i64,
        mut role: role::ActiveModel,
        db: &DatabaseConnection,
    ) -> Result<role::Model, DbErr> {
        role.id = ActiveValue::Set(id);
        let updated_role = role::Entity::update(role).exec(db).await?;

        Ok(updated_role)
    }

    /// 删除角色（物理删除）
    /// 返回操作结果
    pub async fn delete(
        id: i64,
        db: &DatabaseConnection,
    ) -> Result<(), DbErr> {
        // 物理删除
        role::Entity::delete_by_id(id)
            .exec(db)
            .await?;

        Ok(())
    }

    /// 检查角色编码是否存在（数据库表中没有code字段，此方法已禁用）
    /// 返回是否存在
    pub async fn exists_by_code(
        _code: &str,
        _db: &DatabaseConnection,
    ) -> Result<bool, DbErr> {
        // 数据库表中没有code字段
        Ok(false)
    }

    /// 分页查询角色
    /// 返回角色列表和总数
    pub async fn find_with_pagination(
        _repo: &RoleRepository,
        query: &crate::app::role::dto::RolePaginationQuery,
        db: &DatabaseConnection,
    ) -> Result<(Vec<role::Model>, usize), DbErr> {
        let mut select = role::Entity::find();

        // 关键词搜索（数据库表中没有code字段）
        if let Some(keyword) = &query.keyword {
            select = select.filter(
                role::Column::Name.like(format!("%{}%", keyword))
            );
        }

        // 角色名称搜索
        if let Some(name) = &query.name {
            select = select.filter(role::Column::Name.like(format!("%{}%", name)));
        }

        // 状态过滤
        if let Some(status) = query.status {
            select = select.filter(role::Column::Status.eq(status));
        }

        // 排序
        if let Some(sort_by) = &query.sort_by {
            let order = query.sort_order.as_ref().unwrap_or(&crate::app::role::dto::SortOrder::Asc);

            match sort_by {
                crate::app::role::dto::RoleSortField::Id => {
                    select = select.order_by(role::Column::Id, match order {
                        crate::app::role::dto::SortOrder::Asc => sea_orm::Order::Asc,
                        crate::app::role::dto::SortOrder::Desc => sea_orm::Order::Desc,
                    });
                }
                crate::app::role::dto::RoleSortField::Name => {
                    select = select.order_by(role::Column::Name, match order {
                        crate::app::role::dto::SortOrder::Asc => sea_orm::Order::Asc,
                        crate::app::role::dto::SortOrder::Desc => sea_orm::Order::Desc,
                    });
                }
                crate::app::role::dto::RoleSortField::Status => {
                    select = select.order_by(role::Column::Status, match order {
                        crate::app::role::dto::SortOrder::Asc => sea_orm::Order::Asc,
                        crate::app::role::dto::SortOrder::Desc => sea_orm::Order::Desc,
                    });
                }
                crate::app::role::dto::RoleSortField::CreatedTime => {
                    select = select.order_by(role::Column::CreatedTime, match order {
                        crate::app::role::dto::SortOrder::Asc => sea_orm::Order::Asc,
                        crate::app::role::dto::SortOrder::Desc => sea_orm::Order::Desc,
                    });
                }
                crate::app::role::dto::RoleSortField::UpdatedTime => {
                    select = select.order_by(role::Column::UpdatedTime, match order {
                        crate::app::role::dto::SortOrder::Asc => sea_orm::Order::Asc,
                        crate::app::role::dto::SortOrder::Desc => sea_orm::Order::Desc,
                    });
                }
            }
        } else {
            // 默认按ID排序（数据库表中没有sort字段）
            select = select.order_by(role::Column::Id, sea_orm::Order::Asc);
        }

        // 分页
        let page = query.page.unwrap_or(1);
        let size = query.size.unwrap_or(20);
        let offset = (page - 1) * size;

        // 查询总数
        let mut total_select = role::Entity::find();

        if let Some(kw) = &query.keyword {
            total_select = total_select.filter(
                role::Column::Name.like(format!("%{}%", kw))
            );
        }
        if let Some(n) = &query.name {
            total_select = total_select.filter(role::Column::Name.like(format!("%{}%", n)));
        }
        // 数据库表中没有code字段
        // if let Some(c) = &query.code {
        //     total_select = total_select.filter(role::Column::Code.like(format!("%{}%", c)));
        // }
        if let Some(s) = query.status {
            total_select = total_select.filter(role::Column::Status.eq(s));
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

    /// 获取所有角色
    /// 返回所有角色列表
    pub async fn find_all(
        _repo: &RoleRepository,
        db: &DatabaseConnection,
    ) -> Result<Vec<role::Model>, DbErr> {
        let roles = role::Entity::find()
            .all(db)
            .await?;

        Ok(roles)
    }
}

// 为RoleRepo创建别名，以便在Service层中使用
pub use RoleRepository as RoleCrud;
