use tracing::{info, warn, error, debug};

/// 日志级别服务实现
/// 提供日志级别的增删改查、批量操作、统计等功能

use crate::app::log_level::dto::{
    CreateLogLevelRequest, CreateLogLevelResponse, UpdateLogLevelRequest,
    UpdateLogLevelResponse, LogLevelQuery, LogLevelListResponse, LogLevelListItem,
    LogLevelDetailResponse, EnabledLogLevelQuery, EnabledLogLevelList, LogLevelItem,
    SystemLevelStatistics, StatusLevelStatistics,
};
use crate::common::exception::{AppError, ErrorCode};
use crate::database::DatabaseManager;
use crate::database::entity::log_level::{self, LogLevelStatus};
use sea_orm::{DatabaseConnection, DbErr, EntityTrait, QueryFilter, ColumnTrait, Select, Order};
use std::collections::HashMap;
use std::sync::Arc;

/// 日志级别服务
pub struct LogLevelService {
    db: DatabaseConnection,
    /// 级别缓存（级别值 -> 级别名称）
    level_cache: Arc<std::sync::Mutex<HashMap<i32, String>>>,
}

impl LogLevelService {
    /// 创建新的日志级别服务
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            db,
            level_cache: Arc::new(std::sync::Mutex::new(HashMap::new())),
        }
    }

    /// 创建日志级别
    pub async fn create_log_level(
        &self,
        request: &CreateLogLevelRequest,
        create_by: &str,
    ) -> Result<CreateLogLevelResponse, AppError> {
        // 检查级别值是否已存在
        let existing = log_level::Entity::find_by_level_value(request.level_value)
            .one(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to check level value existence: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to check level value")
            })?;

        if existing.is_some() {
            return Err(AppError::new(
                ErrorCode::Conflict,
                "Level value already exists",
            ));
        }

        let active_model = log_level::ActiveModel {
            level_id: Default::default(),
            level_name: sea_orm::Set(request.level_name.clone()),
            level_value: sea_orm::Set(request.level_value),
            description: sea_orm::Set(request.description.clone()),
            is_system: sea_orm::Set(request.is_system.unwrap_or(0)),
            status: sea_orm::Set(LogLevelStatus::from(request.status)),
            create_by: sea_orm::Set(create_by.to_string()),
            created_time: Default::default(),
            update_by: sea_orm::Set(None),
            updated_time: Default::default(),
        };

        let saved_level = active_model.insert(&self.db).await.map_err(|e| {
            error!("Failed to create log level: {:?}", e);
            AppError::new(ErrorCode::DatabaseError, "Failed to create log level")
        })?;

        // 更新缓存
        {
            let mut cache = self.level_cache.lock().unwrap();
            if saved_level.status == 0 {
                cache.insert(saved_level.level_value, saved_level.level_name);
            }
        }

        Ok(CreateLogLevelResponse {
            level_id: saved_level.level_id,
            level_name: saved_level.level_name,
            level_value: saved_level.level_value,
            created_time: saved_level.created_time,
        })
    }

    /// 更新日志级别
    pub async fn update_log_level(
        &self,
        request: &UpdateLogLevelRequest,
        update_by: Option<&str>,
    ) -> Result<UpdateLogLevelResponse, AppError> {
        let existing_level = log_level::Entity::find_by_id(request.level_id)
            .one(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to find log level: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to find log level")
            })?
            .ok_or_else(|| AppError::new(ErrorCode::NotFound, "Log level not found"))?;

        // 检查是否为系统内置
        if existing_level.is_system == 1 {
            return Err(AppError::new(
                ErrorCode::Forbidden,
                "Cannot update system log level",
            ));
        }

        // 检查级别值是否已被其他级别使用
        if request.level_value != existing_level.level_value {
            let existing = log_level::Entity::find_by_level_value(request.level_value)
                .one(&self.db)
                .await
                .map_err(|e| {
                    error!("Failed to check level value existence: {:?}", e);
                    AppError::new(ErrorCode::DatabaseError, "Failed to check level value")
                })?;

            if existing.is_some() {
                return Err(AppError::new(
                    ErrorCode::Conflict,
                    "Level value already exists",
                ));
            }
        }

        let mut active_model = existing_level.into_active_model();
        active_model.level_name = sea_orm::Set(request.level_name.clone());
        active_model.level_value = sea_orm::Set(request.level_value);
        active_model.description = sea_orm::Set(request.description.clone());
        active_model.status = sea_orm::Set(LogLevelStatus::from(request.status));
        active_model.update_by = sea_orm::Set(update_by.map(|s| s.to_string()));

        let updated_level = active_model.update(&self.db).await.map_err(|e| {
            error!("Failed to update log level: {:?}", e);
            AppError::new(ErrorCode::DatabaseError, "Failed to update log level")
        })?;

        // 更新缓存
        {
            let mut cache = self.level_cache.lock().unwrap();
            // 删除旧级别值
            cache.remove(&existing_level.level_value);
            // 如果新状态为启用，添加新级别值
            if updated_level.status == 0 {
                cache.insert(updated_level.level_value, updated_level.level_name);
            }
        }

        Ok(UpdateLogLevelResponse {
            level_id: updated_level.level_id,
            level_name: updated_level.level_name,
            updated_time: updated_level.updated_time,
        })
    }

    /// 删除日志级别（批量）
    pub async fn delete_log_levels(&self, level_ids: &[i64]) -> Result<(), AppError> {
        if level_ids.is_empty() {
            return Ok(());
        }

        // 批量查询级别信息
        let levels = log_level::Entity::find()
            .filter(log_level::Column::LevelId.is_in(level_ids.to_vec()))
            .all(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to find log levels for deletion: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to find log levels")
            })?;

        if levels.is_empty() {
            return Err(AppError::new(ErrorCode::NotFound, "Log levels not found"));
        }

        // 检查是否有系统内置级别
        let system_levels: Vec<_> = levels.iter().filter(|l| l.is_system == 1).collect();

        if !system_levels.is_empty() {
            return Err(AppError::new(
                ErrorCode::Forbidden,
                "Cannot delete system log levels",
            ));
        }

        // 批量删除
        for level in levels {
            level.delete(&self.db).await.map_err(|e| {
                error!("Failed to delete log level: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to delete log level")
            })?;

            // 从缓存中删除
            {
                let mut cache = self.level_cache.lock().unwrap();
                cache.remove(&level.level_value);
            }
        }

        Ok(())
    }

    /// 获取级别列表（分页）
    pub async fn get_log_level_list(
        &self,
        query: &LogLevelQuery,
    ) -> Result<LogLevelListResponse, AppError> {
        let mut select = log_level::Entity::find();

        // 添加查询条件
        if let Some(level_name) = &query.level_name {
            select = select.filter(log_level::Column::LevelName.like(format!("%{}%", level_name)));
        }

        if let Some(level_value) = query.level_value {
            select = select.filter(log_level::Column::LevelValue.eq(level_value));
        }

        if let Some(is_system) = query.is_system {
            select = select.filter(log_level::Column::IsSystem.eq(is_system));
        }

        if let Some(status) = query.status {
            select = select.filter(log_level::Column::Status.eq(status));
        }

        if let Some(create_by) = &query.create_by {
            select = select.filter(log_level::Column::CreateBy.like(format!("%{}%", create_by)));
        }

        // 按级别值升序
        select = select.order_by(log_level::Column::LevelValue, Order::Asc);

        // 分页
        let page_size = query.page_size.unwrap_or(20);
        let page_num = query.page_num.unwrap_or(1);
        let offset = (page_num - 1) * page_size;

        let levels = select
            .offset(offset as u64)
            .limit(page_size as u64)
            .all(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to query log levels: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to query log levels")
            })?;

        let total = log_level::Entity::find()
            .count(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to count log levels: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to count log levels")
            })?;

        let list = levels
            .into_iter()
            .map(|l| {
                let is_system_name = if l.is_system == 1 { "是" } else { "否" };
                let status_name = match LogLevelStatus::from(l.status) {
                    LogLevelStatus::Enabled => "启用",
                    LogLevelStatus::Disabled => "禁用",
                };

                LogLevelListItem {
                    level_id: l.level_id,
                    level_name: l.level_name,
                    level_value: l.level_value,
                    description: l.description,
                    is_system: l.is_system,
                    is_system_name: is_system_name.to_string(),
                    status: l.status,
                    status_name: status_name.to_string(),
                    create_by: l.create_by,
                    created_time: l.created_time,
                    updated_time: l.updated_time,
                }
            })
            .collect();

        Ok(LogLevelListResponse {
            list,
            total: total as usize,
            page_num,
            page_size,
        })
    }

    /// 获取级别详情
    pub async fn get_log_level_detail(&self, level_id: i64) -> Result<LogLevelDetailResponse, AppError> {
        let l = log_level::Entity::find_by_id(level_id)
            .one(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to find log level: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to find log level")
            })?
            .ok_or_else(|| AppError::new(ErrorCode::NotFound, "Log level not found"))?;

        let is_system_name = if l.is_system == 1 { "是" } else { "否" };
        let status_name = match LogLevelStatus::from(l.status) {
            LogLevelStatus::Enabled => "启用",
            LogLevelStatus::Disabled => "禁用",
        };

        Ok(LogLevelDetailResponse {
            level_id: l.level_id,
            level_name: l.level_name,
            level_value: l.level_value,
            description: l.description,
            is_system: l.is_system,
            is_system_name: is_system_name.to_string(),
            status: l.status,
            status_name: status_name.to_string(),
            create_by: l.create_by,
            created_time: l.created_time,
            update_by: l.update_by,
            updated_time: l.updated_time,
        })
    }

    /// 获取启用的级别列表
    pub async fn get_enabled_log_levels(
        &self,
        query: &EnabledLogLevelQuery,
    ) -> Result<EnabledLogLevelList, AppError> {
        // 先从缓存获取
        if query.enabled_only {
            let cache = self.level_cache.lock().unwrap();
            if !cache.is_empty() {
                let mut levels: Vec<LogLevelItem> = cache
                    .iter()
                    .map(|(value, name)| LogLevelItem {
                        level_id: 0, // 缓存中不包含ID
                        level_name: name.clone(),
                        level_value: *value,
                        description: None,
                    })
                    .collect();

                // 按级别值排序
                levels.sort_by_key(|l| l.level_value);

                return Ok(EnabledLogLevelList { levels });
            }
        }

        // 从数据库获取
        let levels_query = if query.enabled_only {
            log_level::Entity::find_enabled()
        } else {
            log_level::Entity::find()
        };

        let levels = levels_query
            .order_by(log_level::Column::LevelValue, Order::Asc)
            .all(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to query log levels: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to query log levels")
            })?;

        let mut result_levels = Vec::new();
        let mut cache = self.level_cache.lock().unwrap();

        for level in levels {
            result_levels.push(LogLevelItem {
                level_id: level.level_id,
                level_name: level.level_name.clone(),
                level_value: level.level_value,
                description: level.description,
            });

            // 更新缓存
            if level.status == 0 {
                cache.insert(level.level_value, level.level_name);
            }
        }

        Ok(EnabledLogLevelList {
            levels: result_levels,
        })
    }

    /// 根据级别值获取级别名称
    pub async fn get_level_name(&self, level_value: i32) -> Result<Option<String>, AppError> {
        // 先从缓存获取
        {
            let cache = self.level_cache.lock().unwrap();
            if let Some(name) = cache.get(&level_value) {
                return Ok(Some(name.clone()));
            }
        }

        // 从数据库获取
        let level = log_level::Entity::find_by_level_value(level_value)
            .one(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to find log level: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to find log level")
            })?;

        if let Some(l) = level {
            // 更新缓存
            {
                let mut cache = self.level_cache.lock().unwrap();
                cache.insert(level_value, l.level_name.clone());
            }
            Ok(Some(l.level_name))
        } else {
            Ok(None)
        }
    }

    /// 启用级别
    pub async fn enable_level(&self, level_id: i64, update_by: Option<&str>) -> Result<(), AppError> {
        let existing_level = log_level::Entity::find_by_id(level_id)
            .one(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to find log level: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to find log level")
            })?
            .ok_or_else(|| AppError::new(ErrorCode::NotFound, "Log level not found"))?;

        if existing_level.status == 0 {
            return Err(AppError::new(
                ErrorCode::Conflict,
                "Log level is already enabled",
            ));
        }

        let mut active_model = existing_level.into_active_model();
        active_model.status = sea_orm::Set(LogLevelStatus::Enabled);
        active_model.update_by = sea_orm::Set(update_by.map(|s| s.to_string()));

        let updated_level = active_model.update(&self.db).await.map_err(|e| {
            error!("Failed to enable log level: {:?}", e);
            AppError::new(ErrorCode::DatabaseError, "Failed to enable log level")
        })?;

        // 更新缓存
        {
            let mut cache = self.level_cache.lock().unwrap();
            cache.insert(updated_level.level_value, updated_level.level_name);
        }

        Ok(())
    }

    /// 禁用级别
    pub async fn disable_level(&self, level_id: i64, update_by: Option<&str>) -> Result<(), AppError> {
        let existing_level = log_level::Entity::find_by_id(level_id)
            .one(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to find log level: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to find log level")
            })?
            .ok_or_else(|| AppError::new(ErrorCode::NotFound, "Log level not found"))?;

        if existing_level.status == 1 {
            return Err(AppError::new(
                ErrorCode::Conflict,
                "Log level is already disabled",
            ));
        }

        let mut active_model = existing_level.into_active_model();
        active_model.status = sea_orm::Set(LogLevelStatus::Disabled);
        active_model.update_by = sea_orm::Set(update_by.map(|s| s.to_string()));

        let updated_level = active_model.update(&self.db).await.map_err(|e| {
            error!("Failed to disable log level: {:?}", e);
            AppError::new(ErrorCode::DatabaseError, "Failed to disable log level")
        })?;

        // 从缓存中删除
        {
            let mut cache = self.level_cache.lock().unwrap();
            cache.remove(&updated_level.level_value);
        }

        Ok(())
    }

    /// 获取系统内置级别统计
    pub async fn get_system_level_statistics(&self) -> Result<SystemLevelStatistics, AppError> {
        let system_count = log_level::Entity::find_system()
            .count(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to count system log levels: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to count log levels")
            })?;

        let custom_count = log_level::Entity::find_non_system()
            .count(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to count custom log levels: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to count log levels")
            })?;

        Ok(SystemLevelStatistics {
            system_count: system_count as usize,
            custom_count: custom_count as usize,
            total_count: (system_count + custom_count) as usize,
        })
    }

    /// 获取状态级别统计
    pub async fn get_status_level_statistics(&self) -> Result<StatusLevelStatistics, AppError> {
        let enabled_count = log_level::Entity::find_enabled()
            .count(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to count enabled log levels: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to count log levels")
            })?;

        let disabled_count = log_level::Entity::find_by_status(LogLevelStatus::Disabled)
            .count(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to count disabled log levels: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to count log levels")
            })?;

        Ok(StatusLevelStatistics {
            enabled_count: enabled_count as usize,
            disabled_count: disabled_count as usize,
            total_count: (enabled_count + disabled_count) as usize,
        })
    }

    /// 初始化缓存
    pub async fn init_cache(&self) -> Result<(), AppError> {
        let levels = log_level::Entity::find_enabled()
            .order_by(log_level::Column::LevelValue, Order::Asc)
            .all(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to query log levels for cache: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to query log levels")
            })?;

        let mut cache = self.level_cache.lock().unwrap();
        cache.clear();

        for level in levels {
            cache.insert(level.level_value, level.level_name);
        }

        info!("Initialized log level cache with {} items", cache.len());

        Ok(())
    }

    /// 清空缓存
    pub fn clear_cache(&self) {
        let mut cache = self.level_cache.lock().unwrap();
        cache.clear();
        info!("Cleared log level cache");
    }
}
