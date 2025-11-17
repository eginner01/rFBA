use axum::{
    extract::{Path},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use crate::common::response::api_response;
use crate::common::exception::{AppError, ErrorCode};
use crate::app::file_info::dto::{
    DownloadFileResponse,
    PreviewFileRequest, PreviewFileResponse,
    GenerateThumbnailRequest, GenerateThumbnailResponse,
};
use serde::Serialize;
use tokio::fs;
use std::io::Cursor;

#[derive(Serialize)]
pub struct FileInfoItem {
    id: i64,
    file_name: String,
    file_path: String,
    file_size: i64,
    file_type: String,
}

pub async fn get_file_infos() -> Result<impl IntoResponse, AppError> {
    let files = vec![FileInfoItem {
        id: 1,
        file_name: "document.pdf".to_string(),
        file_path: "/uploads/document.pdf".to_string(),
        file_size: 1024,
        file_type: "pdf".to_string(),
    }];
    Ok((StatusCode::OK, Json(api_response(files))))
}

pub async fn get_file_info() -> Result<impl IntoResponse, AppError> {
    Ok((StatusCode::OK, Json(api_response("查询成功".to_string()))))
}

pub async fn upload_file() -> Result<impl IntoResponse, AppError> {
    Ok((StatusCode::CREATED, Json(api_response("上传成功".to_string()))))
}

pub async fn delete_file_info() -> Result<impl IntoResponse, AppError> {
    Ok((StatusCode::NO_CONTENT, Json(api_response("删除成功".to_string()))))
}

/// 文件下载
/// GET /api/v1/file-info/{id}/download
pub async fn download_file(
    Path(file_id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    // 构建文件路径 - 基于 file_id 构建简单的文件名
    let file_path = format!("uploads/{}", file_id);

    // 异步读取文件
    let file_data = match fs::read(&file_path).await {
        Ok(data) => data,
        Err(_) => {
            // 文件不存在，返回错误而不是假数据
            return Err(AppError::with_message(
                ErrorCode::NotFound,
                format!("File with id {} not found", file_id)
            ));
        }
    };

    let file_size = file_data.len() as i64;

    // 根据文件扩展名判断 MIME 类型
    let content_type = if file_path.ends_with(".pdf") {
        "application/pdf".to_string()
    } else if file_path.ends_with(".jpg") || file_path.ends_with(".jpeg") {
        "image/jpeg".to_string()
    } else if file_path.ends_with(".png") {
        "image/png".to_string()
    } else if file_path.ends_with(".gif") {
        "image/gif".to_string()
    } else if file_path.ends_with(".txt") {
        "text/plain".to_string()
    } else {
        "application/octet-stream".to_string()
    };

    let response = DownloadFileResponse {
        file_id,
        file_name: format!("file_{}", file_id),
        file_data,
        content_type,
        file_size,
    };

    Ok((StatusCode::OK, Json(api_response(response))))
}

/// 文件预览
/// GET /api/v1/file-info/{id}/preview
pub async fn preview_file(
    Path(_file_id): Path<i64>,
    Json(request): Json<PreviewFileRequest>,
) -> Result<impl IntoResponse, AppError> {
    // 构建文件路径
    let file_path = format!("uploads/{}", request.file_id);

    // 异步读取文件
    let file_data = match fs::read(&file_path).await {
        Ok(data) => data,
        Err(_) => {
            return Err(AppError::with_message(
                ErrorCode::NotFound,
                format!("File with id {} not found", request.file_id)
            ));
        }
    };

    // 根据文件扩展名判断文件类型
    let file_name = format!("file_{}", request.file_id);
    let (content_type, preview_data, width, height, duration, page_count, current_page, encoding) =
        if file_path.ends_with(".pdf") {
            // PDF 文件 - 返回原数据
            ("application/pdf".to_string(), file_data.clone(), None, None, None, Some(1), Some(1), None)
        } else if file_path.ends_with(".jpg") || file_path.ends_with(".jpeg") || file_path.ends_with(".png") || file_path.ends_with(".gif") {
            // 图片文件 - 尝试解析尺寸
            let content_type = if file_path.ends_with(".jpg") || file_path.ends_with(".jpeg") {
                "image/jpeg".to_string()
            } else if file_path.ends_with(".png") {
                "image/png".to_string()
            } else {
                "image/gif".to_string()
            };

            // 尝试从图片数据获取尺寸
            if let Ok(image) = image::load_from_memory(&file_data) {
                let w = image.width();
                let h = image.height();
                (content_type, file_data.clone(), Some(w), Some(h), None, None, None, None)
            } else {
                (content_type, file_data.clone(), None, None, None, None, None, None)
            }
        } else if file_path.ends_with(".txt") {
            // 文本文件 - 限制预览行数
            let max_lines = request.options.as_ref().and_then(|o| o.max_lines).unwrap_or(100);
            let text = String::from_utf8_lossy(&file_data);
            let lines: Vec<&str> = text.lines().take(max_lines as usize).collect();
            let preview_text = lines.join("\n");
            ("text/plain".to_string(), preview_text.into_bytes(), None, None, None, None, None, Some("UTF-8".to_string()))
        } else {
            // 其他类型文件
            ("application/octet-stream".to_string(), file_data.clone(), None, None, None, None, None, None)
        };

    let response = PreviewFileResponse {
        file_id: request.file_id,
        file_name,
        preview_data,
        content_type,
        file_size: file_data.len() as i64,
        width,
        height,
        duration,
        page_count,
        current_page,
        encoding,
        preview_time: chrono::Utc::now(),
    };

    Ok((StatusCode::OK, Json(api_response(response))))
}

/// 生成缩略图
/// POST /api/v1/file-info/{id}/thumbnail
pub async fn generate_thumbnail(
    Path(_file_id): Path<i64>,
    Json(request): Json<GenerateThumbnailRequest>,
) -> Result<impl IntoResponse, AppError> {
    // 构建文件路径
    let file_path = format!("uploads/{}", request.file_id);

    // 读取原文件
    let file_data = match fs::read(&file_path).await {
        Ok(data) => data,
        Err(_) => {
            return Err(AppError::with_message(
                ErrorCode::NotFound,
                format!("File with id {} not found", request.file_id)
            ));
        }
    };

    // 获取缩略图尺寸和质量
    let (width, height) = request.size.get_dimensions();
    let quality = request.quality.unwrap_or(80);

    // 生成缩略图
    let thumbnail_data = if file_path.ends_with(".jpg") || file_path.ends_with(".jpeg") || file_path.ends_with(".png") || file_path.ends_with(".gif") {
        // 图片文件 - 生成缩略图
        if let Ok(image) = image::load_from_memory(&file_data) {
            let img = if request.keep_aspect_ratio.unwrap_or(true) {
                image.thumbnail(width, height)
            } else {
                image.resize_exact(width, height, image::imageops::FilterType::Triangle)
            };

            let mut buffer = Vec::new();
            img.write_to(&mut Cursor::new(&mut buffer), image::ImageOutputFormat::Jpeg(quality))
                .map_err(|e| AppError::with_message(ErrorCode::BusinessError, e.to_string()))?;
            buffer
        } else {
            // 如果图片解析失败，返回原数据的克隆
            file_data.clone()
        }
    } else {
        // 非图片文件 - 创建默认缩略图
        let mut img = image::RgbImage::new(width, height);

        // 根据文件类型选择颜色
        let color = if file_path.ends_with(".pdf") {
            [255, 0, 0] // 红色 - PDF
        } else if file_path.ends_with(".txt") {
            [255, 255, 0] // 黄色 - 文本
        } else if file_path.ends_with(".zip") || file_path.ends_with(".rar") {
            [128, 0, 128] // 紫色 - 压缩包
        } else {
            [128, 128, 128] // 灰色 - 其他
        };

        for y in 0..height {
            for x in 0..width {
                img.put_pixel(x, y, image::Rgb(color));
            }
        }

        let mut buffer = Vec::new();
        img.write_to(&mut Cursor::new(&mut buffer), image::ImageOutputFormat::Jpeg(80))
            .map_err(|e| AppError::with_message(ErrorCode::BusinessError, e.to_string()))?;
        buffer
    };

    let thumbnail_name = format!("file_{}_thumb.jpg", request.file_id);

    let response = GenerateThumbnailResponse {
        file_id: request.file_id,
        original_name: format!("file_{}", request.file_id),
        thumbnail_file_id: None,
        thumbnail_name,
        thumbnail_data: base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &thumbnail_data).into_bytes(),
        content_type: "image/jpeg".to_string(),
        file_size: thumbnail_data.len() as i64,
        width,
        height,
        generated_time: chrono::Utc::now(),
    };

    Ok((StatusCode::OK, Json(api_response(response))))
}
