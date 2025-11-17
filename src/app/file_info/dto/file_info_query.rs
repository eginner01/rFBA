/// 文件信息查询 DTO

use serde::{Deserialize, Serialize};
use validator::Validate;

/// 分页查询参数
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct FileInfoQuery {
    /// 页码
    #[validate(range(min = 1, message = "页码必须大于0"))]
    pub page_num: Option<usize>,

    /// 每页数量
    #[validate(range(min = 1, max = 100, message = "每页数量必须在1-100之间"))]
    pub page_size: Option<usize>,

    /// 文件名
    pub file_name: Option<String>,

    /// 原始文件名
    pub original_name: Option<String>,

    /// 文件后缀
    pub file_suffix: Option<String>,

    /// 存储类型
    pub storage_type: Option<i32>,

    /// 访问权限
    pub access_permission: Option<i32>,

    /// 上传者
    pub uploader: Option<String>,
}

/// 下载文件请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadFileRequest {
    /// 文件ID
    pub file_id: i64,
    /// 下载者
    pub downloader: Option<String>,
}

/// 文件统计查询
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct FileStatisticsQuery {
    /// 上传者
    pub uploader: Option<String>,

    /// 时间范围（天）
    #[validate(range(min = 1, max = 365, message = "时间范围必须在1-365天之间"))]
    pub time_range_days: Option<i32>,
}

/// 分页响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfoListResponse {
    /// 文件列表
    pub list: Vec<FileInfoListItem>,
    /// 总数
    pub total: usize,
    /// 页码
    pub page_num: usize,
    /// 每页数量
    pub page_size: usize,
}

/// 文件列表项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfoListItem {
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
