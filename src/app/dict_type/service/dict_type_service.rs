use tracing::{info, error};

use crate::app::dict_type::dto::{
    CreateDictTypeRequest, DictTypePageResponse, DictTypeQuery, DictTypeResponse,
    UpdateDictTypeRequest,
};
use crate::common::exception::{AppError, ErrorCode};
use sea_orm::{DatabaseConnection, QueryFilter, QuerySelect, PaginatorTrait};

#[derive(Clone)]
pub struct DictTypeService {
    db: DatabaseConnection,
}

impl DictTypeService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn list(&self, query: &DictTypeQuery) -> Result<DictTypePageResponse, AppError> {
        use crate::database::entity::dict_type::{Entity as DictType, Column as DictTypeColumn};
        use sea_orm::{EntityTrait, QueryFilter, QueryOrder, ColumnTrait, Condition};

        let page = query.page.unwrap_or(1);
        let page_size = query.page_size.unwrap_or(10);
        let offset = (page - 1) * page_size;

        let mut condition = Condition::all();
        if let Some(keyword) = &query.keyword {
            if !keyword.is_empty() {
                condition = condition.add(
                    Condition::any()
                        .add(DictTypeColumn::Name.like(format!("%{}%", keyword)))
                        .add(DictTypeColumn::Code.like(format!("%{}%", keyword)))
                );
            }
        }

