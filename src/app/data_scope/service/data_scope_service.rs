
/// 数据权限服务实现
/// 提供数据权限管理、查询、过滤等功能

use crate::app::data_scope::dto::{
    DataScopeConfigRequest, DataScopeConfigResponse, DataScopeDetailResponse,
    DataScopeListResponse, DataScopeQueryParams, DataScopeCheckResult, DataScopeTreeNode, UserDataScope,
    DataScopeFilter, BatchDataScopeConfigRequest, BatchDataScopeConfigResponse,
    CreateDataScopeRequest, UpdateDataScopeRequest, UpdateDataScopeRuleRequest,
};
use crate::common::exception::{AppError, ErrorCode};
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, EntityTrait, ColumnTrait, QueryFilter};
use crate::database::entity::{data_scope, data_scope_rule};

/// 数据权限服务
pub struct DataScopeService {
    db: DatabaseConnection,
}

impl DataScopeService {
    /// 创建新的数据权限服务
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// 创建数据范围
    pub async fn create_data_scope(
        &self,
        request: &CreateDataScopeRequest,
    ) -> Result<DataScopeDetailResponse, AppError> {
        use tracing::{info, error};
        
        info!("创建数据范围: name={}", request.name);
        
        // 检查名称是否已存在
        let exists = data_scope::Entity::find()
            .filter(data_scope::Column::Name.eq(&request.name))
            .one(&self.db)
            .await
            .map_err(|e| {
                error!("检查数据范围名称失败: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, format!("Database error: {}", e))
            })?;
        
        if exists.is_some() {
            return Err(AppError::with_message(
                ErrorCode::Conflict,
                "数据范围已存在"
            ));
        }
        
        // 创建数据范围
        let new_scope = data_scope::ActiveModel {
            id: ActiveValue::NotSet,
            name: ActiveValue::Set(request.name.clone()),
            status: ActiveValue::Set(request.status),
        };
        
        let result = new_scope.insert(&self.db).await.map_err(|e| {
            error!("创建数据范围失败: {:?}", e);
            AppError::with_message(ErrorCode::DatabaseError, format!("Failed to create data scope: {}", e))
        })?;
        
        info!("成功创建数据范围: id={}", result.id);
        
        Ok(DataScopeDetailResponse {
            id: result.id,
            name: result.name,
            status: result.status,
            created_time: None,
            updated_time: None,
        })
    }
    
    /// 更新数据范围
    pub async fn update_data_scope(
        &self,
        id: i64,
        request: &UpdateDataScopeRequest,
    ) -> Result<DataScopeDetailResponse, AppError> {
        use tracing::{info, error};
        
        info!("更新数据范围: id={}, name={}", id, request.name);
        
        // 检查数据范围是否存在
        let existing = data_scope::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| {
                error!("查询数据范围失败: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, format!("Database error: {}", e))
            })?
            .ok_or_else(|| AppError::with_message(ErrorCode::NotFound, "数据范围不存在"))?;
        
        // 检查名称冲突（排除自己）
        if existing.name != request.name {
            let name_exists = data_scope::Entity::find()
                .filter(data_scope::Column::Name.eq(&request.name))
                .filter(data_scope::Column::Id.ne(id))
                .one(&self.db)
                .await
                .map_err(|e| {
                    error!("检查数据范围名称失败: {:?}", e);
                    AppError::with_message(ErrorCode::DatabaseError, format!("Database error: {}", e))
                })?;
            
            if name_exists.is_some() {
                return Err(AppError::with_message(
                    ErrorCode::Conflict,
                    "数据范围已存在"
                ));
            }
        }
        
        // 更新数据范围
        let update_scope = data_scope::ActiveModel {
            id: ActiveValue::Set(id),
            name: ActiveValue::Set(request.name.clone()),
            status: ActiveValue::Set(request.status),
        };
        
        let result = update_scope.update(&self.db).await.map_err(|e| {
            error!("更新数据范围失败: {:?}", e);
            AppError::with_message(ErrorCode::DatabaseError, format!("Failed to update data scope: {}", e))
        })?;
        
        info!("成功更新数据范围: id={}", result.id);
        
        Ok(DataScopeDetailResponse {
            id: result.id,
            name: result.name,
            status: result.status,
            created_time: None,
            updated_time: None,
        })
    }
    
    /// 更新数据范围规则
    pub async fn update_data_scope_rules(
        &self,
        data_scope_id: i64,
        rule_ids: &UpdateDataScopeRuleRequest,
    ) -> Result<usize, AppError> {
        use tracing::{info, error};
        
        info!("更新数据范围规则: data_scope_id={}, rules={:?}", data_scope_id, rule_ids.rules);
        
        // 检查数据范围是否存在
        let _ = data_scope::Entity::find_by_id(data_scope_id)
            .one(&self.db)
            .await
            .map_err(|e| {
                error!("查询数据范围失败: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, format!("Database error: {}", e))
            })?
            .ok_or_else(|| AppError::with_message(ErrorCode::NotFound, "数据范围不存在"))?;
        
        // 删除旧的关联
        data_scope_rule::Entity::delete_many()
            .filter(data_scope_rule::Column::DataScopeId.eq(data_scope_id))
            .exec(&self.db)
            .await
            .map_err(|e| {
                error!("删除旧的数据范围规则关联失败: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, format!("Database error: {}", e))
            })?;
        
        // 插入新的关联
        let mut inserted_count = 0;
        for &rule_id in &rule_ids.rules {
            let new_relation = data_scope_rule::ActiveModel {
                id: ActiveValue::NotSet,
                data_scope_id: ActiveValue::Set(data_scope_id),
                data_rule_id: ActiveValue::Set(rule_id),
            };
            
            new_relation.insert(&self.db).await.map_err(|e| {
                error!("插入数据范围规则关联失败: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, format!("Failed to create relation: {}", e))
            })?;
            
            inserted_count += 1;
        }
        
        info!("成功更新数据范围规则: data_scope_id={}, count={}", data_scope_id, inserted_count);
        
        Ok(inserted_count)
    }
    
    /// 配置角色数据权限（暂未实现 - 需要使用新的关联表）
    pub async fn configure_data_scope(
        &self,
        _request: &DataScopeConfigRequest,
    ) -> Result<DataScopeConfigResponse, AppError> {
        // TODO: 需要使用 sys_role_data_scope 关联表来实现
        Err(AppError::with_message(
            ErrorCode::OperationFailed,
            "功能正在重构中，请稍后再试。需要使用新的关联表结构。"
        ))
    }

    /// 批量配置角色数据权限（暂未实现）
    pub async fn batch_configure_data_scope(
        &self,
        _request: &BatchDataScopeConfigRequest,
    ) -> Result<BatchDataScopeConfigResponse, AppError> {
        // TODO: 需要使用 sys_role_data_scope 关联表来实现
        Err(crate::common::exception::AppError::with_message(
            crate::common::exception::ErrorCode::OperationFailed,
            "功能正在重构中"
        ))
    }

    /// 获取数据权限详情（现在参数应该是 data_scope_id，而不是 role_id）
    pub async fn get_data_scope_detail(
        &self,
        data_scope_id: i64,
    ) -> Result<DataScopeDetailResponse, AppError> {
        use tracing::{info, error};
        use crate::database::entity::data_scope;
        use sea_orm::EntityTrait;
        
        info!("查询数据权限详情: id={}", data_scope_id);
        
        // 查找数据权限
        let data_scope_model = data_scope::Entity::find_by_id(data_scope_id)
            .one(&self.db)
            .await
            .map_err(|e| {
                error!("查询数据权限失败: {:?}", e);
                crate::common::exception::AppError::with_message(
                    crate::common::exception::ErrorCode::DatabaseError,
                    format!("Failed to find data scope: {}", e)
                )
            })?
            .ok_or_else(|| {
                error!("数据权限不存在: id={}", data_scope_id);
                crate::common::exception::AppError::with_message(
                    crate::common::exception::ErrorCode::NotFound,
                    "Data scope not found"
                )
            })?;

        Ok(DataScopeDetailResponse {
            id: data_scope_model.id,
            name: data_scope_model.name,
            status: data_scope_model.status,
            created_time: None,
            updated_time: None,
        })
    }

    /// 获取数据权限列表
    pub async fn get_data_scope_list(
        &self,
        params: &DataScopeQueryParams,
    ) -> Result<DataScopeListResponse, AppError> {
        use tracing::{info, error};
        use crate::database::entity::data_scope;
        use sea_orm::{EntityTrait, QueryOrder, PaginatorTrait};
        
        let page = params.page.unwrap_or(1);
        let size = params.size.unwrap_or(20);
        
        info!("开始查询数据权限列表: page={}, size={}", page, size);
        
        // 使用分页查询
        let paginator = data_scope::Entity::find()
            .order_by(data_scope::Column::Id, sea_orm::Order::Desc)
            .paginate(&self.db, size);
        
        // 获取总数
        let total = paginator.num_items().await.map_err(|e| {
            error!("查询数据权限总数失败: {:?}", e);
            AppError::with_message(ErrorCode::DatabaseError, format!("Failed to count data scopes: {}", e))
        })?;
        
        // 获取当前页数据
        let data_scopes = paginator.fetch_page(page - 1).await.map_err(|e| {
            error!("查询数据权限列表失败: {:?}", e);
            AppError::with_message(ErrorCode::DatabaseError, format!("Failed to find data scopes: {}", e))
        })?;

        info!("查询到 {} 条数据权限记录，总共 {} 条", data_scopes.len(), total);

        // 构建响应
        let items: Vec<DataScopeDetailResponse> = data_scopes.into_iter().map(|scope| DataScopeDetailResponse {
            id: scope.id,
            name: scope.name,
            status: scope.status,
            created_time: None,  // 表中没有这些字段
            updated_time: None,
        }).collect();

        // 计算总页数
        let total_pages = if total > 0 {
            ((total as f64) / (size as f64)).ceil() as usize
        } else {
            1
        };
        
        info!("成功构建响应，共 {} 条记录，{} 页", total, total_pages);
        
        Ok(DataScopeListResponse {
            items,
            total: total as usize,
            page: page as usize,
            size: size as usize,
            total_pages,
        })
    }

    /// 删除数据范围
    pub async fn delete_data_scope(
        &self,
        id: i64,
    ) -> Result<(), AppError> {
        use tracing::{info, error};
        
        info!("删除数据范围: id={}", id);
        
        // 检查数据范围是否存在
        let _ = data_scope::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| {
                error!("查询数据范围失败: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, format!("Database error: {}", e))
            })?
            .ok_or_else(|| AppError::with_message(ErrorCode::NotFound, "数据范围不存在"))?;
        
        // 删除数据范围（级联删除关联表）
        data_scope::Entity::delete_by_id(id)
            .exec(&self.db)
            .await
            .map_err(|e| {
                error!("删除数据范围失败: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, format!("Failed to delete data scope: {}", e))
            })?;
        
        info!("成功删除数据范围: id={}", id);
        
        Ok(())
    }
    
    /// 批量删除数据范围
    pub async fn batch_delete_data_scopes(
        &self,
        pks: &[i64],
    ) -> Result<usize, AppError> {
        use tracing::{info, error};
        
        info!("批量删除数据范围: pks={:?}", pks);
        
        let mut deleted_count = 0;
        
        for &id in pks {
            match self.delete_data_scope(id).await {
                Ok(_) => deleted_count += 1,
                Err(e) => {
                    error!("删除数据范围 {} 失败: {:?}", id, e);
                    // 继续删除其他的
                }
            }
        }
        
        info!("批量删除数据范围完成: 成功 {}/{}", deleted_count, pks.len());
        
        Ok(deleted_count)
    }

    /// 获取用户数据权限（暂未实现）
    pub async fn get_user_data_scope(
        &self,
        _user_id: i64,
    ) -> Result<UserDataScope, AppError> {
        // TODO: 需要重新实现
        Err(crate::common::exception::AppError::with_message(
            crate::common::exception::ErrorCode::OperationFailed,
            "功能正在重构中"
        ))
    }

    /// 检查用户是否有数据权限（暂未实现）
    pub async fn check_data_scope(
        &self,
        _user_id: i64,
        _target_user_id: Option<i64>,
        _target_dept_id: Option<i64>,
    ) -> Result<DataScopeCheckResult, AppError> {
        // TODO: 需要重新实现
        Err(crate::common::exception::AppError::with_message(
            crate::common::exception::ErrorCode::OperationFailed,
            "功能正在重构中"
        ))
    }

    /// 获取数据范围过滤条件（暂未实现）
    pub async fn get_data_scope_filter(
        &self,
        _user_id: i64,
    ) -> Result<DataScopeFilter, AppError> {
        // TODO: 需要重新实现
        Err(crate::common::exception::AppError::with_message(
            crate::common::exception::ErrorCode::OperationFailed,
            "功能正在重构中"
        ))
    }

    /// 获取数据权限树（暂未实现）
    pub async fn get_data_scope_tree(
        &self,
        _role_id: i64,
    ) -> Result<Vec<DataScopeTreeNode>, AppError> {
        // TODO: 需要重新实现
        Err(crate::common::exception::AppError::with_message(
            crate::common::exception::ErrorCode::OperationFailed,
            "功能正在重构中"
        ))
    }
}
