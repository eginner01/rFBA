//! 邮件发送服务层

use sea_orm::*;
use lettre::{
    Message, SmtpTransport, Transport,
    transport::smtp::authentication::Credentials,
};
use tera::{Tera, Context};
use std::collections::HashMap;

use crate::entity::email_record;
use crate::dto::*;
use crate::error::EmailError;

/// SMTP配置
#[derive(Clone)]
pub struct SmtpConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub from: String,
}

impl Default for SmtpConfig {
    fn default() -> Self {
        Self {
            host: std::env::var("SMTP_HOST").unwrap_or_else(|_| "smtp.gmail.com".to_string()),
            port: std::env::var("SMTP_PORT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(587),
            username: std::env::var("SMTP_USERNAME").unwrap_or_default(),
            password: std::env::var("SMTP_PASSWORD").unwrap_or_default(),
            from: std::env::var("SMTP_FROM").unwrap_or_else(|_| "noreply@example.com".to_string()),
        }
    }
}

/// 邮件服务
pub struct EmailService;

impl EmailService {
    /// 发送邮件（核心功能）
    pub async fn send_email(
        db: &DatabaseConnection,
        smtp_config: &SmtpConfig,
        param: SendEmailParam,
    ) -> Result<EmailRecordDetail, EmailError> {
        // 1. 创建邮件记录（状态：待发送）
        let now = chrono::Utc::now().naive_utc();
        let record = email_record::ActiveModel {
            to_email: Set(param.to.clone()),
            subject: Set(param.subject.clone()),
            content: Set(param.content.clone()),
            is_html: Set(if param.is_html { 1 } else { 0 }),
            status: Set(0), // 待发送
            error_msg: Set(None),
            send_time: Set(None),
            created_time: Set(now),
            ..Default::default()
        };
        
        let record = record
            .insert(db)
            .await
            .map_err(|e| EmailError::DatabaseError(e.to_string()))?;
        
        let record_id = record.id;
        
        // 2. 异步发送邮件
        let smtp_config = smtp_config.clone();
        let to_email = param.to.clone();
        let subject = param.subject.clone();
        let content = param.content.clone();
        let is_html = param.is_html;
        let db_clone = db.clone();
        
        tokio::spawn(async move {
            let result = Self::send_smtp_email(
                &smtp_config,
                &to_email,
                &subject,
                &content,
                is_html,
            ).await;
            
            // 更新发送状态
            let _ = Self::update_send_status(&db_clone, record_id, result).await;
        });
        
        Ok(EmailRecordDetail::from(record))
    }
    
    /// SMTP发送邮件
    async fn send_smtp_email(
        smtp_config: &SmtpConfig,
        to: &str,
        subject: &str,
        content: &str,
        is_html: bool,
    ) -> Result<(), String> {
        // 构建邮件
        let email = if is_html {
            Message::builder()
                .from(smtp_config.from.parse().map_err(|e| format!("From地址错误: {}", e))?)
                .to(to.parse().map_err(|e| format!("To地址错误: {}", e))?)
                .subject(subject)
                .body(content.to_string())
                .map_err(|e| format!("邮件构建失败: {}", e))?
        } else {
            Message::builder()
                .from(smtp_config.from.parse().map_err(|e| format!("From地址错误: {}", e))?)
                .to(to.parse().map_err(|e| format!("To地址错误: {}", e))?)
                .subject(subject)
                .body(content.to_string())
                .map_err(|e| format!("邮件构建失败: {}", e))?
        };
        
        // 创建SMTP传输
        let creds = Credentials::new(
            smtp_config.username.clone(),
            smtp_config.password.clone(),
        );
        
        let mailer = SmtpTransport::relay(&smtp_config.host)
            .map_err(|e| format!("SMTP连接失败: {}", e))?
            .credentials(creds)
            .port(smtp_config.port)
            .build();
        
        // 发送邮件
        mailer
            .send(&email)
            .map_err(|e| format!("邮件发送失败: {}", e))?;
        
        Ok(())
    }
    
