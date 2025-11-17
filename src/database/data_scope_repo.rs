/// 数据权限数据访问层
/// 提供数据权限相关的数据库操作

use crate::database::entity::data_scope;
use crate::database::DatabaseConnection;
use sea_orm::{ActiveValue, ColumnTrait, DbErr, EntityTrait, QueryFilter};

pub struct DataScopeRepository;

impl DataScopeRepository {
    /// 根据角色ID查找数据权限配置
    pub async fn find_by_role_id(
        role_id: i64,
        db: &DatabaseConnection,
    ) -> Result<Option<data_scope::Model>, DbErr> {
        data_scope::Entity::find()
            .filter(data_scope::Column::RoleId.eq(role_id))
            .one(db)
            .await
    }

    /// 创建数据权限配置
    pub async fn create(
        role_id: i64,
        data_scope_value: i32,
        custom_data: Option<String>,
        db: &DatabaseConnection,
    ) -> Result<data_scope::Model, DbErr> {
        let active_model = data_scope::ActiveModel {
            id: ActiveValue::NotSet,
            role_id: ActiveValue::Set(role_id),
            data_scope: ActiveValue::Set(data_scope_value),
            custom_data: ActiveValue::Set(custom_data),
            created_time: ActiveValue::Set(chrono::Utc::now().naive_utc()),
            updated_time: ActiveValue::Set(chrono::Utc::now().naive_utc()),
        };

        let insert_result = data_scope::Entity::insert(active_model).exec(db).await?;
        let inserted_id = insert_result.last_insert_id;

        // 重新查询获取完整模型
        data_scope::Entity::find_by_id(inserted_id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("Data scope not found after creation".to_string()))
    }

    /// 更新数据权限配置
    pub async fn update(
        role_id: i64,
        data_scope_value: i32,
        custom_data: Option<String>,
        db: &DatabaseConnection,
    ) -> Result<data_scope::Model, DbErr> {
        // 先查找现有记录
        let existing = Self::find_by_role_id(role_id, db).await?
            .ok_or(DbErr::RecordNotFound("Data scope not found".to_string()))?;

        // 转换为 ActiveModel
        let mut active_model: data_scope::ActiveModel = existing.into();

        // 设置要更新的字段
        active_model.data_scope = ActiveValue::Set(data_scope_value);
        active_model.custom_data = ActiveValue::Set(custom_data);
        active_model.updated_time = ActiveValue::Set(chrono::Utc::now().naive_utc());

        // 执行更新
        let updated = data_scope::Entity::update(active_model).exec(db).await?;
        Ok(updated)
    }

    /// 获取所有数据权限配置
    pub async fn find_all(
        db: &DatabaseConnection,
    ) -> Result<Vec<data_scope::Model>, DbErr> {
        data_scope::Entity::find()
            .all(db)
            .await
    }

    /// 删除数据权限配置
    pub async fn delete(
        role_id: i64,
        db: &DatabaseConnection,
    ) -> Result<(), DbErr> {
        data_scope::Entity::delete_many()
            .filter(data_scope::Column::RoleId.eq(role_id))
            .exec(db)
            .await?;
        Ok(())
    }
}
