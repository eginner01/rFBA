/// 权限数据访问层
/// 提供权限相关的数据库操作

use crate::database::entity::permission;
use crate::database::DatabaseConnection;
use sea_orm::{ActiveValue, ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter, QuerySelect, QueryOrder};

/// 权限仓库
pub struct PermissionRepository;

impl PermissionRepository {
    /// 根据ID查找权限
    /// 返回权限模型
    pub async fn find_by_id(
        id: i64,
        db: &DatabaseConnection,
    ) -> Result<permission::Model, DbErr> {
        let permission = permission::Entity::find_by_id(id)
            .filter(permission::Column::DelFlag.eq(0))
            .one(db)
            .await?;

        permission.ok_or(DbErr::RecordNotFound("Permission not found".to_string()))
    }

    /// 根据权限编码查找权限
    /// 返回权限模型
    pub async fn find_by_code(
        code: &str,
        db: &DatabaseConnection,
    ) -> Result<permission::Model, DbErr> {
        let permission = permission::Entity::find()
            .filter(permission::Column::Code.eq(code))
            .filter(permission::Column::DelFlag.eq(0))
            .one(db)
            .await?;

        permission.ok_or(DbErr::RecordNotFound("Permission not found".to_string()))
    }

    /// 创建权限
    /// 返回创建的权限模型
    pub async fn create(
        permission: permission::ActiveModel,
        db: &DatabaseConnection,
    ) -> Result<permission::Model, DbErr> {
        let insert_result = permission::Entity::insert(permission).exec(db).await?;
        let permission_id = insert_result.last_insert_id;

        // 重新查询获取完整的权限模型
        permission::Entity::find_by_id(permission_id)
            .filter(permission::Column::DelFlag.eq(0))
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("Permission not found after creation".to_string()))
    }

    /// 更新权限
    /// 返回更新后的权限模型
    pub async fn update(
        id: i64,
        mut permission: permission::ActiveModel,
        db: &DatabaseConnection,
    ) -> Result<permission::Model, DbErr> {
        permission.id = ActiveValue::Set(id);
        let updated_permission = permission::Entity::update(permission).exec(db).await?;

        Ok(updated_permission)
    }

    /// 删除权限（软删除）
    /// 返回操作结果
    pub async fn delete(
        id: i64,
        db: &DatabaseConnection,
    ) -> Result<(), DbErr> {
        let permission: permission::ActiveModel = permission::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("Permission not found".to_string()))?
            .into();

        let mut update_permission = permission.clone();
        update_permission.del_flag = ActiveValue::Set(1);
        update_permission.updated_time = ActiveValue::Set(chrono::Utc::now());

        permission::Entity::update(update_permission)
            .exec(db)
            .await?;

        Ok(())
    }

    /// 检查权限编码是否已存在
    /// 返回是否存在
    pub async fn exists_by_code(
        code: &str,
        db: &DatabaseConnection,
    ) -> Result<bool, DbErr> {
        let count = permission::Entity::find()
            .filter(permission::Column::Code.eq(code))
            .filter(permission::Column::DelFlag.eq(0))
            .count(db)
            .await?;

        Ok(count > 0)
    }

    /// 分页查询权限
    /// 返回权限列表和总数
    pub async fn find_with_pagination(
        _repo: &PermissionRepository,
        query: &crate::app::permission::dto::PermissionPaginationQuery,
        db: &DatabaseConnection,
    ) -> Result<(Vec<permission::Model>, usize), DbErr> {
        let mut select = permission::Entity::find()
            .filter(permission::Column::DelFlag.eq(0));

        // 关键词搜索
        if let Some(keyword) = &query.keyword {
            select = select.filter(
                sea_orm::Condition::any()
                    .add(permission::Column::Name.like(format!("%{}%", keyword)))
                    .add(permission::Column::Code.like(format!("%{}%", keyword))),
            );
        }

        // 权限名搜索
        if let Some(name) = &query.name {
            select = select.filter(permission::Column::Name.like(format!("%{}%", name)));
        }

        // 权限编码搜索
        if let Some(code) = &query.code {
            select = select.filter(permission::Column::Code.like(format!("%{}%", code)));
        }

        // 权限类型过滤
        if let Some(permission_type) = query.permission_type {
            select = select.filter(permission::Column::Type.eq(permission_type));
        }

        // 状态过滤
        if let Some(status) = query.status {
            select = select.filter(permission::Column::Status.eq(status));
        }

        // 父权限ID过滤
        if let Some(parent_id) = query.parent_id {
            select = select.filter(permission::Column::ParentId.eq(parent_id));
        }

        // 排序
        if let Some(sort_by) = &query.sort_by {
            let order = query.sort_order.as_ref().unwrap_or(&crate::app::permission::dto::SortOrder::Asc);

            match sort_by {
                crate::app::permission::dto::PermissionSortField::Id => {
                    select = select.order_by(permission::Column::Id, match order {
                        crate::app::permission::dto::SortOrder::Asc => sea_orm::Order::Asc,
                        crate::app::permission::dto::SortOrder::Desc => sea_orm::Order::Desc,
                    });
                }
                crate::app::permission::dto::PermissionSortField::Name => {
                    select = select.order_by(permission::Column::Name, match order {
                        crate::app::permission::dto::SortOrder::Asc => sea_orm::Order::Asc,
                        crate::app::permission::dto::SortOrder::Desc => sea_orm::Order::Desc,
                    });
                }
                crate::app::permission::dto::PermissionSortField::Code => {
                    select = select.order_by(permission::Column::Code, match order {
                        crate::app::permission::dto::SortOrder::Asc => sea_orm::Order::Asc,
                        crate::app::permission::dto::SortOrder::Desc => sea_orm::Order::Desc,
                    });
                }
                crate::app::permission::dto::PermissionSortField::PermissionType => {
                    select = select.order_by(permission::Column::Type, match order {
                        crate::app::permission::dto::SortOrder::Asc => sea_orm::Order::Asc,
                        crate::app::permission::dto::SortOrder::Desc => sea_orm::Order::Desc,
                    });
                }
                crate::app::permission::dto::PermissionSortField::Sort => {
                    select = select.order_by(permission::Column::Sort, match order {
                        crate::app::permission::dto::SortOrder::Asc => sea_orm::Order::Asc,
                        crate::app::permission::dto::SortOrder::Desc => sea_orm::Order::Desc,
                    });
                }
                crate::app::permission::dto::PermissionSortField::CreatedTime => {
                    select = select.order_by(permission::Column::CreatedTime, match order {
                        crate::app::permission::dto::SortOrder::Asc => sea_orm::Order::Asc,
                        crate::app::permission::dto::SortOrder::Desc => sea_orm::Order::Desc,
                    });
                }
                crate::app::permission::dto::PermissionSortField::UpdatedTime => {
                    select = select.order_by(permission::Column::UpdatedTime, match order {
                        crate::app::permission::dto::SortOrder::Asc => sea_orm::Order::Asc,
                        crate::app::permission::dto::SortOrder::Desc => sea_orm::Order::Desc,
                    });
                }
            }
        } else {
            // 默认按排序号升序排序
            select = select.order_by(permission::Column::Sort, sea_orm::Order::Asc);
        }

        // 分页
        let size = query.size.unwrap_or(20);
        let page = query.page.unwrap_or(1);
        let offset = (page - 1) * size;

        // 查询总数
        let mut total_select = permission::Entity::find()
            .filter(permission::Column::DelFlag.eq(0));

        if let Some(kw) = &query.keyword {
            total_select = total_select.filter(
                sea_orm::Condition::any()
                    .add(permission::Column::Name.like(format!("%{}%", kw)))
                    .add(permission::Column::Code.like(format!("%{}%", kw))),
            );
        }
        if let Some(n) = &query.name {
            total_select = total_select.filter(permission::Column::Name.like(format!("%{}%", n)));
        }
        if let Some(c) = &query.code {
            total_select = total_select.filter(permission::Column::Code.like(format!("%{}%", c)));
        }
        if let Some(pt) = query.permission_type {
            total_select = total_select.filter(permission::Column::Type.eq(pt));
        }
        if let Some(s) = query.status {
            total_select = total_select.filter(permission::Column::Status.eq(s));
        }
        if let Some(pid) = query.parent_id {
            total_select = total_select.filter(permission::Column::ParentId.eq(pid));
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

    /// 获取所有权限（不分页，用于树形结构）
    /// 返回权限列表
    pub async fn find_all(
        _repo: &PermissionRepository,
        db: &DatabaseConnection,
    ) -> Result<Vec<permission::Model>, DbErr> {
        let permissions = permission::Entity::find()
            .filter(permission::Column::DelFlag.eq(0))
            .order_by(permission::Column::Sort, sea_orm::Order::Asc)
            .all(db)
            .await?;

        Ok(permissions)
    }

    /// 检查权限是否有子权限
    /// 返回子权限数量
    pub async fn count_children(
        id: i64,
        db: &DatabaseConnection,
    ) -> Result<u64, DbErr> {
        let count = permission::Entity::find()
            .filter(permission::Column::ParentId.eq(id))
            .filter(permission::Column::DelFlag.eq(0))
            .count(db)
            .await?;

        Ok(count)
    }
}

// 为PermissionRepository创建别名，以便在Service层中使用
pub use PermissionRepository as PermissionCrud;
