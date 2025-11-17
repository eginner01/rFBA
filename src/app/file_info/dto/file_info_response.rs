/// 文件信息响应 DTO

use serde::{Deserialize, Serialize};

use super::file_info_query::FileInfoListItem;

/// 文件详情响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfoDetailResponse {
    /// 文件ID
    pub file_id: i64,
    /// 文件名
    pub file_name: String,
    /// 原始文件名
    pub original_name: String,
    /// 文件后缀
    pub file_suffix: String,
    /// 文件大小
    pub file_size: i64,
    /// 文件大小描述
    pub size_description: String,
    /// MIME类型
    pub content_type: String,
    /// 文件路径
    pub file_path: String,
    /// 存储类型
    pub storage_type: i32,
    /// 存储类型名称
    pub storage_type_name: String,
    /// 文件哈希
    pub file_hash: Option<String>,
    /// 上传者
    pub uploader: String,
    /// 访问权限
    pub access_permission: i32,
    /// 访问权限名称
    pub access_permission_name: String,
    /// 下载次数
    pub download_count: i32,
    /// 上传时间
    pub upload_time: chrono::DateTime<chrono::Utc>,
    /// 更新时间
    pub updated_time: chrono::DateTime<chrono::Utc>,
    /// 备注
    pub remark: Option<String>,
}

/// 下载文件响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadFileResponse {
    /// 文件ID
    pub file_id: i64,
    /// 文件名
    pub file_name: String,
    /// 文件内容
    pub file_data: Vec<u8>,
    /// MIME类型
    pub content_type: String,
    /// 文件大小
    pub file_size: i64,
}

/// 存储类型统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageTypeStatistics {
    /// 存储类型
    pub storage_type: i32,
    /// 存储类型名称
    pub storage_type_name: String,
    /// 文件数量
    pub file_count: usize,
    /// 总大小
    pub total_size: i64,
    /// 总大小描述
    pub total_size_description: String,
}

/// 访问权限统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessPermissionStatistics {
    /// 访问权限
    pub access_permission: i32,
    /// 访问权限名称
    pub access_permission_name: String,
    /// 文件数量
    pub file_count: usize,
}

/// 文件统计响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileStatisticsResponse {
    /// 总文件数
    pub total_files: usize,
    /// 总大小
    pub total_size: i64,
    /// 总大小描述
    pub total_size_description: String,
    /// 存储类型统计
    pub storage_type_stats: Vec<StorageTypeStatistics>,
    /// 访问权限统计
    pub access_permission_stats: Vec<AccessPermissionStatistics>,
    /// 最受欢迎文件（按下载次数）
    pub top_downloaded: Vec<FileInfoListItem>,
    /// 最近上传文件
    pub recent_uploads: Vec<FileInfoListItem>,
}
