use tracing::{info, warn, error, debug};

/// 数据字典服务实现
/// 提供数据字典的增删改查、批量操作、类型分组等功能

use crate::app::dict_data::dto::create_dict_data::{
    CreateDictDataRequest as CreateDictDataRequestDTO,
    CreateDictDataResponse,
    UpdateDictDataRequest as UpdateDictDataRequestDTO,
    UpdateDictDataResponse,
};
use crate::app::dict_data::dto::dict_data_api::{
    DictDataListItem, DictDataDetailResponse, DictDataPaginationResponse,
};
use crate::app::dict_data::dto::{
    DictDataQuery, DictDataListResponse, DictTypeQuery, DictDataList, DictItem,
    DictTypeStatistics, DictDataGroupStatistics,
};
use crate::common::exception::{AppError, ErrorCode};
use crate::database::DatabaseManager;
use crate::database::entity::dict_data::{self, DictStatus};
use sea_orm::{DatabaseConnection, DbErr, EntityTrait, QueryFilter, ColumnTrait, Select, Order};
use std::collections::HashMap;
use std::sync::Arc;

/// 数据字典服务
pub struct DictDataService {
    pub db: DatabaseConnection,
    /// 字典缓存（字典类型 -> 字典项列表）
    dict_cache: Arc<std::sync::Mutex<HashMap<String, Vec<DictItem>>>>,
}

