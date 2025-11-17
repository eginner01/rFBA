/// 数据规则数据访问层
/// 提供数据规则相关的数据库操作

use crate::database::entity::data_rule;
use crate::database::DatabaseConnection;
use sea_orm::{ActiveValue, ColumnTrait, DbErr, EntityTrait, QueryFilter, QueryOrder, PaginatorTrait};

pub struct DataRuleRepository;

impl DataRuleRepository {
    /// 根据ID查找数据规则
    pub async fn find_by_id(
        id: i64,
        db: &DatabaseConnection,
    ) -> Result<Option<data_rule::Model>, DbErr> {
        data_rule::Entity::find()
            .filter(data_rule::Column::Id.eq(id))
            .filter(data_rule::Column::DelFlag.eq(0))
            .one(db)
            .await
    }

    /// 根据规则编码查找数据规则
    pub async fn find_by_code(
        code: &str,
        db: &DatabaseConnection,
    ) -> Result<Option<data_rule::Model>, DbErr> {
        data_rule::Entity::find()
            .filter(data_rule::Column::Code.eq(code))
            .filter(data_rule::Column::DelFlag.eq(0))
            .one(db)
            .await
    }

    /// 根据模型查找数据规则列表
    pub async fn find_by_model(
        model: &str,
        db: &DatabaseConnection,
    ) -> Result<Vec<data_rule::Model>, DbErr> {
        data_rule::Entity::find()
            .filter(data_rule::Column::Model.eq(model))
            .filter(data_rule::Column::DelFlag.eq(0))
            .order_by(data_rule::Column::Sort, sea_orm::Order::Asc)
            .all(db)
            .await
    }

    /// 创建数据规则
    pub async fn create(
        data: &CreateDataRule,
        db: &DatabaseConnection,
    ) -> Result<data_rule::Model, DbErr> {
        let active_model = data_rule::ActiveModel {
            id: ActiveValue::NotSet,
            name: ActiveValue::Set(data.name.clone()),
            code: ActiveValue::Set(data.code.clone()),
            model: ActiveValue::Set(data.model.clone()),
            columns: ActiveValue::Set(data.columns.clone()),
            field_permissions: ActiveValue::Set(data.field_permissions.clone()),
            status: ActiveValue::Set(data.status),
            sort: ActiveValue::Set(data.sort),
            remark: ActiveValue::Set(data.remark.clone()),
            created_time: ActiveValue::Set(chrono::Utc::now().naive_utc()),
            updated_time: ActiveValue::Set(chrono::Utc::now().naive_utc()),
            create_by: ActiveValue::Set(data.create_by.clone()),
            update_by: ActiveValue::Set(data.update_by.clone()),
            del_flag: ActiveValue::Set(Some(0)),
        };

        let insert_result = data_rule::Entity::insert(active_model).exec(db).await?;
        let inserted_id = insert_result.last_insert_id;

        // 重新查询获取完整模型
        data_rule::Entity::find_by_id(inserted_id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("Data rule not found after creation".to_string()))
    }

    /// 更新数据规则
    pub async fn update(
        id: i64,
        data: &UpdateDataRule,
        db: &DatabaseConnection,
    ) -> Result<data_rule::Model, DbErr> {
        // 先查找现有记录
        let existing = Self::find_by_id(id, db).await?
            .ok_or(DbErr::RecordNotFound("Data rule not found".to_string()))?;

        // 转换为 ActiveModel
        let mut active_model: data_rule::ActiveModel = existing.into();

        // 设置要更新的字段
        if let Some(name) = &data.name {
            active_model.name = ActiveValue::Set(name.clone());
        }
        if let Some(code) = &data.code {
            active_model.code = ActiveValue::Set(code.clone());
        }
        if let Some(model) = &data.model {
            active_model.model = ActiveValue::Set(model.clone());
        }
        if let Some(columns) = data.columns.clone() {
            active_model.columns = ActiveValue::Set(Some(columns));
        }
        if let Some(field_permissions) = data.field_permissions.clone() {
            active_model.field_permissions = ActiveValue::Set(Some(field_permissions));
        }
        if let Some(status) = data.status {
            active_model.status = ActiveValue::Set(status);
        }
        if let Some(sort) = data.sort {
            active_model.sort = ActiveValue::Set(sort);
        }
        if let Some(remark) = data.remark.clone() {
            active_model.remark = ActiveValue::Set(Some(remark));
        }
        if let Some(update_by) = data.update_by.clone() {
            active_model.update_by = ActiveValue::Set(Some(update_by));
        }
        active_model.updated_time = ActiveValue::Set(chrono::Utc::now().naive_utc());

        // 执行更新
        let updated = data_rule::Entity::update(active_model).exec(db).await?;
        Ok(updated)
    }

