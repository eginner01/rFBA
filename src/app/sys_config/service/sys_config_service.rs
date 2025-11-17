use tracing::{info, warn, error, debug};

/// 系统配置服务实现
/// 提供系统配置的增删改查、批量操作、类型转换等功能

use crate::app::sys_config::dto::{
    CreateSysConfigRequest, CreateSysConfigResponse, UpdateSysConfigRequest,
    UpdateSysConfigResponse, SysConfigQuery, SysConfigListResponse, SysConfigListItem,
    SysConfigDetailResponse, SysConfigKeyQuery, SysConfigKeyValue, SysConfigTypeStatistics,
    SysConfigGroupStatistics,
};
use crate::common::exception::{AppError, ErrorCode};
use crate::database::DatabaseManager;
use crate::database::entity::sys_config;
use sea_orm::{DatabaseConnection, DbErr, EntityTrait, QueryFilter, ColumnTrait, Select, Order};
use std::collections::HashMap;
use std::sync::Arc;

/// 系统配置服务
pub struct SysConfigService {
    db: DatabaseConnection,
    /// 配置缓存（键名 -> 配置值）
    config_cache: Arc<std::sync::Mutex<HashMap<String, String>>>,
}

impl SysConfigService {
    /// 创建新的系统配置服务
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            db,
            config_cache: Arc::new(std::sync::Mutex::new(HashMap::new())),
        }
    }

    /// 创建系统配置
    pub async fn create_config(
        &self,
        request: &CreateSysConfigRequest,
    ) -> Result<CreateSysConfigResponse, AppError> {
        // 检查配置键名是否已存在
        let existing = sys_config::Entity::find_by_config_key(&request.config_key)
            .one(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to check config key existence: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to check config key")
            })?;

        if existing.is_some() {
            return Err(AppError::new(
                ErrorCode::Conflict,
                "Config key already exists",
            ));
        }

        let active_model = sys_config::ActiveModel {
            id: Default::default(),
            config_name: sea_orm::Set(request.config_name.clone()),
            config_key: sea_orm::Set(request.config_key.clone()),
            config_value: sea_orm::Set(request.config_value.clone()),
            config_type: sea_orm::Set(request.config_type),
            is_system: sea_orm::Set(0), // 默认非系统内置
            remark: sea_orm::Set(request.remark.clone()),
            created_time: Default::default(),
            updated_time: Default::default(),
        };

        let saved_config = active_model.insert(&self.db).await.map_err(|e| {
            error!("Failed to create config: {:?}", e);
            AppError::new(ErrorCode::DatabaseError, "Failed to create config")
        })?;

        // 更新缓存
        {
            let mut cache = self.config_cache.lock().unwrap();
            cache.insert(
                saved_config.config_key.clone(),
                saved_config.config_value.clone(),
            );
        }

        Ok(CreateSysConfigResponse {
            id: saved_config.id,
            config_name: saved_config.config_name,
            config_key: saved_config.config_key,
            created_time: saved_config.created_time,
        })
    }

    /// 更新系统配置
    pub async fn update_config(
        &self,
        request: &UpdateSysConfigRequest,
    ) -> Result<UpdateSysConfigResponse, AppError> {
        let existing_config = sys_config::Entity::find_by_id(request.id)
            .one(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to find config: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to find config")
            })?
            .ok_or_else(|| AppError::new(ErrorCode::NotFound, "Config not found"))?;

        // 检查是否为系统内置配置
        if existing_config.is_system == 1 {
            return Err(AppError::new(
                ErrorCode::Forbidden,
                "Cannot update system config",
            ));
        }

        // 检查配置键名是否已被其他配置使用
        if request.config_key != existing_config.config_key {
            let existing = sys_config::Entity::find_by_config_key(&request.config_key)
                .one(&self.db)
                .await
                .map_err(|e| {
                    error!("Failed to check config key existence: {:?}", e);
                    AppError::new(ErrorCode::DatabaseError, "Failed to check config key")
                })?;

            if existing.is_some() {
                return Err(AppError::new(
                    ErrorCode::Conflict,
                    "Config key already exists",
                ));
            }
        }

        let mut active_model = existing_config.into_active_model();
        active_model.config_name = sea_orm::Set(request.config_name.clone());
        active_model.config_key = sea_orm::Set(request.config_key.clone());
        active_model.config_value = sea_orm::Set(request.config_value.clone());
        active_model.config_type = sea_orm::Set(request.config_type);
        active_model.remark = sea_orm::Set(request.remark.clone());

        let updated_config = active_model.update(&self.db).await.map_err(|e| {
            error!("Failed to update config: {:?}", e);
            AppError::new(ErrorCode::DatabaseError, "Failed to update config")
        })?;

        // 更新缓存
        {
            let mut cache = self.config_cache.lock().unwrap();
            // 删除旧键名
            cache.remove(&existing_config.config_key);
            // 添加新键名
            cache.insert(
                updated_config.config_key.clone(),
                updated_config.config_value.clone(),
            );
        }

        Ok(UpdateSysConfigResponse {
            id: updated_config.id,
            config_name: updated_config.config_name,
            updated_time: updated_config.updated_time,
        })
    }

    /// 删除系统配置（批量）
    pub async fn delete_configs(&self, ids: &[i64]) -> Result<(), AppError> {
        if ids.is_empty() {
            return Ok(());
        }

        // 批量查询配置信息，检查是否为系统内置
        let configs = sys_config::Entity::find()
            .filter(sys_config::Column::Id.is_in(ids.to_vec()))
            .all(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to find configs for deletion: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to find configs")
            })?;

        if configs.is_empty() {
            return Err(AppError::new(ErrorCode::NotFound, "Configs not found"));
        }

        // 检查是否有系统内置配置
        let system_configs: Vec<_> = configs
            .iter()
            .filter(|c| c.is_system == 1)
            .collect();

        if !system_configs.is_empty() {
            return Err(AppError::new(
                ErrorCode::Forbidden,
                "Cannot delete system configs",
            ));
        }

        // 批量删除（软删除）
        for config in configs {
            let mut active_model = config.into_active_model();
            active_model
                .updated_time
                .set(sea_orm::Set(chrono::Utc::now().naive_utc()));

            active_model.update(&self.db).await.map_err(|e| {
                error!("Failed to delete config: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to delete config")
            })?;

            // 从缓存中删除
            {
                let mut cache = self.config_cache.lock().unwrap();
                cache.remove(&config.config_key);
            }
        }

        Ok(())
    }

    /// 获取配置列表（分页）
    pub async fn get_config_list(
        &self,
        query: &SysConfigQuery,
    ) -> Result<SysConfigListResponse, AppError> {
        let mut select = sys_config::Entity::find();

        // 添加查询条件
        if let Some(config_name) = &query.config_name {
            select = select.filter(sys_config::Column::ConfigName.like(format!(
                "%{}%",
                config_name
            )));
        }

        if let Some(config_key) = &query.config_key {
            select = select.filter(sys_config::Column::ConfigKey.like(format!(
                "%{}%",
                config_key
            )));
        }

        if let Some(config_type) = query.config_type {
            select = select.filter(sys_config::Column::ConfigType.eq(config_type));
        }

        if let Some(is_system) = query.is_system {
            select = select.filter(sys_config::Column::IsSystem.eq(is_system));
        }

        // 按创建时间倒序
        select = select.order_by(sys_config::Column::CreatedTime, Order::Desc);

        // 分页
        let page_size = query.page_size.unwrap_or(20);
        let page_num = query.page_num.unwrap_or(1);
        let offset = (page_num - 1) * page_size;

        let configs = select
            .offset(offset as u64)
            .limit(page_size as u64)
            .all(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to query configs: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to query configs")
            })?;

        let total = sys_config::Entity::find()
            .count(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to count configs: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to count configs")
            })?;

        let list = configs
            .into_iter()
            .map(|c| {
                let config_type_name = match crate::database::entity::sys_config::SysConfigType::from_i32(c.config_type) {
                    Some(ct) => ct.get_name().to_string(),
                    None => "未知".to_string(),
                };

                let is_system_name = if c.is_system == 1 {
                    "是"
                } else {
                    "否"
                };

                SysConfigListItem {
                    id: c.id,
                    config_name: c.config_name,
                    config_key: c.config_key,
                    config_value: c.config_value,
                    config_type: c.config_type,
                    config_type_name,
                    is_system: c.is_system,
                    is_system_name: is_system_name.to_string(),
                    remark: c.remark,
                    created_time: c.created_time,
                    updated_time: c.updated_time,
                }
            })
            .collect();

        Ok(SysConfigListResponse {
            list,
            total: total as usize,
            page_num,
            page_size,
        })
    }

    /// 获取配置详情
    pub async fn get_config_detail(&self, id: i64) -> Result<SysConfigDetailResponse, AppError> {
        let c = sys_config::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to find config: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to find config")
            })?
            .ok_or_else(|| AppError::new(ErrorCode::NotFound, "Config not found"))?;

        let config_type_name = match crate::database::entity::sys_config::SysConfigType::from_i32(c.config_type) {
            Some(ct) => ct.get_name().to_string(),
            None => "未知".to_string(),
        };

        let is_system_name = if c.is_system == 1 { "是" } else { "否" };

        Ok(SysConfigDetailResponse {
            id: c.id,
            config_name: c.config_name,
            config_key: c.config_key,
            config_value: c.config_value,
            config_type: c.config_type,
            config_type_name,
            is_system: c.is_system,
            is_system_name: is_system_name.to_string(),
            remark: c.remark,
            create_by: None,
            update_by: None,
            created_time: c.created_time,
            updated_time: c.updated_time,
        })
    }

    /// 批量获取配置值
    pub async fn get_config_values(
        &self,
        query: &SysConfigKeyQuery,
    ) -> Result<Vec<SysConfigKeyValue>, AppError> {
        if query.config_keys.is_empty() {
            return Ok(Vec::new());
        }

        let configs = sys_config::Entity::find()
            .filter(sys_config::Column::ConfigKey.is_in(query.config_keys.clone()))
            .all(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to query configs: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to query configs")
            })?;

        let mut result = Vec::new();
        for config in configs {
            let config_type_name = match crate::database::entity::sys_config::SysConfigType::from_i32(config.config_type) {
                Some(ct) => ct.get_name().to_string(),
                None => "未知".to_string(),
            };

            result.push(SysConfigKeyValue {
                config_key: config.config_key,
                config_value: config.config_value,
                config_type: config.config_type,
                config_type_name,
            });

            // 更新缓存
            {
                let mut cache = self.config_cache.lock().unwrap();
                cache.insert(config.config_key.clone(), config.config_value.clone());
            }
        }

        Ok(result)
    }

    /// 获取配置值（根据键名）
    pub async fn get_config_value(&self, config_key: &str) -> Result<Option<String>, AppError> {
        // 先从缓存获取
        {
            let cache = self.config_cache.lock().unwrap();
            if let Some(value) = cache.get(config_key) {
                return Ok(Some(value.clone()));
            }
        }

        // 从数据库获取
        let config = sys_config::Entity::find_by_config_key(config_key)
            .one(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to find config: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to find config")
            })?;

        if let Some(c) = config {
            // 更新缓存
            {
                let mut cache = self.config_cache.lock().unwrap();
                cache.insert(config_key.to_string(), c.config_value.clone());
            }
            Ok(Some(c.config_value))
        } else {
            Ok(None)
        }
    }

    /// 获取配置类型统计
    pub async fn get_config_type_statistics(&self) -> Result<Vec<SysConfigTypeStatistics>, AppError> {
        let configs = sys_config::Entity::find()
            .all(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to query configs: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to query configs")
            })?;

        let mut stats_map: HashMap<i32, usize> = HashMap::new();

        for config in configs {
            *stats_map.entry(config.config_type).or_insert(0) += 1;
        }

        let mut stats = Vec::new();
        for (config_type, count) in stats_map {
            let config_type_name = match crate::database::entity::sys_config::SysConfigType::from_i32(config_type) {
                Some(ct) => ct.get_name().to_string(),
                None => "未知".to_string(),
            };

            stats.push(SysConfigTypeStatistics {
                config_type,
                config_type_name,
                count,
            });
        }

        Ok(stats)
    }

    /// 获取配置分组统计
    pub async fn get_config_group_statistics(&self) -> Result<SysConfigGroupStatistics, AppError> {
        let system_count = sys_config::Entity::find_system()
            .count(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to count system configs: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to count configs")
            })?;

        let custom_count = sys_config::Entity::find_non_system()
            .count(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to count custom configs: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to count configs")
            })?;

        Ok(SysConfigGroupStatistics {
            system_count: system_count as usize,
            custom_count: custom_count as usize,
            total_count: (system_count + custom_count) as usize,
        })
    }

    /// 初始化缓存
    pub async fn init_cache(&self) -> Result<(), AppError> {
        let configs = sys_config::Entity::find()
            .all(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to query configs for cache: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to query configs")
            })?;

        let mut cache = self.config_cache.lock().unwrap();
        cache.clear();

        for config in configs {
            cache.insert(config.config_key.clone(), config.config_value.clone());
        }

        info!("Initialized config cache with {} items", cache.len());

        Ok(())
    }

    /// 清空缓存
    pub fn clear_cache(&self) {
        let mut cache = self.config_cache.lock().unwrap();
        cache.clear();
        info!("Cleared config cache");
    }
}