impl DictDataService {
    /// 创建新的数据字典服务
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            db,
            dict_cache: Arc::new(std::sync::Mutex::new(HashMap::new())),
        }
    }

    /// 创建数据字典
    pub async fn create_dict_data(
        &self,
        request: &CreateDictDataRequestDTO,
    ) -> Result<CreateDictDataResponse, AppError> {
        // 检查同一字典类型下，字典键值是否已存在
        let existing = dict_data::Entity::find_by_dict_value(&request.dict_type, &request.dict_value)
            .one(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to check dict data existence: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, "Failed to check dict data")
            })?;

        if existing.is_some() {
            return Err(AppError::new(
                ErrorCode::Conflict,
                "Dict value already exists in this dict type",
            ));
        }

        // 检查同一字典类型下是否有多个默认
        if request.is_default.unwrap_or(0) == 1 {
            let default_count = dict_data::Entity::find_by_dict_type_and_status(
                &request.dict_type,
                DictStatus::Normal,
            )
            .filter(dict_data::Column::IsDefault.eq(1))
            .count(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to count default dict data: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, "Failed to count default dict data")
            })?;

            if default_count > 0 {
                return Err(AppError::new(
                    ErrorCode::Conflict,
                    "This dict type already has a default value",
                ));
            }
        }

        let active_model = dict_data::ActiveModel {
            dict_code: Default::default(),
            dict_sort: sea_orm::Set(request.dict_sort),
            dict_label: sea_orm::Set(request.dict_label.clone()),
            dict_value: sea_orm::Set(request.dict_value.clone()),
            dict_type: sea_orm::Set(request.dict_type.clone()),
            css_class: sea_orm::Set(request.css_class.clone()),
            list_class: sea_orm::Set(request.list_class.clone()),
            is_default: sea_orm::Set(request.is_default.unwrap_or(0)),
            status: sea_orm::Set(DictStatus::from(request.status)),
            remark: sea_orm::Set(request.remark.clone()),
            created_time: Default::default(),
            updated_time: Default::default(),
        };

        let saved_dict = active_model.insert(&self.db).await.map_err(|e| {
            error!("Failed to create dict data: {:?}", e);
            AppError::with_message(ErrorCode::DatabaseError, "Failed to create dict data")
        })?;

        // 更新缓存
        self.refresh_cache_for_type(&saved_dict.dict_type).await?;

        Ok(CreateDictDataResponse {
            dict_code: saved_dict.dict_code,
            dict_label: saved_dict.dict_label,
            dict_value: saved_dict.dict_value,
            dict_type: saved_dict.dict_type,
            created_time: saved_dict.created_time,
        })
    }

    /// 更新数据字典
    pub async fn update_dict_data(
        &self,
        request: &UpdateDictDataRequestDTO,
    ) -> Result<UpdateDictDataResponse, AppError> {
        let existing_dict = dict_data::Entity::find_by_id(request.dict_code)
            .one(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to find dict data: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, "Failed to find dict data")
            })?
            .ok_or_else(|| AppError::with_message(ErrorCode::NotFound, "Dict data not found"))?;

        // 检查同一字典类型下，字典键值是否已被其他字典使用
        if request.dict_value != existing_dict.dict_value || request.dict_type != existing_dict.dict_type {
            let existing = dict_data::Entity::find_by_dict_value(&request.dict_type, &request.dict_value)
                .one(&self.db)
                .await
                .map_err(|e| {
                    error!("Failed to check dict data existence: {:?}", e);
                    AppError::with_message(ErrorCode::DatabaseError, "Failed to check dict data")
                })?;

            if existing.is_some() {
                return Err(AppError::new(
                    ErrorCode::Conflict,
                    "Dict value already exists in this dict type",
                ));
            }
        }

        // 如果设置为默认，检查同一字典类型下是否已有其他默认
        if request.is_default.unwrap_or(0) == 1 {
            let default_count = dict_data::Entity::find_by_dict_type_and_status(
                &request.dict_type,
                DictStatus::from(request.status),
            )
            .filter(dict_data::Column::IsDefault.eq(1))
            .filter(dict_data::Column::DictCode.ne(request.dict_code))
            .count(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to count default dict data: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, "Failed to count default dict data")
            })?;

            if default_count > 0 {
                return Err(AppError::new(
                    ErrorCode::Conflict,
                    "This dict type already has a default value",
                ));
            }
        }

        let mut active_model = existing_dict.into_active_model();
        active_model.dict_sort = sea_orm::Set(request.dict_sort);
        active_model.dict_label = sea_orm::Set(request.dict_label.clone());
        active_model.dict_value = sea_orm::Set(request.dict_value.clone());
        active_model.dict_type = sea_orm::Set(request.dict_type.clone());
        active_model.css_class = sea_orm::Set(request.css_class.clone());
        active_model.list_class = sea_orm::Set(request.list_class.clone());
        active_model.is_default = sea_orm::Set(request.is_default.unwrap_or(0));
        active_model.status = sea_orm::Set(DictStatus::from(request.status));
        active_model.remark = sea_orm::Set(request.remark.clone());

        let updated_dict = active_model.update(&self.db).await.map_err(|e| {
            error!("Failed to update dict data: {:?}", e);
            AppError::new(ErrorCode::DatabaseError, "Failed to update dict data")
        })?;

        // 更新缓存（可能涉及两个字典类型）
        self.refresh_cache_for_type(&updated_dict.dict_type).await?;
        if request.dict_type != existing_dict.dict_type {
            self.refresh_cache_for_type(&existing_dict.dict_type).await?;
        }

        Ok(UpdateDictDataResponse {
            dict_code: updated_dict.dict_code,
            dict_label: updated_dict.dict_label,
            updated_time: updated_dict.updated_time,
        })
    }

    /// 删除数据字典（批量）
    pub async fn delete_dict_datas(&self, dict_codes: &[i64]) -> Result<(), AppError> {
        if dict_codes.is_empty() {
            return Ok(());
        }

        // 批量查询字典信息
        let dicts = dict_data::Entity::find()
            .filter(dict_data::Column::DictCode.is_in(dict_codes.to_vec()))
            .all(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to find dict datas for deletion: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to find dict datas")
            })?;

        if dicts.is_empty() {
            return Err(AppError::with_message(ErrorCode::NotFound, "Dict datas not found"));
        }

        // 记录所有涉及的字典类型，用于更新缓存
        let mut affected_types = std::collections::HashSet::new();

        // 批量删除
        for dict in dicts {
            affected_types.insert(dict.dict_type.clone());

            let mut active_model = dict.into_active_model();
            active_model
                .updated_time
                .set(sea_orm::Set(chrono::Utc::now().naive_utc()));

            active_model.delete(&self.db).await.map_err(|e| {
                error!("Failed to delete dict data: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, "Failed to delete dict data")
            })?;
        }

        // 更新缓存
        for dict_type in affected_types {
            self.refresh_cache_for_type(dict_type.as_str()).await?;
        }

        Ok(())
    }

    /// 获取字典列表（分页）
    pub async fn get_dict_data_list(
        &self,
        query: &DictDataQuery,
    ) -> Result<DictDataListResponse, AppError> {
        let mut select = dict_data::Entity::find();

        // 添加查询条件
        if let Some(dict_label) = &query.dict_label {
            select = select.filter(dict_data::Column::DictLabel.like(format!("%{}%", dict_label)));
        }

        if let Some(dict_value) = &query.dict_value {
            select = select.filter(dict_data::Column::DictValue.like(format!("%{}%", dict_value)));
        }

        if let Some(dict_type) = &query.dict_type {
            select = select.filter(dict_data::Column::DictType.like(format!("%{}%", dict_type)));
        }

        if let Some(status) = query.status {
            select = select.filter(dict_data::Column::Status.eq(status));
        }

        // 按字典排序和字典编码倒序
        select = select
            .order_by(dict_data::Column::DictSort, Order::Asc)
            .order_by(dict_data::Column::DictCode, Order::Desc);

        // 分页
        let page_size = query.page_size.unwrap_or(20);
        let page_num = query.page_num.unwrap_or(1);
        let offset = (page_num - 1) * page_size;

        let dicts = select
            .offset(offset as u64)
            .limit(page_size as u64)
            .all(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to query dict datas: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, "Failed to query dict datas")
            })?;

        let total = dict_data::Entity::find()
            .count(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to count dict datas: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to count dict datas")
            })?;

        let list = dicts
            .into_iter()
            .map(|d| {
                let is_default_name = if d.is_default == 1 { "是" } else { "否" };
                let status_name = match DictStatus::from(d.status) {
                    DictStatus::Normal => "正常",
                    DictStatus::Disabled => "停用",
                };

                DictDataListItem {
                    dict_code: d.dict_code,
                    dict_sort: d.dict_sort,
                    dict_label: d.dict_label,
                    dict_value: d.dict_value,
                    dict_type: d.dict_type.clone(),
                    dict_type_name: d.dict_type,
                    css_class: d.css_class,
                    list_class: d.list_class,
                    is_default: d.is_default,
                    is_default_name: is_default_name.to_string(),
                    status: d.status,
                    status_name: status_name.to_string(),
                    remark: d.remark,
                    created_time: d.created_time,
                    updated_time: d.updated_time,
                }
            })
            .collect();

        Ok(DictDataListResponse {
            list,
            total: total as usize,
            page_num,
            page_size,
        })
    }

    /// 获取字典详情
    pub async fn get_dict_data_detail(&self, dict_code: i64) -> Result<DictDataDetailResponse, AppError> {
        let d = dict_data::Entity::find_by_id(dict_code)
            .one(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to find dict data: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, "Failed to find dict data")
            })?
            .ok_or_else(|| AppError::with_message(ErrorCode::NotFound, "Dict data not found"))?;

        let is_default_name = if d.is_default == 1 { "是" } else { "否" };
        let status_name = match DictStatus::from(d.status) {
            DictStatus::Normal => "正常",
            DictStatus::Disabled => "停用",
        };

        Ok(DictDataDetailResponse {
            dict_code: d.dict_code,
            dict_sort: d.dict_sort,
            dict_label: d.dict_label,
            dict_value: d.dict_value,
            dict_type: d.dict_type.clone(),
            dict_type_name: d.dict_type,
            css_class: d.css_class,
            list_class: d.list_class,
            is_default: d.is_default,
            is_default_name: is_default_name.to_string(),
            status: d.status,
            status_name: status_name.to_string(),
            remark: d.remark,
            create_by: None,
            update_by: None,
            created_time: d.created_time,
            updated_time: d.updated_time,
        })
    }

    /// 根据字典类型获取字典项
    pub async fn get_dict_data_by_type(
        &self,
        query: &DictTypeQuery,
    ) -> Result<DictDataList, AppError> {
        // 先从缓存获取
        {
            let cache = self.dict_cache.lock().unwrap();
            if let Some(items) = cache.get(&query.dict_type) {
                return Ok(DictDataList {
                    dict_type: query.dict_type.clone(),
                    items: items.clone(),
                });
            }
        }

        // 从数据库获取
        let dicts = dict_data::Entity::find_by_dict_type_and_status(
            &query.dict_type,
            DictStatus::Normal,
        )
        .order_by(dict_data::Column::DictSort, Order::Asc)
        .order_by(dict_data::Column::DictCode, Order::Asc)
        .all(&self.db)
        .await
        .map_err(|e| {
            error!("Failed to query dict datas: {:?}", e);
            AppError::with_message(ErrorCode::DatabaseError, "Failed to query dict datas")
        })?;

        let items = dicts
            .into_iter()
            .map(|d| DictItem {
                label: d.dict_label,
                value: d.dict_value,
                list_class: d.list_class,
            })
            .collect();

        // 更新缓存
        {
            let mut cache = self.dict_cache.lock().unwrap();
            cache.insert(query.dict_type.clone(), items.clone());
        }

        Ok(DictDataList {
            dict_type: query.dict_type.clone(),
            items,
        })
    }

    /// 获取所有字典类型
    pub async fn get_all_dict_types(&self) -> Result<Vec<String>, AppError> {
        let dicts = dict_data::Entity::find()
            .filter(dict_data::Column::Status.eq(0))
            .all(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to query dict types: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, "Failed to query dict types")
            })?;

        let mut types = std::collections::HashSet::new();
        for dict in dicts {
            types.insert(dict.dict_type);
        }

        let mut result: Vec<String> = types.into_iter().collect();
        result.sort();

        Ok(result)
    }

    /// 获取字典类型统计
    pub async fn get_dict_type_statistics(&self) -> Result<Vec<DictTypeStatistics>, AppError> {
        let dicts = dict_data::Entity::find()
            .all(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to query dict datas: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, "Failed to query dict datas")
            })?;

        let mut stats_map: HashMap<String, (usize, usize, usize)> = HashMap::new();

        for dict in dicts {
            let entry = stats_map
                .entry(dict.dict_type.clone())
                .or_insert((0, 0, 0));

            entry.0 += 1; // total count

            if dict.status == 0 {
                entry.1 += 1; // normal count
            } else {
                entry.2 += 1; // disabled count
            }
        }

        let mut stats = Vec::new();
        for (dict_type, (total, normal, disabled)) in stats_map {
            stats.push(DictTypeStatistics {
                dict_type,
                dict_type_name: dict_type.clone(),
                count: total,
                normal_count: normal,
                disabled_count: disabled,
            });
        }

        Ok(stats)
    }

    /// 获取字典分组统计
    pub async fn get_dict_data_group_statistics(&self) -> Result<DictDataGroupStatistics, AppError> {
        let dicts = dict_data::Entity::find()
            .all(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to query dict datas: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, "Failed to query dict datas")
            })?;

        let mut default_count = 0;
        let mut non_default_count = 0;
        let mut normal_count = 0;
        let mut disabled_count = 0;

        for dict in dicts {
            if dict.is_default == 1 {
                default_count += 1;
            } else {
                non_default_count += 1;
            }

            if dict.status == 0 {
                normal_count += 1;
            } else {
                disabled_count += 1;
            }
        }

        Ok(DictDataGroupStatistics {
            default_count,
            non_default_count,
            normal_count,
            disabled_count,
            total_count: default_count + non_default_count,
        })
    }

    /// 刷新指定字典类型的缓存
    async fn refresh_cache_for_type(&self, dict_type: &str) -> Result<(), AppError> {
        let dicts = dict_data::Entity::find_by_dict_type_and_status(
            dict_type,
            DictStatus::Normal,
        )
        .order_by(dict_data::Column::DictSort, Order::Asc)
        .order_by(dict_data::Column::DictCode, Order::Asc)
        .all(&self.db)
        .await
        .map_err(|e| {
            error!("Failed to query dict datas for cache: {:?}", e);
            AppError::with_message(ErrorCode::DatabaseError, "Failed to query dict datas")
        })?;

        let items = dicts
            .into_iter()
            .map(|d| DictItem {
                label: d.dict_label,
                value: d.dict_value,
                list_class: d.list_class,
            })
            .collect();

        let mut cache = self.dict_cache.lock().unwrap();
        cache.insert(dict_type.to_string(), items);

        Ok(())
    }

    /// 初始化缓存
    pub async fn init_cache(&self) -> Result<(), AppError> {
        let dicts = dict_data::Entity::find()
            .filter(dict_data::Column::Status.eq(0))
            .all(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to query dict datas for cache: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, "Failed to query dict datas")
            })?;

        let mut cache = self.dict_cache.lock().unwrap();
        cache.clear();

        // 按字典类型分组
        let mut dict_map: HashMap<String, Vec<DictItem>> = HashMap::new();

        for dict in dicts {
            let item = DictItem {
                label: dict.dict_label,
                value: dict.dict_value,
                list_class: dict.list_class,
            };

            dict_map
                .entry(dict.dict_type.clone())
                .or_insert_with(Vec::new)
                .push(item);
        }

        // 对每个字典类型的字典项进行排序
        for (dict_type, items) in &mut dict_map {
            // 这里需要重新从数据库获取排序后的数据
            // 简化处理，直接使用已有数据
        }

        cache.extend(dict_map);

        info!("Initialized dict data cache with {} types", cache.len());

        Ok(())
    }

    /// 清空缓存
    pub fn clear_cache(&self) {
        let mut cache = self.dict_cache.lock().unwrap();
        cache.clear();
        info!("Cleared dict data cache");
    }
}
