/// 用户导入导出 DTO

use serde::{Deserialize, Serialize};
use validator::Validate;

/// 用户导入请求
#[derive(Debug, Deserialize, Validate)]
pub struct ImportUsersRequest {
    /// 文件内容（CSV或Excel格式的Base64编码）
    pub file_data: String,

    /// 文件类型（csv, xlsx, xls）
    #[validate(length(min = 1))]
    pub file_type: String,

    /// 是否跳过重复数据
    pub skip_duplicates: Option<bool>,

    /// 是否更新已存在的数据
    pub update_existing: Option<bool>,

    /// 导入操作的执行人
    pub operator: Option<String>,
}

/// 用户导入响应
#[derive(Debug, Serialize)]
pub struct ImportUsersResponse {
    /// 导入结果
    pub result: ImportResult,

    /// 成功导入的用户数
    pub success_count: usize,

    /// 失败的用户数
    pub failure_count: usize,

    /// 错误详情列表
    pub errors: Vec<ImportError>,
}

/// 导入结果
#[derive(Debug, Serialize)]
pub struct ImportResult {
    /// 是否成功
    pub success: bool,

    /// 结果消息
    pub message: String,
}

/// 导入错误详情
#[derive(Debug, Serialize)]
pub struct ImportError {
    /// 错误行号（CSV中的行号）
    pub row_number: usize,

    /// 错误数据（原始行数据）
    pub row_data: Option<String>,

    /// 错误字段
    pub field: Option<String>,

    /// 错误消息
    pub message: String,

    /// 错误代码
    pub error_code: String,
}

/// 用户导出请求
#[derive(Debug, Deserialize, Validate)]
pub struct ExportUsersRequest {
    /// 导出格式（csv, xlsx, json）
    #[validate(length(min = 1))]
    pub export_format: String,

    /// 要导出的字段列表（为空则导出所有字段）
    pub fields: Option<Vec<String>>,

    /// 部门ID筛选
    pub dept_id: Option<i64>,

    /// 状态筛选（0:禁用 1:启用）
    pub status: Option<i32>,

    /// 角色ID筛选
    pub role_id: Option<i64>,

    /// 创建时间范围 - 开始
    pub created_time_start: Option<chrono::DateTime<chrono::Utc>>,

    /// 创建时间范围 - 结束
    pub created_time_end: Option<chrono::DateTime<chrono::Utc>>,

    /// 导出操作的执行人
    pub operator: Option<String>,
}

/// 用户导出响应
#[derive(Debug, Serialize)]
pub struct ExportUsersResponse {
    /// 导出的文件ID
    pub file_id: i64,

    /// 文件名
    pub file_name: String,

    /// 文件大小（字节）
    pub file_size: i64,

    /// 文件内容（Base64编码）
    pub file_data: String,

    /// MIME类型
    pub content_type: String,

    /// 文件后缀
    pub file_suffix: String,

    /// 导出时间
    pub export_time: chrono::DateTime<chrono::Utc>,

    /// 下载链接
    pub download_url: Option<String>,
}

/// 用户模板下载请求
#[derive(Debug, Deserialize)]
pub struct DownloadTemplateRequest {
    /// 模板格式（csv, xlsx）
    pub template_format: String,
}

/// 用户模板下载响应
#[derive(Debug, Serialize)]
pub struct DownloadTemplateResponse {
    /// 模板文件ID
    pub file_id: i64,

    /// 模板文件名
    pub file_name: String,

    /// 模板文件大小（字节）
    pub file_size: i64,

    /// 模板文件内容（Base64编码）
    pub file_data: String,

    /// MIME类型
    pub content_type: String,

    /// 文件后缀
    pub file_suffix: String,

    /// 模板说明
    pub description: Option<String>,
}

/// 用户导入模板数据项（用于生成模板）
#[derive(Debug, Serialize, Deserialize)]
pub struct UserImportTemplateItem {
    /// 用户名（必填）
    pub username: String,

    /// 昵称（必填）
    pub nickname: String,

    /// 密码（必填）
    pub password: String,

    /// 邮箱（可选）
    pub email: Option<String>,

    /// 手机号（可选）
    pub phone: Option<String>,

    /// 部门名称（可选）
    pub dept_name: Option<String>,

    /// 角色名称（可选，多个角色用逗号分隔）
    pub role_names: Option<String>,

    /// 状态（0:禁用 1:启用，可选，默认为1）
    pub status: Option<i32>,

    /// 是否超级管理员（true/false，可选，默认为false）
    pub is_superuser: Option<String>,

    /// 是否有后台管理权限（true/false，可选，默认为false）
    pub is_staff: Option<String>,

    /// 是否允许多端登录（true/false，可选，默认为false）
    pub is_multi_login: Option<String>,

    /// 备注（可选）
    pub remark: Option<String>,
}

/// 用户导出数据项
#[derive(Debug, Serialize, Deserialize)]
pub struct UserExportItem {
    /// 用户ID
    pub id: i64,

    /// 用户名
    pub username: String,

    /// 昵称
    pub nickname: String,

    /// 邮箱
    pub email: Option<String>,

    /// 手机号
    pub phone: Option<String>,

    /// 部门名称
    pub dept_name: Option<String>,

    /// 角色名称列表
    pub role_names: Option<String>,

    /// 状态
    pub status: i32,

    /// 状态名称
    pub status_name: String,

    /// 是否超级管理员
    pub is_superuser: bool,

    /// 是否有后台管理权限
    pub is_staff: bool,

    /// 是否允许多端登录
    pub is_multi_login: bool,

    /// 最后登录时间
    pub last_login_time: Option<chrono::DateTime<chrono::Utc>>,

    /// 创建时间
    pub created_time: chrono::DateTime<chrono::Utc>,

    /// 备注
    pub remark: Option<String>,
}

/// 导入验证错误
#[derive(Debug, Serialize)]
pub struct ImportValidationError {
    /// 字段名
    pub field: String,

    /// 错误消息
    pub message: String,

    /// 错误值
    pub value: Option<String>,
}

/// 批量导入用户请求（用于API）
#[derive(Debug, Deserialize)]
pub struct BatchImportUsersRequest {
    /// 用户数据列表
    pub users: Vec<UserImportTemplateItem>,

    /// 是否跳过重复数据
    pub skip_duplicates: Option<bool>,

    /// 是否更新已存在的数据
    pub update_existing: Option<bool>,

    /// 导入操作的执行人
    pub operator: Option<String>,
}

/// 批量导入用户响应
#[derive(Debug, Serialize)]
pub struct BatchImportUsersResponse {
    /// 成功导入的用户数
    pub success_count: usize,

    /// 失败的用户数
    pub failure_count: usize,

    /// 导入的用户列表
    pub success_users: Vec<CreateUserResponse>,

    /// 错误详情列表
    pub errors: Vec<ImportError>,
}

use super::create_user::CreateUserResponse;
