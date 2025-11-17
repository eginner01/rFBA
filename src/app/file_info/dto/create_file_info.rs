/// 文件信息创建和更新 DTO

use serde::{Deserialize, Serialize};
use validator::Validate;

/// 创建文件信息请求
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateFileInfoRequest {
    /// 文件名
    #[validate(length(min = 1, max = 255, message = "文件名长度必须在1-255个字符之间"))]
    pub file_name: String,

    /// 原始文件名
    #[validate(length(min = 1, max = 255, message = "原始文件名长度必须在1-255个字符之间"))]
    pub original_name: String,

    /// 文件后缀
    #[validate(length(min = 1, max = 20, message = "文件后缀长度必须在1-20个字符之间"))]
    pub file_suffix: String,

    /// 文件大小（字节）
    #[validate(range(min = 0, message = "文件大小必须大于等于0"))]
    pub file_size: i64,

    /// MIME类型
    #[validate(length(min = 1, max = 100, message = "MIME类型长度必须在1-100个字符之间"))]
    pub content_type: String,

    /// 文件路径
    #[validate(length(min = 1, max = 500, message = "文件路径长度必须在1-500个字符之间"))]
    pub file_path: String,

    /// 存储方式（1:本地 2:OSS 3:S3）
    #[validate(range(min = 1, max = 3, message = "存储方式必须是1-3之间的值"))]
    pub storage_type: i32,

    /// 文件SHA256哈希
    pub file_hash: Option<String>,

    /// 上传者
    #[validate(length(min = 1, max = 100, message = "上传者长度必须在1-100个字符之间"))]
    pub uploader: String,

    /// 访问权限（1:私有 2:公开 3:组织内）
    #[validate(range(min = 1, max = 3, message = "访问权限必须是1-3之间的值"))]
    pub access_permission: i32,

    /// 备注
    pub remark: Option<String>,
}

/// 更新文件信息请求
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateFileInfoRequest {
    /// 文件ID
    #[validate(range(min = 1, message = "文件ID必须大于0"))]
    pub file_id: i64,

    /// 文件名
    #[validate(length(min = 1, max = 255, message = "文件名长度必须在1-255个字符之间"))]
    pub file_name: String,

    /// 访问权限（1:私有 2:公开 3:组织内）
    #[validate(range(min = 1, max = 3, message = "访问权限必须是1-3之间的值"))]
    pub access_permission: i32,

    /// 备注
    pub remark: Option<String>,
}

/// 删除文件信息请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteFileInfoRequest {
    /// 文件ID列表
    pub file_ids: Vec<i64>,
}

/// 创建文件信息响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFileInfoResponse {
    /// 文件ID
    pub file_id: i64,
    /// 文件名
    pub file_name: String,
    /// 原始文件名
    pub original_name: String,
    /// 上传时间
    pub upload_time: chrono::DateTime<chrono::Utc>,
}

/// 更新文件信息响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFileInfoResponse {
    /// 文件ID
    pub file_id: i64,
    /// 文件名
    pub file_name: String,
    /// 更新时间
    pub updated_time: chrono::DateTime<chrono::Utc>,
}

/// 文件上传请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileUploadRequest {
    /// 是否覆盖已存在文件
    pub overwrite: bool,
    /// 访问权限（1:私有 2:公开 3:组织内）
    pub access_permission: Option<i32>,
}
