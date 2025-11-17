use tracing::{info, warn, error, debug};

/// 文件信息服务实现
/// 提供文件信息的增删改查、上传下载、统计、预览、缩略图等功能

use crate::app::file_info::dto::{
    CreateFileInfoRequest, CreateFileInfoResponse, UpdateFileInfoRequest,
    UpdateFileInfoResponse, FileInfoQuery, FileInfoListResponse, FileInfoListItem,
    FileInfoDetailResponse, DownloadFileRequest, DownloadFileResponse,
    FileStatisticsQuery, FileStatisticsResponse, StorageTypeStatistics,
    AccessPermissionStatistics,
    PreviewFileRequest, PreviewFileResponse, PreviewOptions,
    GenerateThumbnailRequest, GenerateThumbnailResponse, ThumbnailSize,
    BatchGenerateThumbnailsRequest, BatchGenerateThumbnailsResponse,
    ThumbnailGenerationFailure,
    GetPreviewUrlRequest, GetPreviewUrlResponse,
    FileTypeInfo, FileCategory, ParseFileContentRequest, ParseFileContentResponse,
    ParseOptions, FileMetadata,
};
use crate::common::exception::{AppError, ErrorCode};
use crate::database::DatabaseManager;
use crate::database::entity::file_info::{self, StorageType, AccessPermission};
use sea_orm::{DatabaseConnection, DbErr, EntityTrait, QueryFilter, ColumnTrait, Select, Order};
use std::collections::HashMap;
use sha2::{Sha256, Digest};
use base64::{Engine as _, engine::general_purpose};
use image::{ImageOutputFormat, DynamicImage, GenericImageView};
use chrono::Utc;

/// 文件信息服务
pub struct FileInfoService {
    db: DatabaseConnection,
}

impl FileInfoService {
    /// 创建新的文件信息服务
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// 创建文件信息
    pub async fn create_file_info(
        &self,
        request: &CreateFileInfoRequest,
    ) -> Result<CreateFileInfoResponse, AppError> {
        let active_model = file_info::ActiveModel {
            file_id: Default::default(),
            file_name: sea_orm::Set(request.file_name.clone()),
            original_name: sea_orm::Set(request.original_name.clone()),
            file_suffix: sea_orm::Set(request.file_suffix.clone()),
            file_size: sea_orm::Set(request.file_size),
            content_type: sea_orm::Set(request.content_type.clone()),
            file_path: sea_orm::Set(request.file_path.clone()),
            storage_type: sea_orm::Set(request.storage_type),
            file_hash: sea_orm::Set(request.file_hash.clone()),
            uploader: sea_orm::Set(request.uploader.clone()),
            access_permission: sea_orm::Set(request.access_permission),
            download_count: Default::default(),
            is_deleted: Default::default(),
            upload_time: Default::default(),
            updated_time: Default::default(),
            remark: sea_orm::Set(request.remark.clone()),
        };

        let saved_file = active_model.insert(&self.db).await.map_err(|e| {
            error!("Failed to create file info: {:?}", e);
            AppError::with_message(ErrorCode::DatabaseError, "Failed to create file info")
        })?;

        Ok(CreateFileInfoResponse {
            file_id: saved_file.file_id,
            file_name: saved_file.file_name,
            original_name: saved_file.original_name,
            upload_time: saved_file.upload_time,
        })
    }

