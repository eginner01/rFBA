//! 系统配置服务层

use sea_orm::*;
use redis::AsyncCommands;
use crate::entity::config;
use crate::dto::*;
use crate::error::ConfigError;

const CACHE_PREFIX: &str = "config:";
const CACHE_TTL: u64 = 3600; // 1小时

/// 系统配置服务
pub struct ConfigService;

impl ConfigService {
    /// 获取所有配置（支持按type过滤）
    pub async fn get_all(
        db: &DatabaseConnection,
        type_filter: Option<String>,
    ) -> Result<Vec<ConfigDetail>, ConfigError> {
        let mut query = config::Entity::find();
        
        // 如果提供了type参数，添加过滤条件
        if let Some(config_type) = type_filter {
            query = query.filter(config::Column::ConfigType.eq(config_type));
        }
        
        let configs = query
            .order_by_asc(config::Column::Id)
            .all(db)
            .await
            .map_err(|e| ConfigError::DatabaseError(e.to_string()))?;
        
        Ok(configs.into_iter().map(ConfigDetail::from).collect())
    }
    
    /// 根据ID获取配置
    pub async fn get_by_id(
        db: &DatabaseConnection,
        id: i64,
    ) -> Result<ConfigDetail, ConfigError> {
        let config = config::Entity::find_by_id(id)
            .one(db)
            .await
            .map_err(|e| ConfigError::DatabaseError(e.to_string()))?
            .ok_or(ConfigError::NotFound("配置不存在".to_string()))?;
        
        Ok(ConfigDetail::from(config))
    }
    
    /// 根据key获取配置（带Redis缓存）
    pub async fn get_by_key(
        db: &DatabaseConnection,
        redis: &mut redis::aio::ConnectionManager,
        key: &str,
    ) -> Result<ConfigDetail, ConfigError> {
        // 先从Redis缓存获取
        let cache_key = format!("{}{}", CACHE_PREFIX, key);
        
        if let Ok(cached) = redis.get::<_, String>(&cache_key).await {
            if let Ok(config) = serde_json::from_str::<ConfigDetail>(&cached) {
                tracing::debug!("Config cache hit: {}", key);
                return Ok(config);
            }
        }
        
        // 缓存未命中，从数据库查询
        let config = config::Entity::find_by_key(key)
            .one(db)
            .await
            .map_err(|e| ConfigError::DatabaseError(e.to_string()))?
            .ok_or(ConfigError::NotFound(format!("配置键 {} 不存在", key)))?;
        
        let detail = ConfigDetail::from(config);
        
        // 写入Redis缓存
        if let Ok(json) = serde_json::to_string(&detail) {
            let _: Result<(), _> = redis.set_ex(&cache_key, json, CACHE_TTL).await;
        }
        
        Ok(detail)
    }
    
    /// 分页查询配置
    pub async fn get_list(
        db: &DatabaseConnection,
        query: ConfigQuery,
        pagination: PaginationQuery,
    ) -> Result<PageData<ConfigDetail>, ConfigError> {
        let mut select = config::Entity::find();
        
        // 构建查询条件
        if let Some(name) = &query.name {
            select = select.filter(config::Column::Name.contains(name));
        }
        if let Some(key) = &query.key {
            select = select.filter(config::Column::Key.contains(key));
        }
        if let Some(is_frontend) = query.is_frontend {
            select = select.filter(config::Column::IsFrontend.eq(is_frontend));
        }
        
        // 查询总数
        let total = select
            .clone()
            .count(db)
            .await
            .map_err(|e| ConfigError::DatabaseError(e.to_string()))?;
        
        // 分页查询
        let items = select
            .order_by_asc(config::Column::Id)
            .offset(pagination.offset())
            .limit(pagination.limit())
            .all(db)
            .await
            .map_err(|e| ConfigError::DatabaseError(e.to_string()))?
            .into_iter()
            .map(ConfigDetail::from)
            .collect();
        
        Ok(PageData::new(items, total, pagination.page, pagination.size))
    }
    
