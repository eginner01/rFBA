/// 文件预览和缩略图 DTO

use serde::{Deserialize, Serialize};
use validator::Validate;

/// 文件预览请求
#[derive(Debug, Deserialize, Validate)]
pub struct PreviewFileRequest {
    /// 文件ID
    #[validate(range(min = 1))]
    pub file_id: i64,

    /// 预览参数
    pub options: Option<PreviewOptions>,
}

/// 预览选项
#[derive(Debug, Deserialize, Serialize)]
pub struct PreviewOptions {
    /// 图片预览宽度
    pub width: Option<u32>,

    /// 图片预览高度
    pub height: Option<u32>,

    /// 图片质量（1-100）
    pub quality: Option<u8>,

    /// 是否保持原始比例
    pub keep_aspect_ratio: Option<bool>,

    /// PDF预览页码（从1开始）
    pub page_number: Option<u32>,

    /// PDF预览总页数
    pub total_pages: Option<u32>,

    /// 文本编码格式
    pub encoding: Option<String>,

    /// 文本最大行数
    pub max_lines: Option<u32>,
}

/// 文件预览响应
#[derive(Debug, Serialize)]
pub struct PreviewFileResponse {
    /// 文件ID
    pub file_id: i64,

    /// 文件名
    pub file_name: String,

    /// 预览文件内容（Base64编码）
    pub preview_data: Vec<u8>,

    /// MIME类型
    pub content_type: String,

    /// 文件大小（字节）
    pub file_size: i64,

    /// 宽度（图片或视频）
    pub width: Option<u32>,

    /// 高度（图片或视频）
    pub height: Option<u32>,

    /// 持续时间（视频或音频，单位：秒）
    pub duration: Option<f64>,

    /// 页数（PDF）
    pub page_count: Option<u32>,

    /// 当前页码（PDF）
    pub current_page: Option<u32>,

    /// 编码格式（文本）
    pub encoding: Option<String>,

    /// 预览时间
    pub preview_time: chrono::DateTime<chrono::Utc>,
}

/// 缩略图生成请求
#[derive(Debug, Deserialize, Validate)]
pub struct GenerateThumbnailRequest {
    /// 文件ID
    #[validate(range(min = 1))]
    pub file_id: i64,

    /// 缩略图尺寸
    pub size: ThumbnailSize,

    /// 缩略图质量（1-100）
    pub quality: Option<u8>,

    /// 输出格式（jpg, png, webp）
    pub output_format: Option<String>,

    /// 是否保持原始比例
    pub keep_aspect_ratio: Option<bool>,
}

/// 缩略图尺寸
#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub enum ThumbnailSize {
    /// 小尺寸 (100x100)
    Small,
    /// 中尺寸 (300x300)
    Medium,
    /// 大尺寸 (800x600)
    Large,
    /// 自定义尺寸
    Custom {
        /// 宽度
        width: u32,
        /// 高度
        height: u32,
    },
}

impl ThumbnailSize {
    /// 获取尺寸
    pub fn get_dimensions(&self) -> (u32, u32) {
        match self {
            ThumbnailSize::Small => (100, 100),
            ThumbnailSize::Medium => (300, 300),
            ThumbnailSize::Large => (800, 600),
            ThumbnailSize::Custom { width, height } => (*width, *height),
        }
    }
}

/// 缩略图生成响应
#[derive(Debug, Serialize)]
pub struct GenerateThumbnailResponse {
    /// 文件ID
    pub file_id: i64,

    /// 原始文件名
    pub original_name: String,

    /// 缩略图文件ID
    pub thumbnail_file_id: Option<i64>,

    /// 缩略图文件名
    pub thumbnail_name: String,

    /// 缩略图内容（Base64编码）
    pub thumbnail_data: Vec<u8>,

    /// MIME类型
    pub content_type: String,

    /// 文件大小（字节）
    pub file_size: i64,

    /// 宽度
    pub width: u32,

    /// 高度
    pub height: u32,

    /// 缩略图生成时间
    pub generated_time: chrono::DateTime<chrono::Utc>,
}

/// 批量缩略图生成请求
#[derive(Debug, Deserialize)]
pub struct BatchGenerateThumbnailsRequest {
    /// 文件ID列表
    pub file_ids: Vec<i64>,

    /// 缩略图尺寸
    pub size: ThumbnailSize,

    /// 缩略图质量（1-100）
    pub quality: Option<u8>,

    /// 输出格式（jpg, png, webp）
    pub output_format: Option<String>,
}

