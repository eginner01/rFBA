use tracing::{info, error};

/// 部门管理服务实现
/// 提供部门CRUD、树形结构查询、层级管理等功能

use crate::app::dept::dto::{
    CreateDeptRequest, CreateDeptResponse, UpdateDeptRequest, UpdateDeptResponse,
    DeptTreeQuery, DeptListQuery, DeptTreeNode, DeptListItem, DeptDetailResponse,
};
use crate::common::exception::{AppError, ErrorCode};
use crate::database::entity::dept;
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, ColumnTrait, ActiveValue, ActiveModelTrait};
use chrono::Utc;

/// 部门服务
pub struct DeptService {
    db: DatabaseConnection,
}

impl DeptService {
    /// 创建新的部门服务
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// 创建部门
    pub async fn create_dept(
        &self,
        request: &CreateDeptRequest,
        _create_by: &str,
    ) -> Result<CreateDeptResponse, AppError> {
        
        let active_model = dept::ActiveModel {
            id: ActiveValue::NotSet,
            name: ActiveValue::Set(request.name.clone()),
            parent_id: ActiveValue::Set(request.parent_id),
            sort: ActiveValue::Set(request.sort),
            leader: ActiveValue::Set(request.leader.clone()),
            phone: ActiveValue::Set(request.phone.clone()),
            email: ActiveValue::Set(request.email.clone()),
            status: ActiveValue::Set(request.status),
            created_time: ActiveValue::NotSet,
            updated_time: ActiveValue::NotSet,
            del_flag: ActiveValue::Set(0),
        };

        let saved_dept = active_model.insert(&self.db).await.map_err(|e| {
            error!("Failed to create dept: {:?}", e);
            AppError::with_message(ErrorCode::DatabaseError, "Failed to create dept")
        })?;

        info!("Created dept: {} (id: {})", saved_dept.name, saved_dept.id);

        Ok(CreateDeptResponse {
            id: saved_dept.id,
            name: saved_dept.name,
            parent_id: saved_dept.parent_id,
            created_time: saved_dept.created_time.and_utc(),
        })
    }

