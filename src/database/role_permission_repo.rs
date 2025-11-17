/// 角色权限关联数据访问层
/// 提供角色权限相关的数据库操作

use crate::database::entity::role_permission;
use crate::database::DatabaseConnection;
use sea_orm::{ActiveValue, ColumnTrait, DbErr, EntityTrait, QueryFilter, QuerySelect, Related, PaginatorTrait};

/// 角色权限关联仓库
pub struct RolePermissionRepository;

impl RolePermissionRepository {
    /// 查找角色的所有权限
    /// 返回权限ID列表
    pub async fn find_permissions_by_role(
        role_id: i64,
        db: &DatabaseConnection,
    ) -> Result<Vec<i64>, DbErr> {
        let role_permissions = role_permission::Entity::find()
            .filter(role_permission::Column::RoleId.eq(role_id))
            .all(db)
            .await?;

        Ok(role_permissions.into_iter()
            .map(|rp| rp.permission_id)
            .collect())
    }

    /// 查找权限的所有角色
    /// 返回角色ID列表
    pub async fn find_roles_by_permission(
        permission_id: i64,
        db: &DatabaseConnection,
    ) -> Result<Vec<i64>, DbErr> {
        let role_permissions = role_permission::Entity::find()
            .filter(role_permission::Column::PermissionId.eq(permission_id))
            .all(db)
            .await?;

        Ok(role_permissions.into_iter()
            .map(|rp| rp.role_id)
            .collect())
    }

    /// 批量插入角色权限关联
    /// 返回操作结果
    pub async fn batch_insert(
        role_id: i64,
        permission_ids: &[i64],
        db: &DatabaseConnection,
    ) -> Result<(), DbErr> {
        let models: Vec<role_permission::ActiveModel> = permission_ids
            .iter()
            .map(|&permission_id| role_permission::ActiveModel {
                id: ActiveValue::NotSet,
                role_id: ActiveValue::Set(role_id),
                permission_id: ActiveValue::Set(permission_id),
            })
            .collect();

        if !models.is_empty() {
            role_permission::Entity::insert_many(models)
                .exec(db)
                .await?;
        }

        Ok(())
    }

    /// 删除角色的所有权限关联
    /// 返回操作结果
    pub async fn delete_by_role(
        role_id: i64,
        db: &DatabaseConnection,
    ) -> Result<u64, DbErr> {
        let count = role_permission::Entity::delete_many()
            .filter(role_permission::Column::RoleId.eq(role_id))
            .exec(db)
            .await?;

        Ok(count.rows_affected)
    }

    /// 删除权限的所有角色关联
    /// 返回操作结果
    pub async fn delete_by_permission(
        permission_id: i64,
        db: &DatabaseConnection,
    ) -> Result<u64, DbErr> {
        let count = role_permission::Entity::delete_many()
            .filter(role_permission::Column::PermissionId.eq(permission_id))
            .exec(db)
            .await?;

        Ok(count.rows_affected)
    }

    /// 检查角色是否有指定权限
    /// 返回是否存在
    pub async fn exists(
        role_id: i64,
        permission_id: i64,
        db: &DatabaseConnection,
    ) -> Result<bool, DbErr> {
        let count = role_permission::Entity::find()
            .filter(role_permission::Column::RoleId.eq(role_id))
            .filter(role_permission::Column::PermissionId.eq(permission_id))
            .count(db)
            .await?;

        Ok(count > 0)
    }

    /// 批量检查角色是否拥有任意一个权限
    /// 返回是否存在
    pub async fn exists_any(
        role_id: i64,
        permission_ids: &[i64],
        db: &DatabaseConnection,
    ) -> Result<bool, DbErr> {
        let permission_values: Vec<sea_orm::Value> = permission_ids.iter()
            .map(|&id| sea_orm::Value::BigInt(Some(id)))
            .collect();

        let count = role_permission::Entity::find()
            .filter(role_permission::Column::RoleId.eq(role_id))
            .filter(role_permission::Column::PermissionId.is_in(permission_values))
            .count(db)
            .await?;

        Ok(count > 0)
    }

    /// 批量检查角色是否拥有所有权限
    /// 返回是否存在
    pub async fn exists_all(
        role_id: i64,
        permission_ids: &[i64],
        db: &DatabaseConnection,
    ) -> Result<bool, DbErr> {
        for &permission_id in permission_ids {
            let exists = Self::exists(role_id, permission_id, db).await?;
            if !exists {
                return Ok(false);
            }
        }
        Ok(true)
    }
}

// 为RolePermissionRepository创建别名
pub use RolePermissionRepository as RolePermissionCrud;
