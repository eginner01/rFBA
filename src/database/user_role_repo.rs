/// 用户角色关联数据访问层
/// 提供用户角色相关的数据库操作

use crate::database::entity::user_role;
use crate::database::DatabaseConnection;
use sea_orm::{ActiveValue, ColumnTrait, DbErr, EntityTrait, QueryFilter, PaginatorTrait};

/// 用户角色关联仓库
pub struct UserRoleRepository;

impl UserRoleRepository {
    /// 查找用户的所有角色
    /// 返回角色ID列表
    pub async fn find_roles_by_user(
        user_id: i64,
        db: &DatabaseConnection,
    ) -> Result<Vec<i64>, DbErr> {
        let user_roles = user_role::Entity::find()
            .filter(user_role::Column::UserId.eq(user_id))
            .all(db)
            .await?;

        Ok(user_roles.into_iter()
            .map(|ur| ur.role_id)
            .collect())
    }

    /// 查找角色的所有用户
    /// 返回用户ID列表
    pub async fn find_users_by_role(
        role_id: i64,
        db: &DatabaseConnection,
    ) -> Result<Vec<i64>, DbErr> {
        let user_roles = user_role::Entity::find()
            .filter(user_role::Column::RoleId.eq(role_id))
            .all(db)
            .await?;

        Ok(user_roles.into_iter()
            .map(|ur| ur.user_id)
            .collect())
    }

    /// 批量插入用户角色关联
    /// 返回操作结果
    pub async fn batch_insert(
        user_id: i64,
        role_ids: &[i64],
        db: &DatabaseConnection,
    ) -> Result<(), DbErr> {
        let models: Vec<user_role::ActiveModel> = role_ids
            .iter()
            .map(|&role_id| user_role::ActiveModel {
                id: ActiveValue::NotSet,
                user_id: ActiveValue::Set(user_id),
                role_id: ActiveValue::Set(role_id),
            })
            .collect();

        if !models.is_empty() {
            user_role::Entity::insert_many(models)
                .exec(db)
                .await?;
        }

        Ok(())
    }

    /// 删除用户的所有角色关联
    /// 返回操作结果
    pub async fn delete_by_user(
        user_id: i64,
        db: &DatabaseConnection,
    ) -> Result<u64, DbErr> {
        let count = user_role::Entity::delete_many()
            .filter(user_role::Column::UserId.eq(user_id))
            .exec(db)
            .await?;

        Ok(count.rows_affected)
    }

    /// 删除角色的所有用户关联
    /// 返回操作结果
    pub async fn delete_by_role(
        role_id: i64,
        db: &DatabaseConnection,
    ) -> Result<u64, DbErr> {
        let count = user_role::Entity::delete_many()
            .filter(user_role::Column::RoleId.eq(role_id))
            .exec(db)
            .await?;

        Ok(count.rows_affected)
    }

    /// 检查用户是否有指定角色
    /// 返回是否存在
    pub async fn exists(
        user_id: i64,
        role_id: i64,
        db: &DatabaseConnection,
    ) -> Result<bool, DbErr> {
        let count = user_role::Entity::find()
            .filter(user_role::Column::UserId.eq(user_id))
            .filter(user_role::Column::RoleId.eq(role_id))
            .count(db)
            .await?;

        Ok(count > 0)
    }

    /// 批量检查用户是否拥有任意一个角色
    /// 返回是否存在
    pub async fn exists_any(
        user_id: i64,
        role_ids: &[i64],
        db: &DatabaseConnection,
    ) -> Result<bool, DbErr> {
        let role_values: Vec<sea_orm::Value> = role_ids.iter()
            .map(|&id| sea_orm::Value::BigInt(Some(id)))
            .collect();

        let count = user_role::Entity::find()
            .filter(user_role::Column::UserId.eq(user_id))
            .filter(user_role::Column::RoleId.is_in(role_values))
            .count(db)
            .await?;

        Ok(count > 0)
    }

    /// 批量检查用户是否拥有所有角色
    /// 返回是否存在
    pub async fn exists_all(
        user_id: i64,
        role_ids: &[i64],
        db: &DatabaseConnection,
    ) -> Result<bool, DbErr> {
        for &role_id in role_ids {
            let exists = Self::exists(user_id, role_id, db).await?;
            if !exists {
                return Ok(false);
            }
        }
        Ok(true)
    }
}

// 为UserRoleRepository创建别名
pub use UserRoleRepository as UserRoleCrud;
