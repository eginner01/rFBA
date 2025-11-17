/// 文件信息 DTO 模块

pub mod create_file_info;
pub mod file_info_query;
pub mod file_info_response;
pub mod preview_thumbnail;

pub use create_file_info::*;
pub use file_info_query::*;
pub use file_info_response::*;

pub use preview_thumbnail::{
    PreviewFileRequest, PreviewFileResponse, PreviewOptions,
    GenerateThumbnailRequest, GenerateThumbnailResponse, ThumbnailSize,
    BatchGenerateThumbnailsRequest, BatchGenerateThumbnailsResponse,
    ThumbnailGenerationFailure,
    GetPreviewUrlRequest, GetPreviewUrlResponse,
    FileTypeInfo, FileCategory, ParseFileContentRequest, ParseFileContentResponse,
    ParseOptions, FileMetadata,
};

pub use file_info_query::FileInfoListItem;