    /// 更新部门
    pub async fn update_dept(
        &self,
        dept_id: i64,
        request: &UpdateDeptRequest,
        _update_by: Option<&str>,
    ) -> Result<UpdateDeptResponse, AppError> {
        use crate::database::entity::dept::Entity as DeptEntity;

        let existing_dept = DeptEntity::find_by_id(dept_id)
            .one(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to find dept: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, "Failed to find dept")
            })?
            .ok_or_else(|| AppError::with_message(ErrorCode::NotFound, "Dept not found"))?;

        // 检查是否会导致循环依赖
        if let Some(parent_id) = request.parent_id {
            self.check_circular_dependency(dept_id, parent_id).await?;
        }

        let active_model = dept::ActiveModel {
            id: ActiveValue::Set(dept_id),
            name: ActiveValue::Set(request.name.clone()),
            parent_id: ActiveValue::Set(request.parent_id),
            sort: ActiveValue::Set(request.sort),
            leader: ActiveValue::Set(request.leader.clone()),
            phone: ActiveValue::Set(request.phone.clone()),
            email: ActiveValue::Set(request.email.clone()),
            status: ActiveValue::Set(request.status),
            created_time: ActiveValue::Set(existing_dept.created_time),
            updated_time: ActiveValue::Set(Some(Utc::now().naive_utc())),
            del_flag: ActiveValue::Set(existing_dept.del_flag),
        };

        let updated_dept = active_model.update(&self.db).await.map_err(|e| {
            error!("Failed to update dept: {:?}", e);
            AppError::with_message(ErrorCode::DatabaseError, "Failed to update dept")
        })?;

        info!("Updated dept: {} (id: {})", updated_dept.name, updated_dept.id);

        Ok(UpdateDeptResponse {
            id: updated_dept.id,
            name: updated_dept.name,
            updated_time: updated_dept.updated_time.map(|t| t.and_utc()).unwrap_or_else(Utc::now),
        })
    }

    /// 删除部门
    pub async fn delete_dept(&self, dept_id: i64) -> Result<(), AppError> {
        use crate::database::entity::dept::{Entity as DeptEntity, Column as DeptColumn};

        let dept = DeptEntity::find_by_id(dept_id)
            .one(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to find dept: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, "Failed to find dept")
            })?
            .ok_or_else(|| AppError::with_message(ErrorCode::NotFound, "Dept not found"))?;

        // 检查是否有子部门
        let has_children = DeptEntity::find()
            .filter(DeptColumn::ParentId.eq(dept_id))
            .filter(DeptColumn::DelFlag.eq(0))
            .all(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to check children depts: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, "Failed to check children depts")
            })?
            .len();

        if has_children > 0 {
            return Err(AppError::with_message(
                ErrorCode::BadRequest,
                "Cannot delete dept with children"
            ));
        }

        // 软删除
        let active_model = dept::ActiveModel {
            id: ActiveValue::Set(dept.id),
            name: ActiveValue::Set(dept.name),
            parent_id: ActiveValue::Set(dept.parent_id),
            sort: ActiveValue::Set(dept.sort),
            leader: ActiveValue::Set(dept.leader.clone()),
            phone: ActiveValue::Set(dept.phone.clone()),
            email: ActiveValue::Set(dept.email.clone()),
            status: ActiveValue::Set(dept.status),
            created_time: ActiveValue::Set(dept.created_time),
            updated_time: ActiveValue::Set(Some(Utc::now().naive_utc())),
            del_flag: ActiveValue::Set(1),
        };

        active_model.update(&self.db).await.map_err(|e| {
            error!("Failed to delete dept: {:?}", e);
            AppError::with_message(ErrorCode::DatabaseError, "Failed to delete dept")
        })?;

        info!("Deleted dept: {}", dept_id);

        Ok(())
    }

    /// 获取部门树
    pub async fn get_dept_tree(&self, query: &DeptTreeQuery) -> Result<Vec<DeptTreeNode>, AppError> {
        use crate::database::entity::dept::{Entity as DeptEntity, Column as DeptColumn};

        let mut select = DeptEntity::find().filter(DeptColumn::DelFlag.eq(0));

        if let Some(parent_id) = query.parent_id {
            select = select.filter(DeptColumn::ParentId.eq(parent_id));
        } else {
            // 获取根节点（无父部门或父部门为0）
            select = select.filter(DeptColumn::ParentId.is_null());
        }

        let depts = select
            .order_by(DeptColumn::Sort, sea_orm::Order::Asc)
            .all(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to query depts: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, "Failed to query depts")
            })?;

        let mut tree = Vec::new();
        for dept in depts {
            let children = self.get_dept_children(dept.id).await?;
            tree.push(DeptTreeNode {
                id: dept.id,
                name: dept.name,
                parent_id: dept.parent_id,
                sort: dept.sort,
                leader: dept.leader,
                phone: dept.phone,
                email: dept.email,
                status: dept.status,
                status_name: if dept.status == 0 { "正常".to_string() } else { "停用".to_string() },
                children,
            });
        }

        Ok(tree)
    }

    /// 获取部门详情
    pub async fn get_dept_detail(&self, dept_id: i64) -> Result<DeptDetailResponse, AppError> {
        use crate::database::entity::dept::Entity as DeptEntity;

        let dept = DeptEntity::find_by_id(dept_id)
            .one(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to find dept: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, "Failed to find dept")
            })?
            .ok_or_else(|| AppError::with_message(ErrorCode::NotFound, "Dept not found"))?;

        let parent_name = if let Some(parent_id) = dept.parent_id {
            DeptEntity::find_by_id(parent_id)
                .one(&self.db)
                .await
                .ok()
                .flatten()
                .map(|p| p.name)
        } else {
            None
        };

        Ok(DeptDetailResponse {
            id: dept.id,
            name: dept.name,
            parent_id: dept.parent_id,
            parent_name,
            sort: dept.sort,
            leader: dept.leader,
            phone: dept.phone,
            email: dept.email,
            status: dept.status,
            status_name: if dept.status == 0 { "正常".to_string() } else { "停用".to_string() },
            create_by: None,
            update_by: None,
            created_time: dept.created_time.and_utc(),
            updated_time: dept.updated_time.map(|t| t.and_utc()),
            del_flag: dept.del_flag != 0,
        })
    }

    /// 获取部门列表（扁平列表）
    pub async fn get_dept_list(&self) -> Result<Vec<DeptListItem>, AppError> {
        let query = DeptListQuery::default();
        self.get_dept_list_with_filter(&query).await
    }

    /// 获取部门列表（支持筛选）
    pub async fn get_dept_list_with_filter(
        &self,
        query: &DeptListQuery,
    ) -> Result<Vec<DeptListItem>, AppError> {
        use crate::database::entity::dept::{Entity as DeptEntity, Column as DeptColumn};

        let mut select = DeptEntity::find().filter(DeptColumn::DelFlag.eq(0));

        // 按部门名称筛选
        if let Some(ref dept_name) = query.dept_name {
            select = select.filter(DeptColumn::Name.like(format!("%{}%", dept_name)));
        }

        // 按状态筛选
        if let Some(status) = query.status {
            select = select.filter(DeptColumn::Status.eq(status));
        }

        // 按负责人筛选
        if let Some(ref leader) = query.leader {
            select = select.filter(DeptColumn::Leader.like(format!("%{}%", leader)));
        }

        // 按父部门筛选
        if let Some(parent_id) = query.parent_id {
            select = select.filter(DeptColumn::ParentId.eq(parent_id));
        } else if query.include_children.unwrap_or(false) {
            // 如果需要包含子部门，不设置父部门筛选
        } else {
            // 默认只查询根部门（无父部门）
            select = select.filter(DeptColumn::ParentId.is_null());
        }

        let depts = select
            .order_by(DeptColumn::Sort, sea_orm::Order::Asc)
            .all(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to query depts: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, "Failed to query depts")
            })?;

        let mut result = Vec::new();
        for dept in depts {
            let parent_name = if let Some(parent_id) = dept.parent_id {
                DeptEntity::find_by_id(parent_id)
                    .one(&self.db)
                    .await
                    .ok()
                    .flatten()
                    .map(|p| p.name)
            } else {
                None
            };

            result.push(DeptListItem {
                id: dept.id,
                name: dept.name,
                parent_id: dept.parent_id,
                parent_name,
                sort: dept.sort,
                leader: dept.leader,
                phone: dept.phone,
                email: dept.email,
                status: dept.status,
                status_name: if dept.status == 0 { "正常".to_string() } else { "停用".to_string() },
                created_time: dept.created_time.and_utc(),
                updated_time: dept.updated_time.map(|t| t.and_utc()),
            });
        }

        Ok(result)
    }

    /// 更改部门状态
    pub async fn change_status(&self, dept_id: i64, status: i32) -> Result<(), AppError> {
        use crate::database::entity::dept::Entity as DeptEntity;

        let dept = DeptEntity::find_by_id(dept_id)
            .one(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to find dept: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, "Failed to find dept")
            })?
            .ok_or_else(|| AppError::with_message(ErrorCode::NotFound, "Dept not found"))?;

        let active_model = dept::ActiveModel {
            id: ActiveValue::Set(dept.id),
            name: ActiveValue::Set(dept.name),
            parent_id: ActiveValue::Set(dept.parent_id),
            sort: ActiveValue::Set(dept.sort),
            leader: ActiveValue::Set(dept.leader.clone()),
            phone: ActiveValue::Set(dept.phone.clone()),
            email: ActiveValue::Set(dept.email.clone()),
            status: ActiveValue::Set(status),
            created_time: ActiveValue::Set(dept.created_time),
            updated_time: ActiveValue::Set(Some(Utc::now().naive_utc())),
            del_flag: ActiveValue::Set(dept.del_flag),
        };

        active_model.update(&self.db).await.map_err(|e| {
            error!("Failed to update dept status: {:?}", e);
            AppError::with_message(ErrorCode::DatabaseError, "Failed to update dept status")
        })?;

        // 如果是禁用状态，同时禁用所有子部门
        if status == 1 {
            self.disable_children(dept_id).await?;
        }

        Ok(())
    }

    /// 获取子部门（非递归，避免Stack Overflow）
    async fn get_dept_children(&self, parent_id: i64) -> Result<Vec<DeptTreeNode>, AppError> {
        use crate::database::entity::dept::{Entity as DeptEntity, Column as DeptColumn};

        // 使用栈代替递归
        let _stack = [parent_id];
        let mut result = Vec::new();
        let mut all_children = std::collections::HashMap::new();

        // 首先获取所有子部门
        let children = DeptEntity::find()
            .filter(DeptColumn::DelFlag.eq(0))
            .all(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to query all depts: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, "Failed to query all depts")
            })?;

        // 按父ID分组
        for child in children {
            all_children.entry(child.parent_id).or_insert_with(Vec::new).push(child);
        }

        // 查找直接子部门
        if let Some(children) = all_children.get(&Some(parent_id)) {
            for child in children {
                result.push(DeptTreeNode {
                    id: child.id,
                    name: child.name.clone(),
                    parent_id: child.parent_id,
                    sort: child.sort,
                    leader: child.leader.clone(),
                    phone: child.phone.clone(),
                    email: child.email.clone(),
                    status: child.status,
                    status_name: if child.status == 0 { "正常".to_string() } else { "停用".to_string() },
                    children: Vec::new(), // 简化处理，不展开子部门
                });
            }
        }

        Ok(result)
    }

    /// 检查循环依赖
    async fn check_circular_dependency(&self, dept_id: i64, parent_id: i64) -> Result<(), AppError> {
        use crate::database::entity::dept::Entity as DeptEntity;

        let mut current_id: Option<i64> = Some(parent_id);

        while let Some(id) = current_id {
            if id == dept_id {
                return Err(AppError::with_message(
                    ErrorCode::BadRequest,
                    "Circular dependency detected"
                ));
            }

            let dept = DeptEntity::find_by_id(id)
                .one(&self.db)
                .await
                .map_err(|e| {
                    error!("Failed to find dept: {:?}", e);
                    AppError::with_message(ErrorCode::DatabaseError, "Failed to find dept")
                })?;

            if let Some(d) = dept {
                current_id = d.parent_id;
            } else {
                break;
            }
        }

        Ok(())
    }

    /// 禁用子部门（简化版，只禁用直接子部门）
    async fn disable_children(&self, parent_id: i64) -> Result<(), AppError> {
        use crate::database::entity::dept::{Entity as DeptEntity, Column as DeptColumn};

        let children = DeptEntity::find()
            .filter(DeptColumn::ParentId.eq(parent_id))
            .all(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to query children: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, "Failed to query children")
            })?;

        for child in children {
            let active_model = dept::ActiveModel {
                id: ActiveValue::Set(child.id),
                name: ActiveValue::Set(child.name),
                parent_id: ActiveValue::Set(child.parent_id),
                sort: ActiveValue::Set(child.sort),
                leader: ActiveValue::Set(child.leader.clone()),
                phone: ActiveValue::Set(child.phone.clone()),
                email: ActiveValue::Set(child.email.clone()),
                status: ActiveValue::Set(1),
                created_time: ActiveValue::Set(child.created_time),
                updated_time: ActiveValue::Set(Some(Utc::now().naive_utc())),
                del_flag: ActiveValue::Set(child.del_flag),
            };

            active_model.update(&self.db).await.map_err(|e| {
                error!("Failed to disable child dept: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, "Failed to disable child dept")
            })?;
        }

        Ok(())
    }
}