    /// 更新发送状态
    async fn update_send_status(
        db: &DatabaseConnection,
        record_id: i64,
        result: Result<(), String>,
    ) -> Result<(), EmailError> {
        let record = email_record::Entity::find_by_id(record_id)
            .one(db)
            .await
            .map_err(|e| EmailError::DatabaseError(e.to_string()))?
            .ok_or(EmailError::NotFound("记录不存在".to_string()))?;
        
        let now = chrono::Utc::now().naive_utc();
        let mut record: email_record::ActiveModel = record.into();
        
        match result {
            Ok(_) => {
                record.status = Set(1); // 发送成功
                record.send_time = Set(Some(now));
            }
            Err(err) => {
                record.status = Set(2); // 发送失败
                record.error_msg = Set(Some(err));
            }
        }
        
        record
            .update(db)
            .await
            .map_err(|e| EmailError::DatabaseError(e.to_string()))?;
        
        Ok(())
    }
    
    /// 发送模板邮件
    pub async fn send_template_email(
        db: &DatabaseConnection,
        smtp_config: &SmtpConfig,
        param: SendTemplateEmailParam,
    ) -> Result<EmailRecordDetail, EmailError> {
        // 渲染模板
        let content = Self::render_template(&param.template, &param.data)?;
        
        // 发送邮件
        let send_param = SendEmailParam {
            to: param.to,
            subject: format!("[{}] 通知", param.template),
            content,
            is_html: true,
        };
        
        Self::send_email(db, smtp_config, send_param).await
    }
    
    /// 渲染邮件模板
    fn render_template(
        template_name: &str,
        data: &HashMap<String, String>,
    ) -> Result<String, EmailError> {
        // 创建Tera实例
        let tera = Tera::new("plugins/email/templates/**/*.html")
            .map_err(|e| EmailError::TemplateError(format!("模板引擎初始化失败: {}", e)))?;
        
        // 构建上下文
        let mut context = Context::new();
        for (key, value) in data {
            context.insert(key, value);
        }
        
        // 渲染模板
        let template_file = format!("{}.html", template_name);
        let content = tera
            .render(&template_file, &context)
            .map_err(|e| EmailError::TemplateError(format!("模板渲染失败: {}", e)))?;
        
        Ok(content)
    }
    
    /// 测试SMTP配置
    pub async fn test_smtp(param: TestSmtpParam) -> Result<(), EmailError> {
        let smtp_config = SmtpConfig {
            host: param.host,
            port: param.port,
            username: param.username,
            password: param.password,
            from: "test@example.com".to_string(),
        };
        
        Self::send_smtp_email(
            &smtp_config,
            &param.test_to,
            "SMTP配置测试",
            "这是一封SMTP配置测试邮件，如果您收到此邮件，说明配置正确。",
            false,
        )
        .await
        .map_err(|e| EmailError::SmtpError(e))?;
        
        Ok(())
    }
    
    /// 查询发送记录
    pub async fn get_records(
        db: &DatabaseConnection,
        query: EmailRecordQuery,
        pagination: PaginationQuery,
    ) -> Result<PageData<EmailRecordDetail>, EmailError> {
        let mut select = email_record::Entity::find();
        
        // 构建查询条件
        if let Some(to_email) = &query.to_email {
            select = select.filter(email_record::Column::ToEmail.contains(to_email));
        }
        if let Some(status) = query.status {
            select = select.filter(email_record::Column::Status.eq(status));
        }
        
        // 查询总数
        let total = select
            .clone()
            .count(db)
            .await
            .map_err(|e| EmailError::DatabaseError(e.to_string()))?;
        
        // 分页查询
        let items = select
            .order_by_desc(email_record::Column::CreatedTime)
            .offset(pagination.offset())
            .limit(pagination.limit())
            .all(db)
            .await
            .map_err(|e| EmailError::DatabaseError(e.to_string()))?
            .into_iter()
            .map(EmailRecordDetail::from)
            .collect();
        
        Ok(PageData::new(items, total, pagination.page, pagination.size))
    }
    
    /// 根据ID获取记录
    pub async fn get_record_by_id(
        db: &DatabaseConnection,
        id: i64,
    ) -> Result<EmailRecordDetail, EmailError> {
        let record = email_record::Entity::find_by_id(id)
            .one(db)
            .await
            .map_err(|e| EmailError::DatabaseError(e.to_string()))?
            .ok_or(EmailError::NotFound("记录不存在".to_string()))?;
        
        Ok(EmailRecordDetail::from(record))
    }
}
