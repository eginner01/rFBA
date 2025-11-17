//! 邮件发送API处理器

use axum::{
    extract::{Path, Query, State},
    response::Json,
};
use sea_orm::DatabaseConnection;
use validator::Validate;

use crate::dto::*;
use crate::error::EmailError;
use crate::service::{EmailService, SmtpConfig};

/// 应用状态
#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub smtp_config: SmtpConfig,
}

/// 发送邮件
/// POST /api/v1/email/send
pub async fn send_email(
    State(state): State<AppState>,
    Json(param): Json<SendEmailParam>,
) -> Result<Json<ApiResponse<EmailRecordDetail>>, EmailError> {
    // 验证参数
    param.validate()?;
    
    let data = EmailService::send_email(&state.db, &state.smtp_config, param).await?;
    Ok(Json(ApiResponse::success_with_msg("邮件发送请求已提交", data)))
}

/// 发送模板邮件
/// POST /api/v1/email/send-template
pub async fn send_template_email(
    State(state): State<AppState>,
    Json(param): Json<SendTemplateEmailParam>,
) -> Result<Json<ApiResponse<EmailRecordDetail>>, EmailError> {
    // 验证参数
    param.validate()?;
    
    let data = EmailService::send_template_email(&state.db, &state.smtp_config, param).await?;
    Ok(Json(ApiResponse::success_with_msg("模板邮件发送请求已提交", data)))
}

/// 测试SMTP配置
/// POST /api/v1/email/test-smtp
pub async fn test_smtp(
    Json(param): Json<TestSmtpParam>,
) -> Result<Json<ApiResponse<()>>, EmailError> {
    // 验证参数
    param.validate()?;
    
    EmailService::test_smtp(param).await?;
    Ok(Json(ApiResponse::success_msg("SMTP配置测试成功")))
}

/// 查询发送记录（分页）
/// GET /api/v1/email/records
pub async fn get_email_records(
    State(state): State<AppState>,
    Query(query): Query<EmailRecordQuery>,
    Query(pagination): Query<PaginationQuery>,
) -> Result<Json<ApiResponse<PageData<EmailRecordDetail>>>, EmailError> {
    let page_data = EmailService::get_records(&state.db, query, pagination).await?;
    Ok(Json(ApiResponse::success(page_data)))
}

/// 获取发送记录详情
/// GET /api/v1/email/records/{id}
pub async fn get_email_record(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<EmailRecordDetail>>, EmailError> {
    let data = EmailService::get_record_by_id(&state.db, id).await?;
    Ok(Json(ApiResponse::success(data)))
}

/// 创建邮件路由
pub fn email_routes() -> axum::Router<AppState> {
    use axum::routing::{get, post};
    
    axum::Router::new()
        .route("/send", post(send_email))
        .route("/send-template", post(send_template_email))
        .route("/test-smtp", post(test_smtp))
        .route("/records", get(get_email_records))
        .route("/records/:id", get(get_email_record))
}
