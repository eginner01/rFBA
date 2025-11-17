use tracing::{info, error};

use crate::app::user::dto::{
    CreateUserRequest, CreateUserResponse, UpdateUserRequest,
    UserDetailResponse, UserListItem, ChangePasswordRequest, ResetPasswordRequest,
    UserPaginationQuery,
    ImportUsersRequest, ImportUsersResponse, ImportError, ImportResult,
    ExportUsersRequest, ExportUsersResponse, DownloadTemplateRequest, DownloadTemplateResponse,
    UserImportTemplateItem, UserExportItem, BatchImportUsersRequest, BatchImportUsersResponse,
};
use crate::common::exception::{AppError, ErrorCode};
use crate::database::entity::user;
use crate::database::user_repo::UserRepository as UserRepo;
use crate::utils::encrypt::CryptoUtils;
use sea_orm::{ActiveValue, DatabaseConnection, EntityTrait};
use base64::{Engine as _, engine::general_purpose};
use csv::{ReaderBuilder, WriterBuilder};
use std::io::Cursor;
use chrono::Utc;

pub struct UserService {
    db: DatabaseConnection,
}

impl UserService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create_user(
        &self,
        request: &CreateUserRequest,
    ) -> Result<CreateUserResponse, AppError> {
        if UserRepo::exists_by_username(&request.username, &self.db).await? {
            return Err(AppError::with_details(
                ErrorCode::ResourceExists,
                "用户创建失败",
                "用户名已存在",
            ));
        }

        if let Some(email) = &request.email {
            if UserRepo::exists_by_email(email, &self.db).await? {
                return Err(AppError::with_details(
                    ErrorCode::ResourceExists,
                    "用户创建失败",
                    "邮箱已存在",
                ));
            }
        }

        let hashed_password = CryptoUtils::hash_password(&request.password, None)
            .await
            .map_err(|e| AppError::with_details(
                ErrorCode::BusinessError,
                "密码加密失败",
                e.to_string(),
            ))?;

        let user_model = user::ActiveModel {
            id: ActiveValue::NotSet,
            uuid: ActiveValue::Set(uuid::Uuid::new_v4().to_string()),
            username: ActiveValue::Set(request.username.clone()),
            nickname: ActiveValue::Set(request.nickname.clone()),
            password: ActiveValue::Set(Some(hashed_password)),
            salt: ActiveValue::Set(None),
            email: ActiveValue::Set(request.email.clone()),
            phone: ActiveValue::Set(request.phone.clone()),
            avatar: ActiveValue::Set(request.avatar.clone()),
            status: ActiveValue::Set(request.status.unwrap_or(1)),
            is_superuser: ActiveValue::Set(request.is_superuser.unwrap_or(false)),
            is_staff: ActiveValue::Set(request.is_staff.unwrap_or(false)),
            is_multi_login: ActiveValue::Set(request.is_multi_login.unwrap_or(false)),
            join_time: ActiveValue::Set(chrono::Utc::now().naive_utc()),
            last_login_time: ActiveValue::Set(None),
            dept_id: ActiveValue::Set(request.dept_id),
            created_time: ActiveValue::NotSet,
            updated_time: ActiveValue::NotSet,
            del_flag: ActiveValue::Set(0),
        };

        let created_user = UserRepo::create(user_model, &self.db)
            .await
            .map_err(|e| AppError::with_details(
                ErrorCode::DatabaseError,
                "用户创建失败",
                e.to_string(),
            ))?;

        // TODO: 分配角色（关联用户-角色表）

        Ok(CreateUserResponse {
            id: created_user.id,
            username: created_user.username,
            nickname: created_user.nickname,
            email: created_user.email,
            phone: created_user.phone,
            dept_id: created_user.dept_id,
            status: created_user.status,
            created_time: created_user.created_time.and_utc(),
        })
    }

    pub async fn update_user(
        &self,
        user_id: i64,
        request: &UpdateUserRequest,
    ) -> Result<(), AppError> {
        // 1. 检查用户是否存在
        let existing_user = UserRepo::find_by_id(user_id, &self.db)
            .await
            .map_err(|_| AppError::new(ErrorCode::UserNotFound))?;

        // 2. 验证邮箱唯一性（如果更新了邮箱）
        if let Some(email) = &request.email {
            if Some(email) != existing_user.email.as_ref() {
                if let Ok(_) = UserRepo::find_by_email(email, &self.db).await {
                    return Err(AppError::with_details(
                        ErrorCode::ResourceExists,
                        "用户更新失败",
                        "邮箱已存在",
                    ));
                }
            }
        }

        // 3. 构建更新数据
        let mut update_data = user::ActiveModel::default();
        update_data.id = ActiveValue::Set(user_id);

        if let Some(nickname) = &request.nickname {
            update_data.nickname = ActiveValue::Set(nickname.clone());
        }

        if let Some(email) = &request.email {
            update_data.email = ActiveValue::Set(Some(email.clone()));
        }

        if let Some(phone) = &request.phone {
            update_data.phone = ActiveValue::Set(Some(phone.clone()));
        }

        if let Some(avatar) = &request.avatar {
            update_data.avatar = ActiveValue::Set(Some(avatar.clone()));
        }

        if let Some(dept_id) = request.dept_id {
            update_data.dept_id = ActiveValue::Set(Some(dept_id));
        }

        if let Some(is_superuser) = request.is_superuser {
            update_data.is_superuser = ActiveValue::Set(is_superuser);
        }

        if let Some(is_staff) = request.is_staff {
            update_data.is_staff = ActiveValue::Set(is_staff);
        }

        if let Some(is_multi_login) = request.is_multi_login {
            update_data.is_multi_login = ActiveValue::Set(is_multi_login);
        }

        if let Some(status) = request.status {
            update_data.status = ActiveValue::Set(status);
        }

        // 4. 执行更新
        UserRepo::update(user_id, update_data, &self.db)
            .await
            .map_err(|e| AppError::with_details(
                ErrorCode::DatabaseError,
                "用户更新失败",
                e.to_string(),
            ))?;

        // TODO: 更新用户角色关联

        Ok(())
    }

    pub async fn delete_user(&self, user_id: i64) -> Result<(), AppError> {
        // 检查用户是否存在
        let _ = UserRepo::find_by_id(user_id, &self.db)
            .await
            .map_err(|_| AppError::new(ErrorCode::UserNotFound))?;

        // 执行软删除
        UserRepo::delete(user_id, &self.db)
            .await
            .map_err(|e| AppError::with_details(
                ErrorCode::DatabaseError,
                "用户删除失败",
                e.to_string(),
            ))?;

        Ok(())
    }

    pub async fn get_current_user(
        &self,
        user_id: i64,
    ) -> Result<crate::app::user::dto::CurrentUserResponse, AppError> {
        // 获取用户基本信息
        let user = UserRepo::find_by_id(user_id, &self.db)
            .await
            .map_err(|_| AppError::new(ErrorCode::UserNotFound))?;

        // 获取部门名称（部门查询失败时只记录日志，不中断请求）
        let dept_name = if let Some(dept_id) = user.dept_id {
            match crate::database::dept::Entity::find_by_id(dept_id)
                .one(&self.db)
                .await
            {
                Ok(dept) => dept.map(|d| d.name),
                Err(e) => {
                    error!(
                        "查询部门失败: user_id={}, dept_id={}, error={}",
                        user.id,
                        dept_id,
                        e
                    );
                    None
                }
            }
        } else {
            None
        };

        // 获取角色名称列表（角色查询失败时只记录日志，不中断请求）
        let role_ids = match crate::database::UserRoleCrud::find_roles_by_user(user.id, &self.db).await {
            Ok(ids) => ids,
            Err(e) => {
                error!("查询用户角色失败: user_id={}, error={}", user.id, e);
                Vec::new()
            }
        };
        let mut role_names = Vec::new();
        for role_id in role_ids {
            if let Ok(role) = crate::database::RoleCrud::find_by_id(role_id, &self.db).await {
                role_names.push(role.name);
            }
        }

        Ok(crate::app::user::dto::CurrentUserResponse {
            id: user.id,
            uuid: user.uuid,
            username: user.username,
            nickname: user.nickname,
            email: user.email,
            phone: user.phone,
            avatar: user.avatar,
            status: user.status,
            is_superuser: user.is_superuser,
            is_staff: user.is_staff,
            is_multi_login: user.is_multi_login,
            join_time: user.join_time.and_utc(),
            last_login_time: user.last_login_time.map(|t| t.and_utc()),
            dept_id: user.dept_id,
            dept: dept_name,
            roles: role_names,
        })
    }

    pub async fn get_user_detail(
        &self,
        user_id: i64,
    ) -> Result<UserDetailResponse, AppError> {
        // 1. 获取用户信息
        let user = UserRepo::find_by_id(user_id, &self.db)
            .await
            .map_err(|_| AppError::new(ErrorCode::UserNotFound))?;

        // 2. 获取用户角色
        // TODO: 实现获取用户角色的逻辑
        let roles = vec![];

        // 3. 获取部门信息
        // TODO: 实现获取部门名称的逻辑
        let dept_name = None;

        // 4. 构建响应
        Ok(UserDetailResponse {
            id: user.id,
            uuid: user.uuid,
            username: user.username,
            nickname: user.nickname,
            email: user.email,
            phone: user.phone,
            avatar: user.avatar,
            status: user.status,
            is_superuser: user.is_superuser,
            is_staff: user.is_staff,
            is_multi_login: user.is_multi_login,
            join_time: user.join_time.and_utc(),
            last_login_time: user.last_login_time.map(|t| t.and_utc()),
            dept_id: user.dept_id,
            dept_name,
            roles,
            created_time: user.created_time.and_utc(),
            updated_time: user.updated_time.and_utc(),
        })
    }

    pub async fn change_password(
        &self,
        request: &ChangePasswordRequest,
    ) -> Result<(), AppError> {
        // 1. 获取用户信息
        let user = UserRepo::find_by_id(request.user_id, &self.db)
            .await
            .map_err(|_| AppError::new(ErrorCode::UserNotFound))?;

        // 2. 验证旧密码
        let is_valid = if let Some(password) = &user.password {
            CryptoUtils::verify_password(&request.old_password, password)
                .await
                .map_err(|e| AppError::with_details(
                    ErrorCode::PasswordError,
                    "密码验证失败",
                    e.to_string(),
                ))?
        } else {
            false
        };

        if !is_valid {
            return Err(AppError::new(ErrorCode::PasswordError));
        }

        // 3. 加密新密码
        let hashed_new_password = CryptoUtils::hash_password(&request.new_password, None)
            .await
            .map_err(|e| AppError::with_details(
                ErrorCode::BusinessError,
                "密码加密失败",
                e.to_string(),
            ))?;

        // 4. 更新密码
        let mut update_data = user::ActiveModel::default();
        update_data.id = ActiveValue::Set(request.user_id);
        update_data.password = ActiveValue::Set(Some(hashed_new_password));

        UserRepo::update(request.user_id, update_data, &self.db)
            .await
            .map_err(|e| AppError::with_details(
                ErrorCode::DatabaseError,
                "密码更新失败",
                e.to_string(),
            ))?;

        Ok(())
    }

    pub async fn reset_password(
        &self,
        request: &ResetPasswordRequest,
    ) -> Result<(), AppError> {
        // 1. 检查用户是否存在
        let _ = UserRepo::find_by_id(request.user_id, &self.db)
            .await
            .map_err(|_| AppError::new(ErrorCode::UserNotFound))?;

        // 2. 加密新密码
        let hashed_new_password = CryptoUtils::hash_password(&request.new_password, None)
            .await
            .map_err(|e| AppError::with_details(
                ErrorCode::BusinessError,
                "密码加密失败",
                e.to_string(),
            ))?;

        // 3. 更新密码
        let mut update_data = user::ActiveModel::default();
        update_data.id = ActiveValue::Set(request.user_id);
        update_data.password = ActiveValue::Set(Some(hashed_new_password));

        UserRepo::update(request.user_id, update_data, &self.db)
            .await
            .map_err(|e| AppError::with_details(
                ErrorCode::DatabaseError,
                "密码重置失败",
                e.to_string(),
            ))?;

        Ok(())
    }

    pub async fn update_user_status(
        &self,
        user_id: i64,
        status: i32,
    ) -> Result<(), AppError> {
        // 检查用户是否存在
        let _ = UserRepo::find_by_id(user_id, &self.db)
            .await
            .map_err(|_| AppError::new(ErrorCode::UserNotFound))?;

        // 更新状态
        let mut update_data = user::ActiveModel::default();
        update_data.id = ActiveValue::Set(user_id);
        update_data.status = ActiveValue::Set(status);

        UserRepo::update(user_id, update_data, &self.db)
            .await
            .map_err(|e| AppError::with_details(
                ErrorCode::DatabaseError,
                "状态更新失败",
                e.to_string(),
            ))?;

        Ok(())
    }

    pub async fn batch_delete_users(
        &self,
        user_ids: &[i64],
    ) -> Result<(), AppError> {
        for &user_id in user_ids {
            self.delete_user(user_id).await?;
        }
        Ok(())
    }

    pub async fn can_delete_user(&self, user_id: i64) -> Result<bool, AppError> {
        let user = UserRepo::find_by_id(user_id, &self.db).await;

        match user {
            Ok(user) => {
                // 检查是否为超级管理员
                if user.is_superuser {
                    return Ok(false);
                }
                // 检查是否被删除
                if user.del_flag == 1 {
                    return Ok(false);
                }
                Ok(true)
            }
            Err(_) => Ok(false),
        }
    }

    pub async fn get_users_paginated(
        &self,
        query: &UserPaginationQuery,
    ) -> Result<crate::common::pagination::PageData<UserListItem>, AppError> {
        // 1. 查询用户列表和总数
        let (users, total) = UserRepo::find_with_pagination(&UserRepo, query, &self.db)
            .await
            .map_err(|e| AppError::with_details(
                ErrorCode::DatabaseError,
                "用户查询失败",
                e.to_string(),
            ))?;

        // 2. 获取分页参数
        let page = query.page.unwrap_or(1) as i64;
        let size = query.size.unwrap_or(20) as i64;

        // 3. 转换用户列表为响应DTO
        let mut user_list = Vec::new();
        for user in users {
            // TODO: 获取用户角色和部门名称
            let roles = vec![];
            let dept_name = None;

            user_list.push(UserListItem {
                id: user.id,
                username: user.username,
                nickname: user.nickname,
                email: user.email,
                phone: user.phone,
                avatar: user.avatar,
                status: user.status,
                dept_name,
                roles,
                last_login_time: user.last_login_time.map(|t| t.and_utc()),
                created_time: user.created_time.and_utc(),
            });
        }

        // 4. 使用标准 PageData 返回
        Ok(crate::common::pagination::PageData::new(
            user_list,
            total as i64,
            page,
            size,
        ))
    }

    pub async fn import_users(
        &self,
        request: &ImportUsersRequest,
    ) -> Result<ImportUsersResponse, AppError> {
        info!("Starting user import");

        // 1. 解码文件数据
        let file_data = general_purpose::STANDARD
            .decode(&request.file_data)
            .map_err(|e| AppError::with_details(
                ErrorCode::BadRequest,
                "文件解码失败",
                e.to_string(),
            ))?;

        // 2. 解析CSV数据
        let mut reader = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(Cursor::new(file_data));

        let mut success_count = 0;
        let mut failure_count = 0;
        let mut errors = Vec::new();

        for (row_number, result) in reader.records().enumerate() {
            match result {
                Ok(record) => {
                    // 解析CSV记录为UserImportTemplateItem
                    match self.parse_csv_record(&record) {
                        Ok(user_data) => {
                            // 创建用户
                            match self.create_user_from_import(&user_data).await {
                                Ok(_) => {
                                    success_count += 1;
                                    info!("Successfully imported user: {}", user_data.username);
                                }
                                Err(e) => {
                                    failure_count += 1;
                                    errors.push(ImportError {
                                        row_number: row_number + 1,
                                        row_data: Some(record.iter().collect::<Vec<_>>().join(",")),
                                        field: None,
                                        message: e.message.clone(),
                                        error_code: e.code.to_string(),
                                    });
                                    error!("Failed to import user at row {}: {}", row_number + 1, e.message);
                                }
                            }
                        }
                        Err(e) => {
                            failure_count += 1;
                            errors.push(ImportError {
                                row_number: row_number + 1,
                                row_data: Some(record.iter().collect::<Vec<_>>().join(",")),
                                field: e.0,
                                message: e.1,
                                error_code: "VALIDATION_ERROR".to_string(),
                            });
                        }
                    }
                }
                Err(e) => {
                    failure_count += 1;
                    errors.push(ImportError {
                        row_number: row_number + 1,
                        row_data: None,
                        field: None,
                        message: format!("CSV读取失败: {}", e),
                        error_code: "CSV_READ_ERROR".to_string(),
                    });
                }
            }
        }

        let result = ImportResult {
            success: failure_count == 0,
            message: if failure_count == 0 {
                "所有用户导入成功".to_string()
            } else {
                format!("成功导入 {} 个用户，失败 {} 个用户", success_count, failure_count)
            },
        };

        info!("User import completed: {} success, {} failure", success_count, failure_count);

        Ok(ImportUsersResponse {
            result,
            success_count,
            failure_count,
            errors,
        })
    }

    pub async fn export_users(
        &self,
        request: &ExportUsersRequest,
    ) -> Result<ExportUsersResponse, AppError> {
        info!("Starting user export");

        // 1. 构建查询条件
        let mut query = UserPaginationQuery::default();
        query.page = Some(1);
        query.size = Some(10000); // 导出所有用户

        if let Some(_dept_id) = request.dept_id {
            // TODO: 根据部门ID筛选
        }

        if let Some(_status) = request.status {
            // TODO: 根据状态筛选
        }

        // 2. 查询用户数据
        let users = UserRepo::find_with_pagination(&UserRepo, &query, &self.db)
            .await
            .map_err(|e| AppError::with_details(
                ErrorCode::DatabaseError,
                "用户查询失败",
                e.to_string(),
            ))?.0;

        // 3. 转换为导出格式
        let export_data_len = users.len();
        let mut export_data = Vec::new();
        for user in users {
            export_data.push(UserExportItem {
                id: user.id,
                username: user.username,
                nickname: user.nickname,
                email: user.email,
                phone: user.phone,
                dept_name: None, // TODO: 获取部门名称
                role_names: None, // TODO: 获取角色名称
                status: user.status,
                status_name: if user.status == 1 { "启用".to_string() } else { "禁用".to_string() },
                is_superuser: user.is_superuser,
                is_staff: user.is_staff,
                is_multi_login: user.is_multi_login,
                last_login_time: user.last_login_time.map(|t| t.and_utc()),
                created_time: user.created_time.and_utc(),
                remark: None, // TODO: 获取备注
            });
        }

        // 4. 生成CSV数据
        let mut csv_data = Vec::new();
        {
            let mut writer = WriterBuilder::new()
                .has_headers(true)
                .from_writer(&mut csv_data);

            for item in export_data {
                writer.write_record(&[
                    item.username,
                    item.nickname,
                    item.email.unwrap_or_default(),
                    item.phone.unwrap_or_default(),
                    item.dept_name.unwrap_or_default(),
                    item.role_names.unwrap_or_default(),
                    item.status.to_string(),
                    item.is_superuser.to_string(),
                    item.is_staff.to_string(),
                    item.is_multi_login.to_string(),
                    item.created_time.to_rfc3339(),
                ]).map_err(|e| AppError::with_details(
                    ErrorCode::BusinessError,
                    "CSV生成失败",
                    e.to_string(),
                ))?;
            }

            writer.flush().map_err(|e| AppError::with_details(
                ErrorCode::BusinessError,
                "CSV写入失败",
                e.to_string(),
            ))?;
        }

        // 5. 编码为Base64
        let file_data = general_purpose::STANDARD.encode(&csv_data);
        let file_name = format!("users_export_{}.csv", Utc::now().format("%Y%m%d%H%M%S"));
        let file_size = csv_data.len() as i64;

        info!("User export completed: {} users exported", export_data_len);

        Ok(ExportUsersResponse {
            file_id: Utc::now().timestamp_nanos_opt().unwrap_or(0),
            file_name,
            file_size,
            file_data,
            content_type: "text/csv".to_string(),
            file_suffix: "csv".to_string(),
            export_time: Utc::now(),
            download_url: None,
        })
    }

    pub async fn download_template(
        &self,
        request: &DownloadTemplateRequest,
    ) -> Result<DownloadTemplateResponse, AppError> {
        info!("Generating user import template in format: {}", request.template_format);

        // 创建模板数据
        let template_data = vec![
            UserImportTemplateItem {
                username: "user001".to_string(),
                nickname: "张三".to_string(),
                password: "123456".to_string(),
                email: Some("zhangsan@example.com".to_string()),
                phone: Some("13800138001".to_string()),
                dept_name: Some("技术部".to_string()),
                role_names: Some("普通用户".to_string()),
                status: Some(1),
                is_superuser: Some("false".to_string()),
                is_staff: Some("false".to_string()),
                is_multi_login: Some("false".to_string()),
                remark: Some("示例用户1".to_string()),
            },
            UserImportTemplateItem {
                username: "user002".to_string(),
                nickname: "李四".to_string(),
                password: "123456".to_string(),
                email: Some("lisi@example.com".to_string()),
                phone: Some("13800138002".to_string()),
                dept_name: Some("产品部".to_string()),
                role_names: Some("管理员,普通用户".to_string()),
                status: Some(1),
                is_superuser: Some("false".to_string()),
                is_staff: Some("true".to_string()),
                is_multi_login: Some("true".to_string()),
                remark: Some("示例用户2".to_string()),
            },
        ];

        // 生成CSV数据
        let mut csv_data = Vec::new();
        {
            let mut writer = WriterBuilder::new()
                .has_headers(true)
                .from_writer(&mut csv_data);

            for item in template_data {
                writer.write_record(&[
                    item.username,
                    item.nickname,
                    item.password,
                    item.email.unwrap_or_default(),
                    item.phone.unwrap_or_default(),
                    item.dept_name.unwrap_or_default(),
                    item.role_names.unwrap_or_default(),
                    item.status.unwrap_or(1).to_string(),
                    item.is_superuser.unwrap_or_default(),
                    item.is_staff.unwrap_or_default(),
                    item.is_multi_login.unwrap_or_default(),
                    item.remark.unwrap_or_default(),
                ]).map_err(|e| AppError::with_details(
                    ErrorCode::BusinessError,
                    "模板生成失败",
                    e.to_string(),
                ))?;
            }

            writer.flush().map_err(|e| AppError::with_details(
                ErrorCode::BusinessError,
                "模板写入失败",
                e.to_string(),
            ))?;
        }

        // 编码为Base64
        let file_data = general_purpose::STANDARD.encode(&csv_data);
        let file_name = format!("user_import_template.{}", request.template_format);
        let file_size = csv_data.len() as i64;

        Ok(DownloadTemplateResponse {
            file_id: Utc::now().timestamp_nanos_opt().unwrap_or(0),
            file_name,
            file_size,
            file_data,
            content_type: "text/csv".to_string(),
            file_suffix: request.template_format.clone(),
            description: Some("用户导入模板文件，请按格式填写用户信息".to_string()),
        })
    }

    pub async fn batch_import_users(
        &self,
        request: &BatchImportUsersRequest,
    ) -> Result<BatchImportUsersResponse, AppError> {
        info!("Starting batch import of {} users", request.users.len());

        let mut success_count = 0;
        let mut failure_count = 0;
        let mut success_users = Vec::new();
        let mut errors = Vec::new();

        for (index, user_data) in request.users.iter().enumerate() {
            match self.create_user_from_import(user_data).await {
                Ok(user) => {
                    success_count += 1;
                    success_users.push(user);
                    info!("Successfully imported user: {}", user_data.username);
                }
                Err(e) => {
                    failure_count += 1;
                    errors.push(ImportError {
                        row_number: index + 1,
                        row_data: Some(serde_json::to_string(user_data).unwrap_or_default()),
                        field: None,
                        message: e.message.clone(),
                        error_code: e.code.to_string(),
                    });
                    error!("Failed to import user at index {}: {}", index, e.message);
                }
            }
        }

        info!("Batch import completed: {} success, {} failure", success_count, failure_count);

        Ok(BatchImportUsersResponse {
            success_count,
            failure_count,
            success_users,
            errors,
        })
    }

    fn parse_csv_record(
        &self,
        record: &csv::StringRecord,
    ) -> Result<UserImportTemplateItem, (Option<String>, String)> {
        let get_field = |index: usize, _required: bool| -> Option<String> {
            if index < record.len() {
                let value = record[index].trim();
                if value.is_empty() {
                    None
                } else {
                    Some(value.to_string())
                }
            } else {
                None
            }
        };

        // 必填字段
        let username = get_field(0, true).ok_or((Some("username".to_string()), "用户名不能为空".to_string()))?;
        let nickname = get_field(1, true).ok_or((Some("nickname".to_string()), "昵称不能为空".to_string()))?;
        let password = get_field(2, true).ok_or((Some("password".to_string()), "密码不能为空".to_string()))?;

        // 可选字段
        let email = get_field(3, false);
        let phone = get_field(4, false);
        let dept_name = get_field(5, false);
        let role_names = get_field(6, false);
        let status = get_field(7, false).and_then(|s| s.parse::<i32>().ok());
        let is_superuser = get_field(8, false);
        let is_staff = get_field(9, false);
        let is_multi_login = get_field(10, false);
        let remark = get_field(11, false);

        Ok(UserImportTemplateItem {
            username,
            nickname,
            password,
            email,
            phone,
            dept_name,
            role_names,
            status,
            is_superuser,
            is_staff,
            is_multi_login,
            remark,
        })
    }

    async fn create_user_from_import(
        &self,
        user_data: &UserImportTemplateItem,
    ) -> Result<CreateUserResponse, AppError> {
        // 检查用户名是否已存在
        if UserRepo::exists_by_username(&user_data.username, &self.db).await? {
            return Err(AppError::with_details(
                ErrorCode::ResourceExists,
                "用户创建失败",
                "用户名已存在",
            ));
        }

        // 检查邮箱是否已存在
        if let Some(ref email) = user_data.email {
            if UserRepo::exists_by_email(email, &self.db).await? {
                return Err(AppError::with_details(
                    ErrorCode::ResourceExists,
                    "用户创建失败",
                    "邮箱已存在",
                ));
            }
        }

        // 加密密码
        let hashed_password = CryptoUtils::hash_password(&user_data.password, None)
            .await
            .map_err(|e| AppError::with_details(
                ErrorCode::BusinessError,
                "密码加密失败",
                e.to_string(),
            ))?;

        // 创建用户数据
        let user_model = user::ActiveModel {
            id: ActiveValue::NotSet,
            uuid: ActiveValue::Set(uuid::Uuid::new_v4().to_string()),
            username: ActiveValue::Set(user_data.username.clone()),
            nickname: ActiveValue::Set(user_data.nickname.clone()),
            password: ActiveValue::Set(Some(hashed_password)),
            salt: ActiveValue::Set(None),
            email: ActiveValue::Set(user_data.email.clone()),
            phone: ActiveValue::Set(user_data.phone.clone()),
            avatar: ActiveValue::Set(None),
            status: ActiveValue::Set(user_data.status.unwrap_or(1)),
            is_superuser: ActiveValue::Set(
                user_data.is_superuser
                    .as_ref()
                    .map(|s| s.to_lowercase() == "true")
                    .unwrap_or(false)
            ),
            is_staff: ActiveValue::Set(
                user_data.is_staff
                    .as_ref()
                    .map(|s| s.to_lowercase() == "true")
                    .unwrap_or(false)
            ),
            is_multi_login: ActiveValue::Set(
                user_data.is_multi_login
                    .as_ref()
                    .map(|s| s.to_lowercase() == "true")
                    .unwrap_or(false)
            ),
            join_time: ActiveValue::Set(chrono::Utc::now().naive_utc()),
            last_login_time: ActiveValue::Set(None),
            dept_id: ActiveValue::Set(None), // TODO: 根据部门名称查找部门ID
            created_time: ActiveValue::NotSet,
            updated_time: ActiveValue::NotSet,
            del_flag: ActiveValue::Set(0),
        };

        // 保存到数据库
        let created_user = UserRepo::create(user_model, &self.db)
            .await
            .map_err(|e| AppError::with_details(
                ErrorCode::DatabaseError,
                "用户创建失败",
                e.to_string(),
            ))?;

        // TODO: 分配角色

        Ok(CreateUserResponse {
            id: created_user.id,
            username: created_user.username,
            nickname: created_user.nickname,
            email: created_user.email,
            phone: created_user.phone,
            dept_id: created_user.dept_id,
            status: created_user.status,
            created_time: created_user.created_time.and_utc(),
        })
    }

    /// 更新用户权限（切换状态）
    pub async fn update_permission_toggle(
        &self,
        user_id: i64,
        permission_type: crate::common::enums::UserPermissionType,
    ) -> Result<(), AppError> {
        use tracing::info;
        
        // 1. 查询用户当前状态
        let user = UserRepo::find_by_id(user_id, &self.db)
            .await
            .map_err(|_| AppError::new(ErrorCode::UserNotFound))?;

        // 2. 构建更新数据
        let mut update_data = user::ActiveModel::default();
        update_data.id = ActiveValue::Set(user_id);

        // 3. 根据权限类型切换相应字段
        match permission_type {
            crate::common::enums::UserPermissionType::Superuser => {
                let new_value = !user.is_superuser;
                update_data.is_superuser = ActiveValue::Set(new_value);
                info!("切换用户 {} 的超级用户权限: {} -> {}", user_id, user.is_superuser, new_value);
            }
            crate::common::enums::UserPermissionType::Staff => {
                let new_value = !user.is_staff;
                update_data.is_staff = ActiveValue::Set(new_value);
                info!("切换用户 {} 的员工权限: {} -> {}", user_id, user.is_staff, new_value);
            }
            crate::common::enums::UserPermissionType::Status => {
                let new_status = if user.status == 1 { 0 } else { 1 };
                update_data.status = ActiveValue::Set(new_status);
                info!("切换用户 {} 的状态: {} -> {}", user_id, user.status, new_status);
            }
            crate::common::enums::UserPermissionType::MultiLogin => {
                let new_value = !user.is_multi_login;
                update_data.is_multi_login = ActiveValue::Set(new_value);
                info!("切换用户 {} 的多点登录权限: {} -> {}", user_id, user.is_multi_login, new_value);
            }
        }

        // 4. 执行更新
        UserRepo::update(user_id, update_data, &self.db)
            .await
            .map_err(|e| AppError::with_details(
                ErrorCode::DatabaseError,
                "权限更新失败",
                e.to_string(),
            ))?;

        Ok(())
    }

    pub async fn update_permission(
        &self,
        user_id: i64,
        permission_type: crate::common::enums::UserPermissionType,
        enable: bool,
    ) -> Result<(), AppError> {
        // 1. 检查用户是否存在
        let _ = UserRepo::find_by_id(user_id, &self.db)
            .await
            .map_err(|_| AppError::new(ErrorCode::UserNotFound))?;

        // 2. 构建更新数据
        let mut update_data = user::ActiveModel::default();
        update_data.id = ActiveValue::Set(user_id);

        // 3. 根据权限类型更新相应字段
        match permission_type {
            crate::common::enums::UserPermissionType::Superuser => {
                update_data.is_superuser = ActiveValue::Set(enable);
            }
            crate::common::enums::UserPermissionType::Staff => {
                update_data.is_staff = ActiveValue::Set(enable);
            }
            crate::common::enums::UserPermissionType::Status => {
                update_data.status = ActiveValue::Set(if enable { 1 } else { 0 });
            }
            crate::common::enums::UserPermissionType::MultiLogin => {
                update_data.is_multi_login = ActiveValue::Set(enable);
            }
        }

        // 4. 执行更新
        UserRepo::update(user_id, update_data, &self.db)
            .await
            .map_err(|e| AppError::with_details(
                ErrorCode::DatabaseError,
                "权限更新失败",
                e.to_string(),
            ))?;

        Ok(())
    }

    pub async fn admin_reset_password(
        &self,
        user_id: i64,
        new_password: &str,
    ) -> Result<(), AppError> {
        // 1. 检查用户是否存在
        let _ = UserRepo::find_by_id(user_id, &self.db)
            .await
            .map_err(|_| AppError::new(ErrorCode::UserNotFound))?;

        // 2. 加密新密码
        let hashed_new_password = crate::utils::encrypt::CryptoUtils::hash_password(new_password, None)
            .await
            .map_err(|e| AppError::with_details(
                ErrorCode::BusinessError,
                "密码加密失败",
                e.to_string(),
            ))?;

        // 3. 更新密码
        let mut update_data = user::ActiveModel::default();
        update_data.id = ActiveValue::Set(user_id);
        update_data.password = ActiveValue::Set(Some(hashed_new_password));

        UserRepo::update(user_id, update_data, &self.db)
            .await
            .map_err(|e| AppError::with_details(
                ErrorCode::DatabaseError,
                "密码重置失败",
                e.to_string(),
            ))?;

        Ok(())
    }
}