    /// 创建配置
    pub async fn create(
        db: &DatabaseConnection,
        param: CreateConfigParam,
    ) -> Result<ConfigDetail, ConfigError> {
        // 检查键是否已存在
        let exists = config::Entity::find()
            .filter(config::Column::Key.eq(&param.key))
            .count(db)
            .await
            .map_err(|e| ConfigError::DatabaseError(e.to_string()))?;
        
        if exists > 0 {
            return Err(ConfigError::AlreadyExists(format!(
                "配置键 {} 已存在",
                param.key
            )));
        }
        
        let now = chrono::Utc::now().naive_utc();
        
        let config = config::ActiveModel {
            name: Set(param.name),
            key: Set(param.key),
            value: Set(param.value),
            config_type: Set(param.config_type),
            is_frontend: Set(param.is_frontend),
            remark: Set(param.remark),
            created_time: Set(now),
            updated_time: Set(None),
            ..Default::default()
        };
        
        let result = config
            .insert(db)
            .await
            .map_err(|e| ConfigError::DatabaseError(e.to_string()))?;
        
        Ok(ConfigDetail::from(result))
    }
    
    /// 更新配置
    pub async fn update(
        db: &DatabaseConnection,
        redis: &mut redis::aio::ConnectionManager,
        id: i64,
        param: UpdateConfigParam,
    ) -> Result<u64, ConfigError> {
        // 检查是否存在
        let config = config::Entity::find_by_id(id)
            .one(db)
            .await
            .map_err(|e| ConfigError::DatabaseError(e.to_string()))?
            .ok_or(ConfigError::NotFound("配置不存在".to_string()))?;
        
        let now = chrono::Utc::now().naive_utc();
        
        let mut config: config::ActiveModel = config.into();
        config.name = Set(param.name);
        config.value = Set(param.value);
        config.config_type = Set(param.config_type);
        config.is_frontend = Set(param.is_frontend);
        config.remark = Set(param.remark);
        config.updated_time = Set(Some(now));
        
        let updated = config
            .update(db)
            .await
            .map_err(|e| ConfigError::DatabaseError(e.to_string()))?;
        
        // 清除Redis缓存
        let cache_key = format!("{}{}", CACHE_PREFIX, updated.key);
        let _: Result<(), _> = redis.del(&cache_key).await;
        
        Ok(1)
    }
    
    /// 批量删除配置
    pub async fn delete_batch(
        db: &DatabaseConnection,
        redis: &mut redis::aio::ConnectionManager,
        ids: Vec<i64>,
    ) -> Result<u64, ConfigError> {
        if ids.is_empty() {
            return Ok(0);
        }
        
        // 获取要删除的配置key，用于清除缓存
        let configs = config::Entity::find()
            .filter(config::Column::Id.is_in(ids.clone()))
            .all(db)
            .await
            .map_err(|e| ConfigError::DatabaseError(e.to_string()))?;
        
        // 删除数据库记录
        let result = config::Entity::delete_many()
            .filter(config::Column::Id.is_in(ids))
            .exec(db)
            .await
            .map_err(|e| ConfigError::DatabaseError(e.to_string()))?;
        
        // 清除Redis缓存
        for cfg in configs {
            let cache_key = format!("{}{}", CACHE_PREFIX, cfg.key);
            let _: Result<(), _> = redis.del(&cache_key).await;
        }
        
        Ok(result.rows_affected)
    }
    
    /// 刷新所有配置缓存
    pub async fn refresh_cache(
        db: &DatabaseConnection,
        redis: &mut redis::aio::ConnectionManager,
    ) -> Result<(), ConfigError> {
        // 获取所有配置
        let configs = config::Entity::find()
            .all(db)
            .await
            .map_err(|e| ConfigError::DatabaseError(e.to_string()))?;
        
        // 重新写入Redis
        for cfg in configs {
            let cache_key = format!("{}{}", CACHE_PREFIX, cfg.key);
            let detail = ConfigDetail::from(cfg);
            
            if let Ok(json) = serde_json::to_string(&detail) {
                let _: Result<(), _> = redis.set_ex(&cache_key, json, CACHE_TTL).await;
            }
        }
        
        Ok(())
    }
}
