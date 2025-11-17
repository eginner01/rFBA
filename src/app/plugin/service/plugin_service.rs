use tracing::{info, warn, error, debug};

/// 插件服务实现
/// 提供插件管理、安装、卸载、配置等功能

use crate::app::plugin::dto::{
    PluginPaginationQuery, PluginPaginationResponse,
    PluginDetailResponse, PluginListItem,
    InstallPluginRequest, InstallPluginResponse,
    UninstallPluginRequest, UninstallPluginResponse,
    EnablePluginRequest, EnablePluginResponse,
    DisablePluginRequest, DisablePluginResponse,
    UpdatePluginRequest, UpdatePluginResponse,
    PluginStatistics, PluginTypeStat,
    PluginConfigPaginationQuery, PluginConfigPaginationResponse,
    PluginConfigItem, UpdatePluginConfigRequest, UpdatePluginConfigResponse,
};
use crate::common::exception::{AppError, ErrorCode};
use crate::database::DatabaseManager;
use crate::database::entity::plugin;
use crate::database::entity::plugin_config;
use sea_orm::{DatabaseConnection, DbErr, EntityTrait, QueryFilter, ColumnTrait, Select};
use std::collections::HashMap;

/// 插件服务
pub struct PluginService {
    db: DatabaseConnection,
}

impl PluginService {
    /// 创建新的插件服务
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// 分页查询插件
    pub async fn get_plugins_paginated(
        &self,
        query: &PluginPaginationQuery,
    ) -> Result<PluginPaginationResponse, AppError> {
        let page = query.page.unwrap_or(1);
        let size = query.size.unwrap_or(10);
        let offset = (page - 1) * size;

        let mut select = plugin::Entity::find();

        if let Some(ref keyword) = query.keyword {
            select = select.filter(
                sea_orm::Condition::any()
                    .add(plugin::Column::Name.like(format!("%{}%", keyword)))
                    .add(plugin::Column::Code.like(format!("%{}%", keyword)))
                    .add(plugin::Column::Description.like(format!("%{}%", keyword))),
            );
        }

        if let Some(ref name) = query.name {
            select = select.filter(plugin::Column::Name.like(format!("%{}%", name)));
        }

        if let Some(ref code) = query.code {
            select = select.filter(plugin::Column::Code.like(format!("%{}%", code)));
        }

        if let Some(plugin_type) = query.plugin_type {
            select = select.filter(plugin::Column::PluginType.eq(plugin_type));
        }

        if let Some(status) = query.status {
            select = select.filter(plugin::Column::Status.eq(status));
        }

        if let Some(is_system) = query.is_system {
            select = select.filter(plugin::Column::IsSystem.eq(is_system));
        }

        if let Some(ref author) = query.author {
            select = select.filter(plugin::Column::Author.like(format!("%{}%", author)));
        }

        if let Some(start_time) = query.start_time {
            select = select.filter(plugin::Column::CreatedTime.gte(start_time));
        }

        if let Some(end_time) = query.end_time {
            select = select.filter(plugin::Column::CreatedTime.lte(end_time));
        }

        let sort_field = query.sort_by.as_ref().unwrap_or(&PluginSortField::SortOrder);
        let sort_order = query.sort_order.as_ref().unwrap_or(&SortOrder::Asc);

        let order_by = match sort_order {
            SortOrder::Asc => sea_orm::Order::Asc,
            SortOrder::Desc => sea_orm::Order::Desc,
        };

        select = select.order_by(
            match sort_field {
                PluginSortField::Id => plugin::Column::Id,
                PluginSortField::Name => plugin::Column::Name,
                PluginSortField::Code => plugin::Column::Code,
                PluginSortField::PluginType => plugin::Column::PluginType,
                PluginSortField::Status => plugin::Column::Status,
                PluginSortField::SortOrder => plugin::Column::SortOrder,
                PluginSortField::CreatedTime => plugin::Column::CreatedTime,
                PluginSortField::UpdatedTime => plugin::Column::UpdatedTime,
            },
            order_by,
        );

        let total = select.clone().count(&self.db).await.map_err(|e| {
            error!("Failed to count plugins: {:?}", e);
            AppError::new(ErrorCode::DatabaseError, "Failed to count plugins")
        })?;

        let plugins = select
            .offset(offset as u64)
            .limit(size as u64)
            .all(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to query plugins: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to query plugins")
            })?;

