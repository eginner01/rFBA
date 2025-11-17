use axum::{routing::{get, post, delete}, Router};
use crate::app::file_info::api::{get_file_infos, get_file_info, upload_file, delete_file_info, download_file, preview_file, generate_thumbnail};

pub fn file_info_routes() -> Router {
    Router::new()
        .route("/file-infos", get(get_file_infos))
        .route("/file-infos/upload", post(upload_file))
        .route("/file-infos/{id}", get(get_file_info))
        .route("/file-infos/{id}", delete(delete_file_info))
        .route("/file-infos/{id}/download", get(download_file))
        .route("/file-infos/{id}/preview", get(preview_file))
        .route("/file-infos/{id}/thumbnail", post(generate_thumbnail))
}