    /// 获取所有数据规则（分页）
    pub async fn find_paginated(
        page: u64,
        size: u64,
        db: &DatabaseConnection,
    ) -> Result<(Vec<data_rule::Model>, u64), DbErr> {
        let paginator = data_rule::Entity::find()
            .filter(data_rule::Column::DelFlag.eq(0))
            .order_by(data_rule::Column::Sort, sea_orm::Order::Asc)
            .paginate(db, size);

        let total = paginator.num_pages().await?;
        let items = paginator.fetch_page(page - 1).await?;

        Ok((items, total))
    }

    /// 获取所有数据规则
    pub async fn find_all(
        db: &DatabaseConnection,
    ) -> Result<Vec<data_rule::Model>, DbErr> {
        data_rule::Entity::find()
            .filter(data_rule::Column::DelFlag.eq(0))
            .order_by(data_rule::Column::Sort, sea_orm::Order::Asc)
            .all(db)
            .await
    }

    /// 删除数据规则（软删除）
    pub async fn delete(
        id: i64,
        db: &DatabaseConnection,
    ) -> Result<(), DbErr> {
        // 软删除：更新del_flag为1
        let existing = Self::find_by_id(id, db).await?
            .ok_or(DbErr::RecordNotFound("Data rule not found".to_string()))?;

        let mut active_model: data_rule::ActiveModel = existing.into();
        active_model.del_flag = ActiveValue::Set(Some(1));
        active_model.updated_time = ActiveValue::Set(chrono::Utc::now().naive_utc());

        data_rule::Entity::update(active_model).exec(db).await?;
        Ok(())
    }

    /// 批量删除数据规则（软删除）
    pub async fn batch_delete(
        ids: &[i64],
        db: &DatabaseConnection,
    ) -> Result<(), DbErr> {
        for &id in ids {
            Self::delete(id, db).await?;
        }
        Ok(())
    }

    /// 检查规则编码是否已存在
    pub async fn check_code_exists(
        code: &str,
        exclude_id: Option<i64>,
        db: &DatabaseConnection,
    ) -> Result<bool, DbErr> {
        let mut query = data_rule::Entity::find()
            .filter(data_rule::Column::Code.eq(code))
            .filter(data_rule::Column::DelFlag.eq(0));

        if let Some(id) = exclude_id {
            query = query.filter(data_rule::Column::Id.ne(id));
        }

        let result = query.one(db).await?;
        Ok(result.is_some())
    }
}

/// 创建数据规则的数据结构
pub struct CreateDataRule {
    pub name: String,
    pub code: String,
    pub model: String,
    pub columns: Option<String>,
    pub field_permissions: Option<String>,
    pub status: i32,
    pub sort: i32,
    pub remark: Option<String>,
    pub create_by: Option<String>,
    pub update_by: Option<String>,
}

/// 更新数据规则的数据结构
pub struct UpdateDataRule {
    pub name: Option<String>,
    pub code: Option<String>,
    pub model: Option<String>,
    pub columns: Option<String>,
    pub field_permissions: Option<String>,
    pub status: Option<i32>,
    pub sort: Option<i32>,
    pub remark: Option<String>,
    pub update_by: Option<String>,
}
