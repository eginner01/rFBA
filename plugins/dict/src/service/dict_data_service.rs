//! 字典数据服务层

use sea_orm::*;
use crate::entity::{dict_data, dict_type};
use crate::dto::*;
use crate::error::DictError;

/// 字典数据服务
pub struct DictDataService;

impl DictDataService {
    /// 获取所有字典数据
    pub async fn get_all(db: &DatabaseConnection) -> Result<Vec<DictDataDetail>, DictError> {
        let data_list = dict_data::Entity::find()
            .order_by_asc(dict_data::Column::TypeId)
            .order_by_asc(dict_data::Column::Sort)
            .all(db)
            .await
            .map_err(|e| DictError::DatabaseError(e.to_string()))?;
        
        Ok(data_list.into_iter().map(DictDataDetail::from).collect())
    }
    
    /// 根据ID获取字典数据
    pub async fn get_by_id(
        db: &DatabaseConnection,
        id: i64,
    ) -> Result<DictDataDetail, DictError> {
        let dict_data = dict_data::Entity::find_by_id(id)
            .one(db)
            .await
            .map_err(|e| DictError::DatabaseError(e.to_string()))?
            .ok_or(DictError::NotFound("字典数据不存在".to_string()))?;
        
        Ok(DictDataDetail::from(dict_data))
    }
    
    /// 根据类型编码获取字典数据列表
    pub async fn get_by_type_code(
        db: &DatabaseConnection,
        type_code: &str,
    ) -> Result<Vec<DictDataDetail>, DictError> {
        let data_list = dict_data::Entity::find_by_type_code(type_code)
            .filter(dict_data::Column::Status.eq(1))
            .all(db)
            .await
            .map_err(|e| DictError::DatabaseError(e.to_string()))?;
        
        Ok(data_list.into_iter().map(DictDataDetail::from).collect())
    }
    
    /// 分页查询字典数据
    pub async fn get_list(
        db: &DatabaseConnection,
        query: DictDataQuery,
        pagination: PaginationQuery,
    ) -> Result<PageData<DictDataDetail>, DictError> {
        let mut select = dict_data::Entity::find();
        
        // 构建查询条件
        if let Some(type_code) = &query.type_code {
            select = select.filter(dict_data::Column::TypeCode.eq(type_code));
        }
        if let Some(label) = &query.label {
            select = select.filter(dict_data::Column::Label.contains(label));
        }
        if let Some(value) = &query.value {
            select = select.filter(dict_data::Column::Value.contains(value));
        }
        if let Some(status) = query.status {
            select = select.filter(dict_data::Column::Status.eq(status));
        }
        if let Some(type_id) = query.type_id {
            select = select.filter(dict_data::Column::TypeId.eq(type_id));
        }
        
        // 查询总数
        let total = select
            .clone()
            .count(db)
            .await
            .map_err(|e| DictError::DatabaseError(e.to_string()))?;
        
        // 分页查询
        let items = select
            .order_by_asc(dict_data::Column::TypeId)
            .order_by_asc(dict_data::Column::Sort)
            .offset(pagination.offset())
            .limit(pagination.limit())
            .all(db)
            .await
            .map_err(|e| DictError::DatabaseError(e.to_string()))?
            .into_iter()
            .map(DictDataDetail::from)
            .collect();
        
        Ok(PageData::new(items, total, pagination.page, pagination.size))
    }
    
    /// 创建字典数据
    pub async fn create(
        db: &DatabaseConnection,
        param: CreateDictDataParam,
    ) -> Result<DictDataDetail, DictError> {
        // 检查字典类型是否存在
        let dict_type_exists = dict_type::Entity::find_by_id(param.type_id)
            .count(db)
            .await
            .map_err(|e| DictError::DatabaseError(e.to_string()))?;
        
        if dict_type_exists == 0 {
            return Err(DictError::NotFound("字典类型不存在".to_string()));
        }
        
        let now = chrono::Utc::now().naive_utc();
        
        let dict_data = dict_data::ActiveModel {
            label: Set(param.label),
            value: Set(param.value),
            sort: Set(param.sort),
            type_id: Set(param.type_id),
            type_code: Set(param.type_code),
            is_default: Set(param.is_default.to_uppercase()),
            status: Set(param.status),
            remark: Set(param.remark),
            created_time: Set(now),
            updated_time: Set(None),
            ..Default::default()
        };
        
        let result = dict_data
            .insert(db)
            .await
            .map_err(|e| DictError::DatabaseError(e.to_string()))?;
        
        Ok(DictDataDetail::from(result))
    }
    
    /// 更新字典数据
    pub async fn update(
        db: &DatabaseConnection,
        id: i64,
        param: UpdateDictDataParam,
    ) -> Result<u64, DictError> {
        // 检查是否存在
        let dict_data = dict_data::Entity::find_by_id(id)
            .one(db)
            .await
            .map_err(|e| DictError::DatabaseError(e.to_string()))?
            .ok_or(DictError::NotFound("字典数据不存在".to_string()))?;
        
        let now = chrono::Utc::now().naive_utc();
        
        let mut dict_data: dict_data::ActiveModel = dict_data.into();
        dict_data.label = Set(param.label);
        dict_data.value = Set(param.value);
        dict_data.sort = Set(param.sort);
        dict_data.is_default = Set(param.is_default.to_uppercase());
        dict_data.status = Set(param.status);
        dict_data.remark = Set(param.remark);
        dict_data.updated_time = Set(Some(now));
        
        dict_data
            .update(db)
            .await
            .map_err(|e| DictError::DatabaseError(e.to_string()))?;
        
        Ok(1)
    }
    
    /// 批量删除字典数据
    pub async fn delete_batch(
        db: &DatabaseConnection,
        ids: Vec<i64>,
    ) -> Result<u64, DictError> {
        if ids.is_empty() {
            return Ok(0);
        }
        
        let result = dict_data::Entity::delete_many()
            .filter(dict_data::Column::Id.is_in(ids))
            .exec(db)
            .await
            .map_err(|e| DictError::DatabaseError(e.to_string()))?;
        
        Ok(result.rows_affected)
    }
}
