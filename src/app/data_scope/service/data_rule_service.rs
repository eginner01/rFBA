/// 数据规则服务实现
/// 提供数据规则管理、查询等功能

use crate::app::data_scope::dto::{
    CreateDataRuleRequest, UpdateDataRuleRequest, DataRuleDetailResponse,
    DataRuleListResponse, DataRuleSimpleResponse, DataRuleModelListResponse,
    DataRuleModelResponse, DataRuleColumnListResponse, DataRuleColumnResponse,
    DataRulePaginationQuery, DataRuleBatchOperationResponse,
};
use crate::app::data_scope::dto::DataRuleQueryParams;
use crate::common::exception::{AppError, ErrorCode};
use crate::database::entity::data_rule;
use sea_orm::{ActiveValue, ActiveModelTrait, DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, PaginatorTrait, QueryOrder};
use tracing::{error, info};

/// 数据规则服务
pub struct DataRuleService {
    db: DatabaseConnection,
}

impl DataRuleService {
    /// 创建新的数据规则服务
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// 创建数据规则
    pub async fn create_data_rule(
        &self,
        request: &CreateDataRuleRequest,
    ) -> Result<DataRuleDetailResponse, AppError> {
        use data_rule::Entity as DataRuleEntity;
        
        // 检查规则名称是否已存在
        let exists = DataRuleEntity::find()
            .filter(data_rule::Column::Name.eq(&request.name))
            .one(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to check data rule name: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, format!("Database error: {}", e))
            })?;

        if exists.is_some() {
            return Err(AppError::with_message(
                ErrorCode::BusinessError,
                "数据规则名称已存在",
            ));
        }

        // 创建数据规则
        let new_rule = data_rule::ActiveModel {
            id: ActiveValue::NotSet,
            name: ActiveValue::Set(request.name.clone()),
            model: ActiveValue::Set(request.model.clone()),
            column: ActiveValue::Set(request.column.clone()),
            operator: ActiveValue::Set(request.operator),
            expression: ActiveValue::Set(request.expression),
            value: ActiveValue::Set(request.value.clone()),
        };

        let result = new_rule.insert(&self.db).await.map_err(|e| {
            error!("Failed to create data rule: {:?}", e);
            AppError::with_message(ErrorCode::DatabaseError, format!("Failed to create data rule: {}", e))
        })?;

        info!("Created data rule: {} (id: {})", result.name, result.id);

        Ok(DataRuleDetailResponse {
            id: result.id,
            name: result.name,
            model: result.model,
            column: result.column,
            operator: result.operator,
            expression: result.expression,
            value: result.value,
        })
    }

    /// 更新数据规则
    pub async fn update_data_rule(
        &self,
        id: i64,
        request: &UpdateDataRuleRequest,
    ) -> Result<DataRuleDetailResponse, AppError> {
        use data_rule::Entity as DataRuleEntity;
        
        // 检查数据规则是否存在
        let existing = DataRuleEntity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to find data rule: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, format!("Database error: {}", e))
            })?
            .ok_or_else(|| AppError::with_message(ErrorCode::NotFound, "数据规则不存在"))?;

        // 检查名称是否冲突（排除自己）
        let name_exists = DataRuleEntity::find()
            .filter(data_rule::Column::Name.eq(&request.name))
            .filter(data_rule::Column::Id.ne(id))
            .one(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to check data rule name: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, format!("Database error: {}", e))
            })?;

        if name_exists.is_some() {
            return Err(AppError::with_message(
                ErrorCode::BusinessError,
                "数据规则名称已存在",
            ));
        }

        // 更新数据规则
        let update_rule = data_rule::ActiveModel {
            id: ActiveValue::Set(id),
            name: ActiveValue::Set(request.name.clone()),
            model: ActiveValue::Set(request.model.clone()),
            column: ActiveValue::Set(request.column.clone()),
            operator: ActiveValue::Set(request.operator),
            expression: ActiveValue::Set(request.expression),
            value: ActiveValue::Set(request.value.clone()),
        };

        let result = update_rule.update(&self.db).await.map_err(|e| {
            error!("Failed to update data rule: {:?}", e);
            AppError::with_message(ErrorCode::DatabaseError, format!("Failed to update data rule: {}", e))
        })?;

        info!("Updated data rule: {} (id: {})", result.name, result.id);

        Ok(DataRuleDetailResponse {
            id: result.id,
            name: result.name,
            model: result.model,
            column: result.column,
            operator: result.operator,
            expression: result.expression,
            value: result.value,
        })
    }

    /// 获取数据规则详情
    pub async fn get_data_rule_detail(
        &self,
        id: i64,
    ) -> Result<DataRuleDetailResponse, AppError> {
        use data_rule::Entity as DataRuleEntity;
        
        let data_rule = DataRuleEntity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to find data rule: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, format!("Database error: {}", e))
            })?
            .ok_or_else(|| AppError::with_message(ErrorCode::NotFound, "数据规则不存在"))?;

        Ok(DataRuleDetailResponse {
            id: data_rule.id,
            name: data_rule.name,
            model: data_rule.model,
            column: data_rule.column,
            operator: data_rule.operator,
            expression: data_rule.expression,
            value: data_rule.value,
        })
    }

    /// 获取数据规则列表（分页）
    pub async fn get_data_rule_list(
        &self,
        query: &DataRulePaginationQuery,
    ) -> Result<DataRuleListResponse, AppError> {
        use data_rule::Entity as DataRuleEntity;
        
        let page = query.page.unwrap_or(1);
        let size = query.size.unwrap_or(20);
        
        let paginator = DataRuleEntity::find()
            .order_by(data_rule::Column::Id, sea_orm::Order::Desc)
            .paginate(&self.db, size);
        
        let total = paginator.num_items().await.map_err(|e| {
            error!("Failed to count data rules: {:?}", e);
            AppError::with_message(ErrorCode::DatabaseError, format!("Database error: {}", e))
        })?;
        
        let data_rules = paginator.fetch_page(page - 1).await.map_err(|e| {
            error!("Failed to fetch data rules: {:?}", e);
            AppError::with_message(ErrorCode::DatabaseError, format!("Database error: {}", e))
        })?;

        let list = data_rules.into_iter().map(|rule| DataRuleDetailResponse {
            id: rule.id,
            name: rule.name,
            model: rule.model,
            column: rule.column,
            operator: rule.operator,
            expression: rule.expression,
            value: rule.value,
        }).collect();

        Ok(DataRuleListResponse {
            list,
            total,
            page,
            size,
        })
    }

    /// 获取所有数据规则
    pub async fn get_all_data_rules(
        &self,
        _params: &DataRuleQueryParams,
    ) -> Result<Vec<DataRuleSimpleResponse>, AppError> {
        use data_rule::Entity as DataRuleEntity;
        
        let data_rules = DataRuleEntity::find()
            .all(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to find all data rules: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, format!("Database error: {}", e))
            })?;

        let list = data_rules.into_iter().map(|rule| DataRuleSimpleResponse {
            id: rule.id,
            name: rule.name,
            model: rule.model,
            column: rule.column,
        }).collect();

        Ok(list)
    }

    /// 获取数据规则模型列表
    pub async fn get_data_rule_models(
        &self,
        _keyword: Option<&str>,
    ) -> Result<DataRuleModelListResponse, AppError> {
        // 这里应该返回系统中可用的模型列表
        // 实际项目中可能需要从数据库或配置中读取
        let models = vec![
            DataRuleModelResponse {
                name: "User".to_string(),
                display_name: "用户".to_string(),
                description: "用户管理模型".to_string(),
                table_name: "sys_user".to_string(),
            },
            DataRuleModelResponse {
                name: "Role".to_string(),
                display_name: "角色".to_string(),
                description: "角色管理模型".to_string(),
                table_name: "sys_role".to_string(),
            },
            DataRuleModelResponse {
                name: "Dept".to_string(),
                display_name: "部门".to_string(),
                description: "部门管理模型".to_string(),
                table_name: "sys_dept".to_string(),
            },
            DataRuleModelResponse {
                name: "Menu".to_string(),
                display_name: "菜单".to_string(),
                description: "菜单管理模型".to_string(),
                table_name: "sys_menu".to_string(),
            },
        ];

        Ok(DataRuleModelListResponse { models })
    }

    /// 获取模型列信息
    pub async fn get_model_columns(
        &self,
        model: &str,
        _include_system: Option<bool>,
    ) -> Result<DataRuleColumnListResponse, AppError> {
        // 这里应该根据模型名称返回对应的列信息
        // 实际项目中可能需要从数据库Schema或配置中读取
        let columns = match model {
            "User" => vec![
                DataRuleColumnResponse {
                    name: "id".to_string(),
                    display_name: "ID".to_string(),
                    type_: "bigint".to_string(),
                    nullable: false,
                    comment: Some("主键ID".to_string()),
                    primary_key: true,
                    unique: true,
                },
                DataRuleColumnResponse {
                    name: "username".to_string(),
                    display_name: "用户名".to_string(),
                    type_: "varchar".to_string(),
                    nullable: false,
                    comment: Some("用户名".to_string()),
                    primary_key: false,
                    unique: true,
                },
                DataRuleColumnResponse {
                    name: "email".to_string(),
                    display_name: "邮箱".to_string(),
                    type_: "varchar".to_string(),
                    nullable: true,
                    comment: Some("邮箱地址".to_string()),
                    primary_key: false,
                    unique: true,
                },
            ],
            "Role" => vec![
                DataRuleColumnResponse {
                    name: "id".to_string(),
                    display_name: "ID".to_string(),
                    type_: "bigint".to_string(),
                    nullable: false,
                    comment: Some("主键ID".to_string()),
                    primary_key: true,
                    unique: true,
                },
                DataRuleColumnResponse {
                    name: "name".to_string(),
                    display_name: "角色名称".to_string(),
                    type_: "varchar".to_string(),
                    nullable: false,
                    comment: Some("角色名称".to_string()),
                    primary_key: false,
                    unique: false,
                },
            ],
            _ => vec![],
        };

        Ok(DataRuleColumnListResponse { columns })
    }

    /// 删除数据规则
    pub async fn delete_data_rule(
        &self,
        id: i64,
    ) -> Result<(), AppError> {
        use data_rule::Entity as DataRuleEntity;
        
        // 检查数据规则是否存在
        let existing = DataRuleEntity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to find data rule: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, format!("Database error: {}", e))
            })?
            .ok_or_else(|| AppError::with_message(ErrorCode::NotFound, "数据规则不存在"))?;

        // 删除数据规则
        DataRuleEntity::delete_by_id(id)
            .exec(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to delete data rule: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, format!("Failed to delete data rule: {}", e))
            })?;
        
        info!("Deleted data rule: id={}", id);

        Ok(())
    }

    /// 批量删除数据规则
    pub async fn batch_delete_data_rules(
        &self,
        ids: &[i64],
    ) -> Result<DataRuleBatchOperationResponse, AppError> {
        use data_rule::Entity as DataRuleEntity;
        
        let mut success_ids = Vec::new();
        let mut failed_ids = Vec::new();

        for &id in ids {
            match DataRuleEntity::delete_by_id(id).exec(&self.db).await {
                Ok(_) => {
                    info!("Deleted data rule: id={}", id);
                    success_ids.push(id);
                }
                Err(e) => {
                    error!("Failed to delete data rule {}: {:?}", id, e);
                    failed_ids.push(id);
                }
            }
        }

        let success_count = success_ids.len();
        let failed_count = failed_ids.len();

        Ok(DataRuleBatchOperationResponse {
            success_ids,
            failed_ids,
            message: format!(
                "批量删除完成：成功 {} 条，失败 {} 条",
                success_count,
                failed_count
            ),
        })
    }
}