/// 批量缩略图生成响应
#[derive(Debug, Serialize)]
pub struct BatchGenerateThumbnailsResponse {
    /// 成功生成缩略图的文件数
    pub success_count: usize,

    /// 失败的文件数
    pub failure_count: usize,

    /// 成功的缩略图列表
    pub success_thumbnails: Vec<GenerateThumbnailResponse>,

    /// 失败的详情列表
    pub failures: Vec<ThumbnailGenerationFailure>,
}

/// 缩略图生成失败详情
#[derive(Debug, Serialize)]
pub struct ThumbnailGenerationFailure {
    /// 文件ID
    pub file_id: i64,

    /// 错误消息
    pub error_message: String,

    /// 错误代码
    pub error_code: String,
}

/// 获取文件预览URL请求
#[derive(Debug, Deserialize)]
pub struct GetPreviewUrlRequest {
    /// 文件ID
    pub file_id: i64,

    /// 过期时间（秒，默认3600秒）
    pub expires_in: Option<i64>,
}

/// 获取文件预览URL响应
#[derive(Debug, Serialize)]
pub struct GetPreviewUrlResponse {
    /// 文件ID
    pub file_id: i64,

    /// 预览URL
    pub preview_url: String,

    /// 过期时间
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

/// 文件类型检测结果
#[derive(Debug, Serialize, Deserialize)]
pub struct FileTypeInfo {
    /// MIME类型
    pub mime_type: String,

    /// 文件类型分类
    pub category: FileCategory,

    /// 是否支持预览
    pub is_previewable: bool,

    /// 是否支持缩略图
    pub is_thumbnailable: bool,

    /// 文件类型描述
    pub description: String,
}

/// 文件类型分类
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum FileCategory {
    /// 图片
    Image = 1,
    /// 视频
    Video = 2,
    /// 音频
    Audio = 3,
    /// 文档
    Document = 4,
    /// 文本
    Text = 5,
    /// 压缩包
    Archive = 6,
    /// 其他
    Other = 99,
}

impl FileCategory {
    /// 获取分类名称
    pub fn get_name(&self) -> &'static str {
        match self {
            FileCategory::Image => "图片",
            FileCategory::Video => "视频",
            FileCategory::Audio => "音频",
            FileCategory::Document => "文档",
            FileCategory::Text => "文本",
            FileCategory::Archive => "压缩包",
            FileCategory::Other => "其他",
        }
    }
}

/// 文件内容解析请求
#[derive(Debug, Deserialize)]
pub struct ParseFileContentRequest {
    /// 文件ID
    pub file_id: i64,

    /// 解析参数
    pub options: Option<ParseOptions>,
}

/// 解析选项
#[derive(Debug, Deserialize, Serialize)]
pub struct ParseOptions {
    /// 文本编码格式
    pub encoding: Option<String>,

    /// 最大解析行数（文本文件）
    pub max_lines: Option<u32>,

    /// 最大解析大小（字节）
    pub max_size: Option<i64>,

    /// 是否提取元数据
    pub extract_metadata: Option<bool>,
}

/// 文件内容解析响应
#[derive(Debug, Serialize)]
pub struct ParseFileContentResponse {
    /// 文件ID
    pub file_id: i64,

    /// 文件名
    pub file_name: String,

    /// 文件类型
    pub file_type: FileCategory,

    /// 解析的内容
    pub content: Option<String>,

    /// 元数据
    pub metadata: Option<FileMetadata>,

    /// 解析时间
    pub parse_time: chrono::DateTime<chrono::Utc>,
}

/// 文件元数据
#[derive(Debug, Serialize, Deserialize)]
pub struct FileMetadata {
    /// 文件大小
    pub size: i64,

    /// 创建时间
    pub created_time: Option<chrono::DateTime<chrono::Utc>>,

    /// 修改时间
    pub modified_time: Option<chrono::DateTime<chrono::Utc>>,

    /// 宽度（图片或视频）
    pub width: Option<u32>,

    /// 高度（图片或视频）
    pub height: Option<u32>,

    /// 持续时间（视频或音频）
    pub duration: Option<f64>,

    /// 帧率（视频）
    pub fps: Option<f64>,

    /// 比特率（视频或音频）
    pub bitrate: Option<i64>,

    /// 采样率（音频）
    pub sample_rate: Option<i32>,

    /// 声道数（音频）
    pub channels: Option<i32>,

    /// 页数（PDF或文档）
    pub page_count: Option<u32>,

    /// 字符编码（文本）
    pub encoding: Option<String>,

    /// 行数（文本）
    pub line_count: Option<u32>,

    /// 自定义属性
    pub custom: Option<serde_json::Value>,
}