        let select = DictType::find().filter(condition);
        let total = select.clone()
            .count(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to count dict types: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, format!("Failed to count dict types: {}", e))
            })?;

        let total_pages = total.div_ceil(page_size);

        // 查询字典类型列表
        let dict_types = select
            .offset((page - 1) * page_size)
            .limit(page_size)
            .order_by_asc(DictTypeColumn::Code)
            .all(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to query dict types: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, format!("Failed to query dict types: {}", e))
            })?;

        // 转换为响应DTO
        let list = dict_types
            .into_iter()
            .map(|dict_type| DictTypeResponse {
                id: dict_type.id,
                name: dict_type.name,
                code: dict_type.code,
                remark: dict_type.remark,
                created_time: dict_type.created_time,
                updated_time: dict_type.updated_time,
            })
            .collect();

        // 构建分页响应
        let response = DictTypePageResponse {
            total,
            page,
            page_size,
            total_pages,
            list,
        };

        tracing::info!("Retrieved {} dict types (page {}/{})", response.list.len(), page, total_pages);

        Ok(response)
    }

    /// 根据 ID 获取字典类型
    pub async fn get_by_id(&self, id: i64) -> Result<DictTypeResponse, AppError> {
        use crate::database::entity::dict_type::Entity as DictType;
        use sea_orm::EntityTrait;

        let dict_type_model = DictType::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to find dict type: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, format!("Failed to find dict type: {}", e))
            })?
            .ok_or_else(|| {
                AppError::with_message(ErrorCode::NotFound, "字典类型不存在")
            })?;

        let response = DictTypeResponse {
            id: dict_type_model.id,
            name: dict_type_model.name,
            code: dict_type_model.code,
            remark: dict_type_model.remark,
            created_time: dict_type_model.created_time,
            updated_time: dict_type_model.updated_time,
        };

        tracing::info!("Retrieved dict type id: {}", id);

        Ok(response)
    }

    /// 创建字典类型
    pub async fn create(&self, request: &CreateDictTypeRequest) -> Result<DictTypeResponse, AppError> {
        use crate::database::entity::dict_type::{Entity as DictType, Column as DictTypeColumn, ActiveModel};
        use sea_orm::{EntityTrait, ActiveValue, ColumnTrait, ActiveModelTrait, QueryFilter};
        use chrono::Utc;

        // 检查字典类型code是否已存在
        let existing = DictType::find()
            .filter(DictTypeColumn::Code.eq(&request.code))
            .one(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to find dict type: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, format!("Failed to find dict type: {}", e))
            })?;

        if existing.is_some() {
            return Err(AppError::with_message(ErrorCode::BadRequest, "字典类型已存在"));
        }

        // 创建新的字典类型
        let active_model = ActiveModel {
            id: ActiveValue::NotSet,
            name: ActiveValue::Set(request.name.clone()),
            code: ActiveValue::Set(request.code.clone()),
            remark: ActiveValue::Set(request.remark.clone()),
            created_time: ActiveValue::Set(Utc::now()),
            updated_time: ActiveValue::Set(Some(Utc::now())),
        };

        let saved_dict_type = active_model.insert(&self.db).await.map_err(|e| {
            error!("Failed to create dict type: {:?}", e);
            AppError::with_message(ErrorCode::DatabaseError, format!("Failed to create dict type: {}", e))
        })?;

        let response = DictTypeResponse {
            id: saved_dict_type.id,
            name: saved_dict_type.name,
            code: saved_dict_type.code,
            remark: saved_dict_type.remark,
            created_time: saved_dict_type.created_time,
            updated_time: saved_dict_type.updated_time,
        };

        info!("Created dict type: {}", request.code);

        Ok(response)
    }

    /// 更新字典类型
    pub async fn update(
        &self,
        id: i64,
        request: &UpdateDictTypeRequest,
    ) -> Result<DictTypeResponse, AppError> {
        use crate::database::entity::dict_type::{Entity as DictType, Column as DictTypeColumn, ActiveModel};
        use sea_orm::{EntityTrait, ActiveValue, ColumnTrait, ActiveModelTrait, QueryFilter};
        use chrono::Utc;

        // 检查字典类型是否存在
        let existing = DictType::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to find dict type: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, format!("Failed to find dict type: {}", e))
            })?
            .ok_or_else(|| {
                AppError::with_message(ErrorCode::NotFound, "字典类型不存在")
            })?;

        // 检查code是否与其他记录冲突
        if existing.code != request.code {
            let code_exists = DictType::find()
                .filter(DictTypeColumn::Code.eq(&request.code))
                .one(&self.db)
                .await
                .map_err(|e| {
                    error!("Failed to check code existence: {:?}", e);
                    AppError::with_message(ErrorCode::DatabaseError, format!("Failed to check code: {}", e))
                })?;

            if code_exists.is_some() {
                return Err(AppError::with_message(ErrorCode::BadRequest, "字典类型编码已存在"));
            }
        }

        // 更新字典类型
        let active_model = ActiveModel {
            id: ActiveValue::Set(existing.id),
            name: ActiveValue::Set(request.name.clone()),
            code: ActiveValue::Set(request.code.clone()),
            remark: ActiveValue::Set(request.remark.clone()),
            created_time: ActiveValue::Set(existing.created_time),
            updated_time: ActiveValue::Set(Some(Utc::now())),
        };

        let saved_dict_type = active_model.update(&self.db).await.map_err(|e| {
            error!("Failed to update dict type: {:?}", e);
            AppError::with_message(ErrorCode::DatabaseError, format!("Failed to update dict type: {}", e))
        })?;

        let response = DictTypeResponse {
            id: saved_dict_type.id,
            name: saved_dict_type.name,
            code: saved_dict_type.code,
            remark: saved_dict_type.remark,
            created_time: saved_dict_type.created_time,
            updated_time: saved_dict_type.updated_time,
        };

        info!("Updated dict type id: {}", id);

        Ok(response)
    }

    /// 删除字典类型
    pub async fn delete(&self, id: i64) -> Result<(), AppError> {
        use crate::database::entity::dict_type::Entity as DictType;
        use sea_orm::{EntityTrait, ModelTrait};

        // 检查字典类型是否存在
        let existing = DictType::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to find dict type: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, format!("Failed to find dict type: {}", e))
            })?
            .ok_or_else(|| {
                AppError::with_message(ErrorCode::NotFound, "字典类型不存在")
            })?;

        // 执行硬删除
        existing.delete(&self.db).await.map_err(|e| {
            error!("Failed to delete dict type: {:?}", e);
            AppError::with_message(ErrorCode::DatabaseError, format!("Failed to delete dict type: {}", e))
        })?;

        info!("Deleted dict type id: {}", id);

        Ok(())
    }

    /// 批量删除字典类型
    pub async fn batch_delete(&self, ids: &[i64]) -> Result<(), AppError> {
        use crate::database::entity::dict_type::Entity as DictType;
        use sea_orm::{EntityTrait, ColumnTrait};
        use crate::database::entity::dict_type::Column as DictTypeColumn;

        // 执行批量删除
        let delete_result = DictType::delete_many()
            .filter(DictTypeColumn::Id.is_in(ids.iter().copied()))
            .exec(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to batch delete dict types: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, format!("Failed to batch delete: {}", e))
            })?;

        info!("Batch deleted {} dict types", delete_result.rows_affected);

        Ok(())
    }

    /// 获取所有字典类型
    pub async fn get_all(&self) -> Result<Vec<DictTypeResponse>, AppError> {
        use crate::database::entity::dict_type::{Entity as DictType, Column as DictTypeColumn};
        use sea_orm::{EntityTrait, QueryOrder};

        // 查询所有字典类型
        let dict_types = DictType::find()
            .order_by_asc(DictTypeColumn::Code)
            .all(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to query dict types: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, format!("Failed to query dict types: {}", e))
            })?;

        // 转换为DTO
        let list: Vec<DictTypeResponse> = dict_types
            .into_iter()
            .map(|dict_type| DictTypeResponse {
                id: dict_type.id,
                name: dict_type.name,
                code: dict_type.code,
                remark: dict_type.remark,
                created_time: dict_type.created_time,
                updated_time: dict_type.updated_time,
            })
            .collect();

        info!("Retrieved {} dict types", list.len());

        Ok(list)
    }
}