        let list = plugins
            .into_iter()
            .map(|p| {
                let plugin_type_name =
                    crate::database::entity::plugin::PluginType::from_i32(p.plugin_type)
                        .map(|pt| pt.get_name().to_string())
                        .unwrap_or_else(|| "未知".to_string());

                let status_name =
                    crate::database::entity::plugin::PluginStatus::from_i32(p.status)
                        .map(|ps| ps.get_name().to_string())
                        .unwrap_or_else(|| "未知".to_string());

                PluginListItem {
                    id: p.id,
                    name: p.name,
                    code: p.code,
                    version: p.version,
                    plugin_type: p.plugin_type,
                    plugin_type_name,
                    description: p.description,
                    author: p.author,
                    status: p.status,
                    status_name,
                    sort_order: p.sort_order,
                    is_system: p.is_system,
                    install_time: p.install_time,
                    created_time: p.created_time,
                    updated_time: p.updated_time,
                }
            })
            .collect();

        let pages = (total + size - 1) / size;

        Ok(PluginPaginationResponse {
            list,
            total,
            page,
            size,
            pages,
        })
    }

    /// 获取插件详情
    pub async fn get_plugin_detail(
        &self,
        plugin_id: i64,
    ) -> Result<PluginDetailResponse, AppError> {
        let p = plugin::Entity::find_by_id(plugin_id)
            .one(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to find plugin: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to find plugin")
            })?
            .ok_or_else(|| {
                AppError::new(ErrorCode::NotFound, "Plugin not found")
            })?;

        let plugin_type_name =
            crate::database::entity::plugin::PluginType::from_i32(p.plugin_type)
                .map(|pt| pt.get_name().to_string())
                .unwrap_or_else(|| "未知".to_string());

        let status_name =
            crate::database::entity::plugin::PluginStatus::from_i32(p.status)
                .map(|ps| ps.get_name().to_string())
                .unwrap_or_else(|| "未知".to_string());

        Ok(PluginDetailResponse {
            id: p.id,
            name: p.name,
            code: p.code,
            version: p.version,
            plugin_type: p.plugin_type,
            plugin_type_name,
            description: p.description,
            author: p.author,
            homepage: p.homepage,
            file_path: p.file_path,
            class_name: p.class_name,
            config: p.config,
            status: p.status,
            status_name,
            sort_order: p.sort_order,
            is_system: p.is_system,
            dependencies: p.dependencies,
            install_time: p.install_time,
            uninstall_time: p.uninstall_time,
            created_time: p.created_time,
            updated_time: p.updated_time,
        })
    }

    /// 安装插件
    pub async fn install_plugin(
        &self,
        request: &InstallPluginRequest,
    ) -> Result<InstallPluginResponse, AppError> {
        // TODO: 实现插件文件解析
        // 目前只是一个示例实现

        let plugin_info = self.parse_plugin_file(&request.file_path).await?;

        let active_model = plugin::ActiveModel {
            id: Default::default(),
            name: sea_orm::Set(plugin_info.name),
            code: sea_orm::Set(plugin_info.code),
            version: sea_orm::Set(plugin_info.version),
            plugin_type: sea_orm::Set(plugin_info.plugin_type),
            description: sea_orm::Set(plugin_info.description),
            author: sea_orm::Set(plugin_info.author),
            homepage: sea_orm::Set(plugin_info.homepage),
            file_path: sea_orm::Set(request.file_path.clone()),
            class_name: sea_orm::Set(plugin_info.class_name),
            config: sea_orm::Set(plugin_info.config),
            status: sea_orm::Set(1), // 默认启用
            sort_order: sea_orm::Set(0),
            is_system: sea_orm::Set(0),
            dependencies: sea_orm::Set(plugin_info.dependencies),
            install_time: sea_orm::Set(Some(chrono::Utc::now().naive_utc())),
            uninstall_time: sea_orm::Set(None),
            created_time: sea_orm::Set(chrono::Utc::now().naive_utc()),
            updated_time: sea_orm::Set(chrono::Utc::now().naive_utc()),
        };

        let saved_plugin = active_model.insert(&self.db).await.map_err(|e| {
            error!("Failed to install plugin: {:?}", e);
            AppError::new(ErrorCode::DatabaseError, "Failed to install plugin")
        })?;

        Ok(InstallPluginResponse {
            id: saved_plugin.id,
            name: saved_plugin.name,
            code: saved_plugin.code,
            status: 0,
            message: "插件安装成功".to_string(),
            install_time: saved_plugin.install_time.unwrap(),
        })
    }

    /// 卸载插件
    pub async fn uninstall_plugin(
        &self,
        request: &UninstallPluginRequest,
    ) -> Result<UninstallPluginResponse, AppError> {
        let p = plugin::Entity::find_by_id(request.id)
            .one(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to find plugin: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to find plugin")
            })?
            .ok_or_else(|| {
                AppError::new(ErrorCode::NotFound, "Plugin not found")
            })?;

        if p.is_system == 1 {
            return Err(AppError::new(
                ErrorCode::Forbidden,
                "Cannot uninstall system plugin",
            ));
        }

        let mut active_model = p.into_active_model();
        active_model.status = sea_orm::Set(2); // 已卸载
        active_model.uninstall_time = sea_orm::Set(Some(chrono::Utc::now().naive_utc()));
        active_model.updated_time = sea_orm::Set(chrono::Utc::now().naive_utc());

        active_model.update(&self.db).await.map_err(|e| {
            error!("Failed to uninstall plugin: {:?}", e);
            AppError::new(ErrorCode::DatabaseError, "Failed to uninstall plugin")
        })?;

        Ok(UninstallPluginResponse {
            id: request.id,
            name: "插件名称".to_string(), // TODO: 从数据库获取
            status: 0,
            message: "插件卸载成功".to_string(),
            uninstall_time: chrono::Utc::now(),
        })
    }

    /// 启用插件
    pub async fn enable_plugin(
        &self,
        request: &EnablePluginRequest,
    ) -> Result<EnablePluginResponse, AppError> {
        self.update_plugin_status(request.id, 1).await?;

        Ok(EnablePluginResponse {
            id: request.id,
            name: "插件名称".to_string(), // TODO: 从数据库获取
            status: 1,
            message: "插件启用成功".to_string(),
            time: chrono::Utc::now(),
        })
    }

    /// 禁用插件
    pub async fn disable_plugin(
        &self,
        request: &DisablePluginRequest,
    ) -> Result<DisablePluginResponse, AppError> {
        self.update_plugin_status(request.id, 0).await?;

        Ok(DisablePluginResponse {
            id: request.id,
            name: "插件名称".to_string(), // TODO: 从数据库获取
            status: 0,
            message: "插件禁用成功".to_string(),
            time: chrono::Utc::now(),
        })
    }

    /// 更新插件
    pub async fn update_plugin(
        &self,
        request: &UpdatePluginRequest,
    ) -> Result<UpdatePluginResponse, AppError> {
        let existing_plugin = plugin::Entity::find_by_id(request.id)
            .one(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to find plugin: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to find plugin")
            })?
            .ok_or_else(|| {
                AppError::new(ErrorCode::NotFound, "Plugin not found")
            })?;

        let mut active_model = existing_plugin.into_active_model();

        if let Some(ref name) = request.name {
            active_model.name = sea_orm::Set(name.clone());
        }

        if let Some(ref description) = request.description {
            active_model.description = sea_orm::Set(Some(description.clone()));
        }

        if let Some(sort_order) = request.sort_order {
            active_model.sort_order = sea_orm::Set(sort_order);
        }

        active_model.updated_time = sea_orm::Set(chrono::Utc::now().naive_utc());

        let updated_plugin = active_model.update(&self.db).await.map_err(|e| {
            error!("Failed to update plugin: {:?}", e);
            AppError::new(ErrorCode::DatabaseError, "Failed to update plugin")
        })?;

        Ok(UpdatePluginResponse {
            id: updated_plugin.id,
            name: updated_plugin.name,
            updated_time: updated_plugin.updated_time,
        })
    }

    /// 获取插件统计
    pub async fn get_plugin_statistics(&self) -> Result<PluginStatistics, AppError> {
        let total_count = plugin::Entity::find()
            .filter(plugin::Column::Status.ne(2))
            .count(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to count total plugins: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to count total plugins")
            })?;

        let enabled_count = plugin::Entity::find()
            .filter(plugin::Column::Status.eq(1))
            .count(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to count enabled plugins: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to count enabled plugins")
            })?;

        let disabled_count = plugin::Entity::find()
            .filter(plugin::Column::Status.eq(0))
            .count(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to count disabled plugins: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to count disabled plugins")
            })?;

        let uninstalled_count = plugin::Entity::find()
            .filter(plugin::Column::Status.eq(2))
            .count(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to count uninstalled plugins: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to count uninstalled plugins")
            })?;

        let system_count = plugin::Entity::find()
            .filter(
                sea_orm::Condition::all()
                    .add(plugin::Column::IsSystem.eq(1))
                    .add(plugin::Column::Status.ne(2))
            )
            .count(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to count system plugins: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to count system plugins")
            })?;

        let custom_count = total_count - system_count;

        // 获取所有插件进行类型统计
        let all_plugins = plugin::Entity::find()
            .filter(plugin::Column::Status.ne(2))
            .all(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to query all plugins: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to query all plugins")
            })?;

        let mut type_map: HashMap<i32, usize> = HashMap::new();
        for p in all_plugins {
            *type_map.entry(p.plugin_type).or_insert(0) += 1;
        }

        let type_stats: Vec<PluginTypeStat> = type_map
            .into_iter()
            .map(|(plugin_type, count)| {
                let plugin_type_name =
                    crate::database::entity::plugin::PluginType::from_i32(plugin_type)
                        .map(|pt| pt.get_name().to_string())
                        .unwrap_or_else(|| "未知".to_string());

                PluginTypeStat {
                    plugin_type,
                    plugin_type_name,
                    count,
                }
            })
            .collect();

        Ok(PluginStatistics {
            total_count,
            enabled_count,
            disabled_count,
            uninstalled_count,
            system_count,
            custom_count,
            type_stats,
        })
    }

    /// 分页查询插件配置
    pub async fn get_plugin_configs_paginated(
        &self,
        query: &PluginConfigPaginationQuery,
    ) -> Result<PluginConfigPaginationResponse, AppError> {
        let page = query.page.unwrap_or(1);
        let size = query.size.unwrap_or(10);
        let offset = (page - 1) * size;

        let mut select = plugin_config::Entity::find();

        if let Some(plugin_id) = query.plugin_id {
            select = select.filter(plugin_config::Column::PluginId.eq(plugin_id));
        }

        select = select.order_by(plugin_config::Column::ConfigKey, sea_orm::Order::Asc);

        let total = select.clone().count(&self.db).await.map_err(|e| {
            error!("Failed to count plugin configs: {:?}", e);
            AppError::new(ErrorCode::DatabaseError, "Failed to count plugin configs")
        })?;

        let configs = select
            .offset(offset as u64)
            .limit(size as u64)
            .all(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to query plugin configs: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to query plugin configs")
            })?;

        let list = configs
            .into_iter()
            .map(|c| {
                let value_type_name =
                    crate::database::entity::plugin_config::ConfigValueType::from_i32(c.value_type)
                        .map(|vt| vt.get_name().to_string())
                        .unwrap_or_else(|| "未知".to_string());

                PluginConfigItem {
                    id: c.id,
                    plugin_id: c.plugin_id,
                    config_key: c.config_key,
                    config_value: c.config_value,
                    description: c.description,
                    value_type: c.value_type,
                    value_type_name,
                    is_required: c.is_required,
                    default_value: c.default_value,
                    created_time: c.created_time,
                    updated_time: c.updated_time,
                }
            })
            .collect();

        let pages = (total + size - 1) / size;

        Ok(PluginConfigPaginationResponse {
            list,
            total,
            page,
            size,
            pages,
        })
    }

    /// 更新插件配置
    pub async fn update_plugin_config(
        &self,
        request: &UpdatePluginConfigRequest,
    ) -> Result<UpdatePluginConfigResponse, AppError> {
        let existing_config = plugin_config::Entity::find_by_id(request.id)
            .one(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to find plugin config: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to find plugin config")
            })?
            .ok_or_else(|| {
                AppError::new(ErrorCode::NotFound, "Plugin config not found")
            })?;

        let mut active_model = existing_config.into_active_model();
        active_model.config_value = sea_orm::Set(request.config_value.clone());
        active_model.updated_time = sea_orm::Set(chrono::Utc::now().naive_utc());

        let updated_config = active_model.update(&self.db).await.map_err(|e| {
            error!("Failed to update plugin config: {:?}", e);
            AppError::new(ErrorCode::DatabaseError, "Failed to update plugin config")
        })?;

        Ok(UpdatePluginConfigResponse {
            id: updated_config.id,
            config_key: updated_config.config_key,
            updated_time: updated_config.updated_time,
        })
    }

    /// 更新插件状态
    async fn update_plugin_status(&self, plugin_id: i64, status: i32) -> Result<(), AppError> {
        let existing_plugin = plugin::Entity::find_by_id(plugin_id)
            .one(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to find plugin: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to find plugin")
            })?
            .ok_or_else(|| {
                AppError::new(ErrorCode::NotFound, "Plugin not found")
            })?;

        let mut active_model = existing_plugin.into_active_model();
        active_model.status = sea_orm::Set(status);
        active_model.updated_time = sea_orm::Set(chrono::Utc::now().naive_utc());

        active_model.update(&self.db).await.map_err(|e| {
            error!("Failed to update plugin status: {:?}", e);
            AppError::new(ErrorCode::DatabaseError, "Failed to update plugin status")
        })?;

        Ok(())
    }

    /// 解析插件文件
    async fn parse_plugin_file(
        &self,
        _file_path: &str,
    ) -> Result<PluginInfo, AppError> {
        // TODO: 实现插件文件解析
        // 这里应该解析插件的元数据信息
        // 目前只是一个示例实现

        Ok(PluginInfo {
            name: "示例插件".to_string(),
            code: "example-plugin".to_string(),
            version: "1.0.0".to_string(),
            plugin_type: 0,
            description: Some("这是一个示例插件".to_string()),
            author: Some("系统".to_string()),
            homepage: None,
            class_name: "ExamplePlugin".to_string(),
            config: None,
            dependencies: None,
        })
    }
}

/// 插件信息
struct PluginInfo {
    name: String,
    code: String,
    version: String,
    plugin_type: i32,
    description: Option<String>,
    author: Option<String>,
    homepage: Option<String>,
    class_name: String,
    config: Option<String>,
    dependencies: Option<String>,
}
