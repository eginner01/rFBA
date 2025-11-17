
use crate::app::role::dto::{
    CreateRoleRequest, CreateRoleResponse, UpdateRoleRequest,
    RoleDetailResponse, RoleListItem, RolePaginationQuery,
    RolePermissionTree,
};
use crate::common::exception::{AppError, ErrorCode};
use crate::database::entity::role;
use crate::database::role_repo::RoleRepository as RoleRepo;
use sea_orm::{ActiveValue, DatabaseConnection};

pub struct RoleService {
    db: DatabaseConnection,
}

impl RoleService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create_role(
        &self,
        request: &CreateRoleRequest,
    ) -> Result<CreateRoleResponse, AppError> {
        let role_model = role::ActiveModel {
            id: ActiveValue::NotSet,
            name: ActiveValue::Set(request.name.clone()),
            status: ActiveValue::Set(request.status.unwrap_or(1)),
            remark: ActiveValue::Set(request.remark.clone()),
            is_filter_scopes: ActiveValue::Set(request.is_filter_scopes.unwrap_or(false)),
            created_time: ActiveValue::NotSet,
            updated_time: ActiveValue::NotSet,
        };

        // 3. 保存到数据库
        let created_role = RoleRepo::create(role_model, &self.db)
            .await
            .map_err(|e| AppError::with_details(
                ErrorCode::DatabaseError,
                "角色创建失败",
                e.to_string(),
            ))?;

        Ok(CreateRoleResponse {
            id: created_role.id,
            name: created_role.name,
            status: created_role.status,
            created_time: created_role.created_time.and_utc(),
        })
    }

    /// 更新角色
    ///
    /// 返回操作结果
    pub async fn update_role(
        &self,
        role_id: i64,
        request: &UpdateRoleRequest,
    ) -> Result<(), AppError> {
        // 1. 检查角色是否存在
        let existing_role = RoleRepo::find_by_id(role_id, &self.db)
            .await
            .map_err(|_| AppError::new(ErrorCode::RoleNotFound))?;

        // 2. 验证角色编码唯一性（数据库表中没有code字段，跳过此检查）
        // if let Some(code) = &request.code {
        //     if RoleRepo::exists_by_code(code, &self.db).await? {
        //         return Err(AppError::with_details(
        //             ErrorCode::ResourceExists,
        //             "角色更新失败",
        //             "角色编码已存在",
        //         ));
        //     }
        // }

        // 3. 构建更新数据
        let mut update_data = role::ActiveModel::default();
        update_data.id = ActiveValue::Set(role_id);

        if let Some(name) = &request.name {
            update_data.name = ActiveValue::Set(name.clone());
        }

        // 数据库表中没有code和sort字段，跳过这些字段的更新
        // if let Some(code) = &request.code {
        //     update_data.code = ActiveValue::Set(code.clone());
        // }

        if let Some(status) = request.status {
            update_data.status = ActiveValue::Set(status);
        }

        // if let Some(sort) = request.sort {
        //     update_data.sort = ActiveValue::Set(sort);
        // }

        if let Some(is_filter_scopes) = request.is_filter_scopes {
            update_data.is_filter_scopes = ActiveValue::Set(is_filter_scopes);
        }

        // 4. 执行更新
        RoleRepo::update(role_id, update_data, &self.db)
            .await
            .map_err(|e| AppError::with_details(
                ErrorCode::DatabaseError,
                "角色更新失败",
                e.to_string(),
            ))?;

        Ok(())
    }

    /// 删除角色
    ///
    /// 返回操作结果
    pub async fn delete_role(&self, role_id: i64) -> Result<(), AppError> {
        // 检查角色是否存在
        let _ = RoleRepo::find_by_id(role_id, &self.db)
            .await
            .map_err(|_| AppError::new(ErrorCode::RoleNotFound))?;

        // TODO: 检查角色是否有关联用户
        // 如果有关联用户，不允许删除

        // 执行软删除
        RoleRepo::delete(role_id, &self.db)
            .await
            .map_err(|e| AppError::with_details(
                ErrorCode::DatabaseError,
                "角色删除失败",
                e.to_string(),
            ))?;

        Ok(())
    }

    /// 获取角色详情
    ///
    /// 返回角色详情
    pub async fn get_role_detail(
        &self,
        role_id: i64,
    ) -> Result<RoleDetailResponse, AppError> {
        // 1. 获取角色信息
        let role = RoleRepo::find_by_id(role_id, &self.db)
            .await
            .map_err(|_| AppError::new(ErrorCode::RoleNotFound))?;

        // 2. 构建响应
        Ok(RoleDetailResponse {
            id: role.id,
            name: role.name,
            status: role.status,
            is_filter_scopes: role.is_filter_scopes,
            remark: role.remark,
            created_time: role.created_time.and_utc(),
            updated_time: role.updated_time.map(|t| t.and_utc()),
        })
    }

    /// 分页查询角色
    ///
    /// 返回分页查询结果（使用标准 PageData 格式）
    pub async fn get_roles_paginated(
        &self,
        query: &RolePaginationQuery,
    ) -> Result<crate::common::pagination::PageData<RoleListItem>, AppError> {
        // 1. 查询角色列表和总数
        let repo = RoleRepo;  // 创建 RoleRepository 实例
        let (roles, total) = RoleRepo::find_with_pagination(&repo, query, &self.db)
            .await
            .map_err(|e| AppError::with_details(
                ErrorCode::DatabaseError,
                "角色查询失败",
                e.to_string(),
            ))?;

        // 2. 构建分页响应
        let page = query.page.unwrap_or(1) as i64;
        let size = query.size.unwrap_or(20) as i64;

        // 3. 转换角色列表为响应DTO
        let role_list = roles
            .into_iter()
            .map(|role| RoleListItem {
                id: role.id,
                name: role.name,
                status: role.status,
                is_filter_scopes: role.is_filter_scopes,
                remark: role.remark,
                created_time: role.created_time.and_utc(),
            })
            .collect::<Vec<_>>();

        // 4. 使用标准 PageData 返回
        Ok(crate::common::pagination::PageData::new(
            role_list,
            total as i64,
            page,
            size,
        ))
    }

    /// 获取角色权限树
    ///
    /// 返回权限树
    pub async fn get_role_permission_tree(
        &self,
        _role_id: i64,
    ) -> Result<Vec<RolePermissionTree>, AppError> {
        // TODO: 实现获取角色权限树的逻辑
        // 1. 查询所有权限
        // 2. 查询角色已分配的权限
        // 3. 构建树形结构
        Ok(vec![])
    }

    /// 分配角色权限
    ///
    /// 返回操作结果
    pub async fn assign_role_permissions(
        &self,
        _role_id: i64,
        _permission_ids: &[i64],
    ) -> Result<(), AppError> {
        // TODO: 实现分配角色权限的逻辑
        // 1. 检查角色是否存在
        // 2. 检查权限是否存在
        // 3. 删除旧的角色-权限关联
        // 4. 插入新的角色-权限关联
        Ok(())
    }

    /// 获取角色下的用户列表
    ///
    /// 返回用户列表
    pub async fn get_role_users(
        &self,
        _role_id: i64,
    ) -> Result<Vec<i64>, AppError> {
        // TODO: 实现获取角色下用户列表的逻辑
        // 1. 查询用户-角色关联表
        // 2. 返回用户ID列表
        Ok(vec![])
    }

    /// 分配用户到角色
    ///
    /// 返回操作结果
    pub async fn assign_role_users(
        &self,
        _role_id: i64,
        _user_ids: &[i64],
    ) -> Result<(), AppError> {
        // TODO: 实现分配用户到角色的逻辑
        // 1. 检查角色是否存在
        // 2. 检查用户是否存在
        // 3. 删除旧的用户-角色关联
        // 4. 插入新的用户-角色关联
        Ok(())
    }

    /// 检查角色是否可以删除
    ///
    /// 返回是否可以删除
    pub async fn can_delete_role(&self, _role_id: i64) -> Result<bool, AppError> {
        // TODO: 检查角色是否有关联用户
        Ok(true)
    }

    /// 获取所有角色
    /// 返回所有角色列表
    pub async fn get_all_roles(&self) -> Result<Vec<RoleListItem>, AppError> {
        let repo = RoleRepo;  // 创建 RoleRepository 实例
        let roles = RoleRepo::find_all(&repo, &self.db)
            .await
            .map_err(|e| AppError::with_details(
                ErrorCode::DatabaseError,
                "获取所有角色失败",
                e.to_string(),
            ))?;

        let list = roles
            .into_iter()
            .map(|role| RoleListItem {
                id: role.id,
                name: role.name,
                status: role.status,
                is_filter_scopes: role.is_filter_scopes,
                remark: role.remark,
                created_time: role.created_time.and_utc(),
            })
            .collect();

        Ok(list)
    }

    /// 获取角色菜单树
    ///
    /// 返回角色菜单树
    pub async fn get_role_menus(
        &self,
        role_id: i64,
    ) -> Result<Vec<serde_json::Value>, AppError> {
        // 1. 检查角色是否存在
        let _ = RoleRepo::find_by_id(role_id, &self.db)
            .await
            .map_err(|_| AppError::new(ErrorCode::RoleNotFound))?;

        // 2. 获取角色的菜单列表
        // TODO: 从角色菜单关联表查询
        // 暂时返回空列表，后续实现角色-菜单关联逻辑

        Ok(vec![])
    }

    /// 更新角色菜单
    ///
    /// 返回操作结果
    pub async fn update_role_menus(
        &self,
        role_id: i64,
        menu_ids: &[i64],
    ) -> Result<(), AppError> {
        // 1. 检查角色是否存在
        let _ = RoleRepo::find_by_id(role_id, &self.db)
            .await
            .map_err(|_| AppError::new(ErrorCode::RoleNotFound))?;

        // 2. 验证菜单是否存在
        for _menu_id in menu_ids {
            // TODO: 验证菜单存在性
            // let _ = MenuRepo::find_by_id(*menu_id, &self.db)
            //     .await
            //     .map_err(|_| AppError::new(ErrorCode::MenuNotFound))?;
        }

        // 3. TODO: 更新角色-菜单关联表
        // 暂时简化实现

        Ok(())
    }

    /// 获取角色数据权限
    ///
    /// 返回数据权限ID列表
    pub async fn get_role_scopes(
        &self,
        role_id: i64,
    ) -> Result<Vec<i64>, AppError> {
        // 1. 检查角色是否存在
        let _ = RoleRepo::find_by_id(role_id, &self.db)
            .await
            .map_err(|_| AppError::new(ErrorCode::RoleNotFound))?;

        // 2. 获取角色的数据权限
        // TODO: 从角色数据权限关联表查询
        // 暂时返回空列表，后续实现角色-数据权限关联逻辑

        Ok(vec![])
    }

    /// 更新角色数据权限
    ///
    /// 返回操作结果
    pub async fn update_role_scopes(
        &self,
        role_id: i64,
        scope_ids: &[i64],
    ) -> Result<(), AppError> {
        // 1. 检查角色是否存在
        let _ = RoleRepo::find_by_id(role_id, &self.db)
            .await
            .map_err(|_| AppError::new(ErrorCode::RoleNotFound))?;

        // 2. 验证数据权限是否存在
        for _scope_id in scope_ids {
            // TODO: 验证数据权限存在性
        }

        // 3. TODO: 更新角色-数据权限关联表
        // 暂时简化实现

        Ok(())
    }

    /// 批量删除角色
    ///
    /// 返回操作结果
    pub async fn batch_delete_roles(
        &self,
        role_ids: &[i64],
    ) -> Result<(), AppError> {
        // 1. 检查所有角色是否存在
        for role_id in role_ids {
            let _ = RoleRepo::find_by_id(*role_id, &self.db)
                .await
                .map_err(|_| AppError::new(ErrorCode::RoleNotFound))?;

            // TODO: 检查角色是否有关联用户
            // 如果有关联用户，不允许删除
        }

        // 2. 批量软删除
        // TODO: 实现批量删除逻辑
        // 暂时简化实现

        Ok(())
    }
}