    /// 更新文件信息
    pub async fn update_file_info(
        &self,
        request: &UpdateFileInfoRequest,
    ) -> Result<UpdateFileInfoResponse, AppError> {
        let existing_file = file_info::Entity::find_by_id(request.file_id)
            .one(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to find file info: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, "Failed to find file info")
            })?
            .ok_or_else(|| AppError::with_message(ErrorCode::NotFound, "File not found"))?;

        let mut active_model = existing_file.into_active_model();
        active_model.file_name = sea_orm::Set(request.file_name.clone());
        active_model.access_permission = sea_orm::Set(request.access_permission);
        active_model.remark = sea_orm::Set(request.remark.clone());

        let updated_file = active_model.update(&self.db).await.map_err(|e| {
            error!("Failed to update file info: {:?}", e);
            AppError::with_message(ErrorCode::DatabaseError, "Failed to update file info")
        })?;

        Ok(UpdateFileInfoResponse {
            file_id: updated_file.file_id,
            file_name: updated_file.file_name,
            updated_time: updated_file.updated_time,
        })
    }

    /// 删除文件信息（软删除）
    pub async fn delete_file_infos(&self, file_ids: &[i64]) -> Result<(), AppError> {
        if file_ids.is_empty() {
            return Ok(());
        }

        let files = file_info::Entity::find()
            .filter(file_info::Column::FileId.is_in(file_ids.to_vec()))
            .all(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to find file infos for deletion: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, "Failed to find file infos")
            })?;

        if files.is_empty() {
            return Err(AppError::with_message(ErrorCode::NotFound, "Files not found"));
        }

        for file in files {
            let mut active_model = file.into_active_model();
            active_model.is_deleted = sea_orm::Set(1);
            active_model.updated_time = sea_orm::Set(chrono::Utc::now().naive_utc());

            active_model.update(&self.db).await.map_err(|e| {
                error!("Failed to delete file info: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, "Failed to delete file info")
            })?;
        }

        Ok(())
    }

    /// 获取文件列表（分页）
    pub async fn get_file_info_list(
        &self,
        query: &FileInfoQuery,
    ) -> Result<FileInfoListResponse, AppError> {
        let mut select = file_info::Entity::find_not_deleted();

        if let Some(file_name) = &query.file_name {
            select = select.filter(file_info::Column::FileName.like(format!(
                "%{}%",
                file_name
            )));
        }

        if let Some(original_name) = &query.original_name {
            select = select.filter(file_info::Column::OriginalName.like(format!(
                "%{}%",
                original_name
            )));
        }

        if let Some(file_suffix) = &query.file_suffix {
            select = select.filter(file_info::Column::FileSuffix.eq(file_suffix));
        }

        if let Some(storage_type) = query.storage_type {
            select = select.filter(file_info::Column::StorageType.eq(storage_type));
        }

        if let Some(access_permission) = query.access_permission {
            select = select.filter(file_info::Column::AccessPermission.eq(access_permission));
        }

        if let Some(uploader) = &query.uploader {
            select = select.filter(file_info::Column::Uploader.like(format!("%{}%", uploader)));
        }

        select = select.order_by(file_info::Column::UploadTime, Order::Desc);

        let page_size = query.page_size.unwrap_or(20);
        let page_num = query.page_num.unwrap_or(1);
        let offset = (page_num - 1) * page_size;

        let files = select
            .offset(offset as u64)
            .limit(page_size as u64)
            .all(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to query file infos: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, "Failed to query file infos")
            })?;

        let total = file_info::Entity::find()
            .filter(file_info::Column::IsDeleted.eq(0))
            .count(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to count file infos: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, "Failed to count file infos")
            })?;

        let list = files
            .into_iter()
            .map(|f| {
                let storage_type_name = match StorageType::from(f.storage_type) {
                    StorageType::Local => "本地存储",
                    StorageType::Oss => "阿里云OSS",
                    StorageType::S3 => "亚马逊S3",
                };

                let access_permission_name = match AccessPermission::from(f.access_permission) {
                    AccessPermission::Private => "私有",
                    AccessPermission::Public => "公开",
                    AccessPermission::Organization => "组织内",
                };

                FileInfoListItem {
                    file_id: f.file_id,
                    file_name: f.file_name,
                    original_name: f.original_name,
                    file_suffix: f.file_suffix.clone(),
                    file_size: f.file_size,
                    size_description: f.get_size_description(),
                    content_type: f.content_type.clone(),
                    file_path: f.file_path.clone(),
                    storage_type: f.storage_type,
                    storage_type_name: storage_type_name.to_string(),
                    file_hash: f.file_hash,
                    uploader: f.uploader,
                    access_permission: f.access_permission,
                    access_permission_name: access_permission_name.to_string(),
                    download_count: f.download_count,
                    upload_time: f.upload_time,
                    updated_time: f.updated_time,
                    remark: f.remark,
                }
            })
            .collect();

        Ok(FileInfoListResponse {
            list,
            total: total as usize,
            page_num,
            page_size,
        })
    }

    /// 获取文件详情
    pub async fn get_file_info_detail(
        &self,
        file_id: i64,
    ) -> Result<FileInfoDetailResponse, AppError> {
        let f = file_info::Entity::find_by_id(file_id)
            .one(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to find file info: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, "Failed to find file info")
            })?
            .ok_or_else(|| AppError::with_message(ErrorCode::NotFound, "File not found"))?;

        if f.is_deleted == 1 {
            return Err(AppError::with_message(ErrorCode::NotFound, "File has been deleted"));
        }

        let storage_type_name = match StorageType::from(f.storage_type) {
            StorageType::Local => "本地存储",
            StorageType::Oss => "阿里云OSS",
            StorageType::S3 => "亚马逊S3",
        };

        let access_permission_name = match AccessPermission::from(f.access_permission) {
            AccessPermission::Private => "私有",
            AccessPermission::Public => "公开",
            AccessPermission::Organization => "组织内",
        };

        Ok(FileInfoDetailResponse {
            file_id: f.file_id,
            file_name: f.file_name,
            original_name: f.original_name,
            file_suffix: f.file_suffix,
            file_size: f.file_size,
            size_description: f.get_size_description(),
            content_type: f.content_type,
            file_path: f.file_path,
            storage_type: f.storage_type,
            storage_type_name: storage_type_name.to_string(),
            file_hash: f.file_hash,
            uploader: f.uploader,
            access_permission: f.access_permission,
            access_permission_name: access_permission_name.to_string(),
            download_count: f.download_count,
            upload_time: f.upload_time,
            updated_time: f.updated_time,
            remark: f.remark,
        })
    }

    /// 增加下载次数
    pub async fn increment_download_count(&self, file_id: i64) -> Result<(), AppError> {
        let file = file_info::Entity::find_by_id(file_id)
            .one(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to find file info: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, "Failed to find file info")
            })?
            .ok_or_else(|| AppError::with_message(ErrorCode::NotFound, "File not found"))?;

        let mut active_model = file.into_active_model();
        active_model.download_count = sea_orm::Set(file.download_count + 1);

        active_model.update(&self.db).await.map_err(|e| {
            error!("Failed to increment download count: {:?}", e);
            AppError::with_message(ErrorCode::DatabaseError, "Failed to increment download count")
        })?;

        Ok(())
    }

    /// 计算文件SHA256哈希
    pub fn calculate_file_hash(file_data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(file_data);
        format!("{:x}", hasher.finalize())
    }

    /// 获取文件统计
    pub async fn get_file_statistics(
        &self,
        query: &FileStatisticsQuery,
    ) -> Result<FileStatisticsResponse, AppError> {
        let mut select = file_info::Entity::find_not_deleted();

        if let Some(uploader) = &query.uploader {
            select = select.filter(file_info::Column::Uploader.eq(uploader));
        }

        let files = select
            .order_by(file_info::Column::UploadTime, Order::Desc)
            .all(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to query file infos: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, "Failed to query file infos")
            })?;

        let mut storage_stats_map: HashMap<i32, (usize, i64)> = HashMap::new();
        let mut access_stats_map: HashMap<i32, usize> = HashMap::new();
        let mut total_size = 0i64;

        for file in &files {
            // 存储类型统计
            let entry = storage_stats_map
                .entry(file.storage_type)
                .or_insert((0, 0));
            entry.0 += 1;
            entry.1 += file.file_size;

            // 访问权限统计
            *access_stats_map.entry(file.access_permission).or_insert(0) += 1;

            total_size += file.file_size;
        }

        let storage_type_stats: Vec<StorageTypeStatistics> = storage_stats_map
            .into_iter()
            .map(|(storage_type, (count, size))| {
                let storage_type_name = match StorageType::from(storage_type) {
                    StorageType::Local => "本地存储",
                    StorageType::Oss => "阿里云OSS",
                    StorageType::S3 => "亚马逊S3",
                };

                StorageTypeStatistics {
                    storage_type,
                    storage_type_name: storage_type_name.to_string(),
                    file_count: count,
                    total_size: size,
                    total_size_description: format_file_size(size),
                }
            })
            .collect();

        let access_permission_stats: Vec<AccessPermissionStatistics> = access_stats_map
            .into_iter()
            .map(|(access_permission, count)| {
                let access_permission_name = match AccessPermission::from(access_permission) {
                    AccessPermission::Private => "私有",
                    AccessPermission::Public => "公开",
                    AccessPermission::Organization => "组织内",
                };

                AccessPermissionStatistics {
                    access_permission,
                    access_permission_name: access_permission_name.to_string(),
                    file_count: count,
                }
            })
            .collect();

        // 最受欢迎文件（按下载次数）
        let mut top_downloaded = files
            .iter()
            .cloned()
            .take(10)
            .map(|f| {
                let storage_type_name = match StorageType::from(f.storage_type) {
                    StorageType::Local => "本地存储",
                    StorageType::Oss => "阿里云OSS",
                    StorageType::S3 => "亚马逊S3",
                };

                let access_permission_name = match AccessPermission::from(f.access_permission) {
                    AccessPermission::Private => "私有",
                    AccessPermission::Public => "公开",
                    AccessPermission::Organization => "组织内",
                };

                FileInfoListItem {
                    file_id: f.file_id,
                    file_name: f.file_name,
                    original_name: f.original_name,
                    file_suffix: f.file_suffix,
                    file_size: f.file_size,
                    size_description: f.get_size_description(),
                    content_type: f.content_type,
                    file_path: f.file_path,
                    storage_type: f.storage_type,
                    storage_type_name: storage_type_name.to_string(),
                    file_hash: f.file_hash,
                    uploader: f.uploader,
                    access_permission: f.access_permission,
                    access_permission_name: access_permission_name.to_string(),
                    download_count: f.download_count,
                    upload_time: f.upload_time,
                    updated_time: f.updated_time,
                    remark: f.remark,
                }
            })
            .collect();

        top_downloaded.sort_by(|a, b| b.download_count.cmp(&a.download_count));

        // 最近上传文件
        let recent_uploads = files
            .iter()
            .cloned()
            .take(10)
            .map(|f| {
                let storage_type_name = match StorageType::from(f.storage_type) {
                    StorageType::Local => "本地存储",
                    StorageType::Oss => "阿里云OSS",
                    StorageType::S3 => "亚马逊S3",
                };

                let access_permission_name = match AccessPermission::from(f.access_permission) {
                    AccessPermission::Private => "私有",
                    AccessPermission::Public => "公开",
                    AccessPermission::Organization => "组织内",
                };

                FileInfoListItem {
                    file_id: f.file_id,
                    file_name: f.file_name,
                    original_name: f.original_name,
                    file_suffix: f.file_suffix,
                    file_size: f.file_size,
                    size_description: f.get_size_description(),
                    content_type: f.content_type,
                    file_path: f.file_path,
                    storage_type: f.storage_type,
                    storage_type_name: storage_type_name.to_string(),
                    file_hash: f.file_hash,
                    uploader: f.uploader,
                    access_permission: f.access_permission,
                    access_permission_name: access_permission_name.to_string(),
                    download_count: f.download_count,
                    upload_time: f.upload_time,
                    updated_time: f.updated_time,
                    remark: f.remark,
                }
            })
            .collect();

        Ok(FileStatisticsResponse {
            total_files: files.len(),
            total_size,
            total_size_description: format_file_size(total_size),
            storage_type_stats,
            access_permission_stats,
            top_downloaded,
            recent_uploads,
        })
    }

    /// 预览文件
    pub async fn preview_file(
        &self,
        request: &PreviewFileRequest,
    ) -> Result<PreviewFileResponse, AppError> {
        info!("Previewing file: {}", request.file_id);

        // 1. 查询文件信息
        let file = file_info::Entity::find_by_id(request.file_id)
            .one(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to find file: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, "Failed to find file")
            })?
            .ok_or_else(|| AppError::with_message(ErrorCode::NotFound, "File not found"))?;

        // 2. 获取文件类型信息
        let file_type = self.detect_file_type(&file.content_type, &file.file_suffix)?;

        // 3. 检查文件是否支持预览
        if !file_type.is_previewable {
            return Err(AppError::with_message(
                ErrorCode::BadRequest,
                "File type not supported for preview"
            ));
        }

        // 4. 读取文件数据（模拟，实际需要从存储系统读取）
        let file_data = Vec::new(); // TODO: 从存储系统读取

        // 5. 根据文件类型处理预览
        let (preview_data, width, height, duration, page_count, current_page, encoding) = match file_type.category {
            FileCategory::Image => {
                // 图片预览
                if let Ok(image) = image::load_from_memory(&file_data) {
                    let (w, h) = image.dimensions();
                    let preview_data = self.resize_image(&image, request.options.as_ref())?;
                    (preview_data, Some(w), Some(h), None, None, None, None)
                } else {
                    (file_data, None, None, None, None, None, None)
                }
            }
            FileCategory::Video => {
                // 视频预览（返回第一帧）
                (file_data, None, None, Some(0.0), None, None, None)
            }
            FileCategory::Audio => {
                // 音频预览
                (file_data, None, None, Some(0.0), None, None, None)
            }
            FileCategory::Document => {
                // 文档预览（PDF等）
                (file_data, None, None, None, Some(1), Some(1), None)
            }
            FileCategory::Text => {
                // 文本预览
                let content = String::from_utf8_lossy(&file_data).to_string();
                let max_lines = request.options.as_ref().and_then(|o| o.max_lines).unwrap_or(100);
                let lines: Vec<&str> = content.lines().take(max_lines as usize).collect();
                (lines.join("\n").into_bytes(), None, None, None, None, None, Some("UTF-8".to_string()))
            }
            _ => (file_data, None, None, None, None, None, None),
        };

        // 6. 编码为Base64
        let preview_data_base64 = general_purpose::STANDARD.encode(&preview_data);

        info!("File preview generated successfully");

        Ok(PreviewFileResponse {
            file_id: file.file_id,
            file_name: file.file_name,
            preview_data: preview_data_base64.into_bytes(),
            content_type: if file_type.category == FileCategory::Image {
                "image/jpeg".to_string()
            } else {
                file.content_type
            },
            file_size: file_data.len() as i64,
            width,
            height,
            duration,
            page_count,
            current_page,
            encoding,
            preview_time: Utc::now(),
        })
    }

    /// 生成缩略图
    pub async fn generate_thumbnail(
        &self,
        request: &GenerateThumbnailRequest,
    ) -> Result<GenerateThumbnailResponse, AppError> {
        info!("Generating thumbnail for file: {}", request.file_id);

        // 1. 查询文件信息
        let file = file_info::Entity::find_by_id(request.file_id)
            .one(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to find file: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, "Failed to find file")
            })?
            .ok_or_else(|| AppError::with_message(ErrorCode::NotFound, "File not found"))?;

        // 2. 检查文件是否支持缩略图
        let file_type = self.detect_file_type(&file.content_type, &file.file_suffix)?;
        if !file_type.is_thumbnailable {
            return Err(AppError::with_message(
                ErrorCode::BadRequest,
                "File type not supported for thumbnail generation"
            ));
        }

        // 3. 读取文件数据（模拟）
        let file_data = Vec::new(); // TODO: 从存储系统读取

        // 4. 生成缩略图
        let (width, height) = request.size.get_dimensions();
        let quality = request.quality.unwrap_or(80);

        let thumbnail_data = if file_type.category == FileCategory::Image {
            // 图片缩略图
            if let Ok(image) = image::load_from_memory(&file_data) {
                self.create_thumbnail_from_image(&image, width, height, request.keep_aspect_ratio.unwrap_or(true), quality)?
            } else {
                file_data
            }
        } else {
            // 其他类型使用默认缩略图
            self.create_default_thumbnail(file_type.category, width, height)?
        };

        let thumbnail_name = format!("{}_thumb.{}", file.file_name, request.output_format.as_deref().unwrap_or("jpg"));

        info!("Thumbnail generated successfully");

        Ok(GenerateThumbnailResponse {
            file_id: file.file_id,
            original_name: file.original_name,
            thumbnail_file_id: None,
            thumbnail_name,
            thumbnail_data: general_purpose::STANDARD.encode(&thumbnail_data),
            content_type: format!("image/{}", request.output_format.as_deref().unwrap_or("jpeg")),
            file_size: thumbnail_data.len() as i64,
            width,
            height,
            generated_time: Utc::now(),
        })
    }

    /// 检测文件类型
    fn detect_file_type(
        &self,
        content_type: &str,
        file_suffix: &str,
    ) -> Result<FileTypeInfo, AppError> {
        let suffix = file_suffix.to_lowercase();
        let mime_type = content_type.to_lowercase();

        // 判断文件类型
        let (category, is_previewable, is_thumbnailable) = if mime_type.starts_with("image/") || ["jpg", "jpeg", "png", "gif", "bmp", "webp", "svg"].contains(&suffix.as_str()) {
            (FileCategory::Image, true, true)
        } else if mime_type.starts_with("video/") || ["mp4", "avi", "mkv", "mov", "wmv", "flv"].contains(&suffix.as_str()) {
            (FileCategory::Video, true, true)
        } else if mime_type.starts_with("audio/") || ["mp3", "wav", "flac", "aac", "ogg"].contains(&suffix.as_str()) {
            (FileCategory::Audio, true, false)
        } else if ["pdf", "doc", "docx", "xls", "xlsx", "ppt", "pptx"].contains(&suffix.as_str()) {
            (FileCategory::Document, true, false)
        } else if mime_type.starts_with("text/") || ["txt", "md", "json", "xml", "csv"].contains(&suffix.as_str()) {
            (FileCategory::Text, true, false)
        } else if ["zip", "rar", "7z", "tar", "gz"].contains(&suffix.as_str()) {
            (FileCategory::Archive, false, false)
        } else {
            (FileCategory::Other, false, false)
        };

        Ok(FileTypeInfo {
            mime_type: content_type.to_string(),
            category,
            is_previewable,
            is_thumbnailable,
            description: category.get_name().to_string(),
        })
    }

    /// 调整图片大小
    fn resize_image(
        &self,
        image: &DynamicImage,
        options: Option<&PreviewOptions>,
    ) -> Result<Vec<u8>, AppError> {
        let mut img = image.clone();

        if let Some(opts) = options {
            let target_width = opts.width.unwrap_or(img.width());
            let target_height = opts.height.unwrap_or(img.height());
            let keep_aspect = opts.keep_aspect_ratio.unwrap_or(true);

            if keep_aspect {
                img = img.thumbnail(target_width, target_height);
            } else {
                img = img.resize_exact(target_width, target_height, image::imageops::FilterType::Triangle);
            }

            let quality = opts.quality.unwrap_or(90);
            let mut buffer = Vec::new();
            img.write_to(&mut buffer, ImageOutputFormat::Jpeg(quality))
                .map_err(|e| AppError::with_message(ErrorCode::BusinessError, e.to_string()))?;
            Ok(buffer)
        } else {
            Ok(image::codecs::jpeg::JpegEncoder::new_with_quality(&mut Vec::new(), 90)
                .encode_image(img))
        }
    }

    /// 从图片创建缩略图
    fn create_thumbnail_from_image(
        &self,
        image: &DynamicImage,
        width: u32,
        height: u32,
        keep_aspect: bool,
        quality: u8,
    ) -> Result<Vec<u8>, AppError> {
        let img = if keep_aspect {
            image.thumbnail(width, height)
        } else {
            image.resize_exact(width, height, image::imageops::FilterType::Triangle)
        };

        let mut buffer = Vec::new();
        img.write_to(&mut buffer, ImageOutputFormat::Jpeg(quality))
            .map_err(|e| AppError::with_message(ErrorCode::BusinessError, e.to_string()))?;

        Ok(buffer)
    }

    /// 创建默认缩略图
    fn create_default_thumbnail(
        &self,
        category: FileCategory,
        width: u32,
        height: u32,
    ) -> Result<Vec<u8>, AppError> {
        // 创建一个简单的默认缩略图
        let mut img = image::DynamicImage::new_rgb8(width, height);
        let color = match category {
            FileCategory::Video => (255, 0, 0), // 红色
            FileCategory::Audio => (0, 255, 0), // 绿色
            FileCategory::Document => (0, 0, 255), // 蓝色
            FileCategory::Text => (255, 255, 0), // 黄色
            FileCategory::Archive => (128, 0, 128), // 紫色
            _ => (128, 128, 128), // 灰色
        };

        for y in 0..height {
            for x in 0..width {
                img.put_pixel(x, y, image::Rgb(color));
            }
        }

        let mut buffer = Vec::new();
        img.write_to(&mut buffer, ImageOutputFormat::Jpeg(80))
            .map_err(|e| AppError::with_message(ErrorCode::BusinessError, e.to_string()))?;

        Ok(buffer)
    }
}

/// 格式化文件大小
fn format_file_size(size: i64) -> String {
    if size < 1024 {
        format!("{} B", size)
    } else if size < 1024 * 1024 {
        format!("{:.2} KB", size as f64 / 1024.0)
    } else if size < 1024 * 1024 * 1024 {
        format!("{:.2} MB", size as f64 / (1024.0 * 1024.0))
    } else {
        format!(
            "{:.2} GB",
            size as f64 / (1024.0 * 1024.0 * 1024.0)
        )
    }
}
