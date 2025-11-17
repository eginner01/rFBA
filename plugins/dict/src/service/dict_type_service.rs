//! 字典类型服务层

use sea_orm::*;
use crate::entity::{dict_type, dict_data};
use crate::dto::*;
use crate::error::DictError;

/// 字典类型服务
pub struct DictTypeService;

impl DictTypeService {
    /// 获取所有字典类型
    pub async fn get_all(db: &DatabaseConnection) -> Result<Vec<DictTypeDetail>, DictError> {
        let types = dict_type::Entity::find()
            .order_by_asc(dict_type::Column::Id)
            .all(db)
            .await
            .map_err(|e| DictError::DatabaseError(e.to_string()))?;
        
        Ok(types.into_iter().map(DictTypeDetail::from).collect())
    }
    
    /// 根据ID获取字典类型
    pub async fn get_by_id(
        db: &DatabaseConnection,
        id: i64,
    ) -> Result<DictTypeDetail, DictError> {
        let dict_type = dict_type::Entity::find_by_id(id)
            .one(db)
            .await
            .map_err(|e| DictError::DatabaseError(e.to_string()))?
            .ok_or(DictError::NotFound("字典类型不存在".to_string()))?;
        
        Ok(DictTypeDetail::from(dict_type))
    }
    
    /// 分页查询字典类型
    pub async fn get_list(
        db: &DatabaseConnection,
        query: DictTypeQuery,
        pagination: PaginationQuery,
    ) -> Result<PageData<DictTypeDetail>, DictError> {
        let mut select = dict_type::Entity::find();
        
        // 构建查询条件
        if let Some(name) = &query.name {
            select = select.filter(dict_type::Column::Name.contains(name));
        }
        if let Some(code) = &query.code {
            select = select.filter(dict_type::Column::Code.contains(code));
        }
        if let Some(status) = query.status {
            select = select.filter(dict_type::Column::Status.eq(status));
        }
        
        // 查询总数
        let total = select
            .clone()
            .count(db)
            .await
            .map_err(|e| DictError::DatabaseError(e.to_string()))?;
        
        // 分页查询
        let items = select
            .order_by_asc(dict_type::Column::Id)
            .offset(pagination.offset())
            .limit(pagination.limit())
            .all(db)
            .await
            .map_err(|e| DictError::DatabaseError(e.to_string()))?
            .into_iter()
            .map(DictTypeDetail::from)
            .collect();
        
        Ok(PageData::new(items, total, pagination.page, pagination.size))
    }
    
    /// 创建字典类型
    pub async fn create(
        db: &DatabaseConnection,
        param: CreateDictTypeParam,
    ) -> Result<DictTypeDetail, DictError> {
        // 检查编码是否已存在
        let exists = dict_type::Entity::find()
            .filter(dict_type::Column::Code.eq(&param.code))
            .count(db)
            .await
            .map_err(|e| DictError::DatabaseError(e.to_string()))?;
        
        if exists > 0 {
            return Err(DictError::AlreadyExists(format!(
                "字典编码 {} 已存在",
                param.code
            )));
        }
        
        let now = chrono::Utc::now().naive_utc();
        
        let dict_type = dict_type::ActiveModel {
            name: Set(param.name),
            code: Set(param.code),
            status: Set(param.status),
            remark: Set(param.remark),
            created_time: Set(now),
            updated_time: Set(None),
            ..Default::default()
        };
        
        let result = dict_type
            .insert(db)
            .await
            .map_err(|e| DictError::DatabaseError(e.to_string()))?;
        
        Ok(DictTypeDetail::from(result))
    }
    
    /// 更新字典类型
    pub async fn update(
        db: &DatabaseConnection,
        id: i64,
        param: UpdateDictTypeParam,
    ) -> Result<u64, DictError> {
        // 检查是否存在
        let dict_type = dict_type::Entity::find_by_id(id)
            .one(db)
            .await
            .map_err(|e| DictError::DatabaseError(e.to_string()))?
            .ok_or(DictError::NotFound("字典类型不存在".to_string()))?;
        
        let now = chrono::Utc::now().naive_utc();
        
        let mut dict_type: dict_type::ActiveModel = dict_type.into();
        dict_type.name = Set(param.name);
        dict_type.status = Set(param.status);
        dict_type.remark = Set(param.remark);
        dict_type.updated_time = Set(Some(now));
        
        dict_type
            .update(db)
            .await
            .map_err(|e| DictError::DatabaseError(e.to_string()))?;
        
        Ok(1)
    }
    
    /// 批量删除字典类型
    pub async fn delete_batch(
        db: &DatabaseConnection,
        ids: Vec<i64>,
    ) -> Result<u64, DictError> {
        if ids.is_empty() {
            return Ok(0);
        }
        
        // 检查是否有关联的字典数据
        for id in &ids {
            let count = dict_data::Entity::find()
                .filter(dict_data::Column::TypeId.eq(*id))
                .count(db)
                .await
                .map_err(|e| DictError::DatabaseError(e.to_string()))?;
            
            if count > 0 {
                return Err(DictError::OperationFailed(
                    "存在关联的字典数据，无法删除".to_string(),
                ));
            }
        }
        
        let result = dict_type::Entity::delete_many()
            .filter(dict_type::Column::Id.is_in(ids))
            .exec(db)
            .await
            .map_err(|e| DictError::DatabaseError(e.to_string()))?;
        
        Ok(result.rows_affected)
    }
}
