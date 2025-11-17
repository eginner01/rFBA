//! 邮件发送DTO定义

use serde::{Deserialize, Serialize};
use validator::Validate;
use chrono::NaiveDateTime;
use std::collections::HashMap;

/// 邮件发送记录详情
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmailRecordDetail {
    pub id: i64,
    pub to_email: String,
    pub subject: String,
    pub content: String,
    pub is_html: i16,
    pub status: i16,
    pub error_msg: Option<String>,
    pub send_time: Option<NaiveDateTime>,
    pub created_time: NaiveDateTime,
}

impl From<crate::entity::email_record::Model> for EmailRecordDetail {
    fn from(model: crate::entity::email_record::Model) -> Self {
        Self {
            id: model.id,
            to_email: model.to_email,
            subject: model.subject,
            content: model.content,
            is_html: model.is_html,
            status: model.status,
            error_msg: model.error_msg,
            send_time: model.send_time,
            created_time: model.created_time,
        }
    }
}

/// 发送邮件请求参数
#[derive(Debug, Deserialize, Validate)]
pub struct SendEmailParam {
    /// 收件人邮箱
    #[validate(email(message = "收件人邮箱格式不正确"))]
    pub to: String,
    
    /// 邮件主题
    #[validate(length(min = 1, max = 255, message = "主题长度必须在1-255之间"))]
    pub subject: String,
    
    /// 邮件内容
    #[validate(length(min = 1, max = 100000, message = "内容长度必须在1-100000之间"))]
    pub content: String,
    
    /// 是否HTML格式
    #[serde(default)]
    pub is_html: bool,
}

/// 发送模板邮件请求参数
#[derive(Debug, Deserialize, Validate)]
pub struct SendTemplateEmailParam {
    /// 收件人邮箱
    #[validate(email(message = "收件人邮箱格式不正确"))]
    pub to: String,
    
    /// 模板名称
    #[validate(length(min = 1, max = 50, message = "模板名称长度必须在1-50之间"))]
    pub template: String,
    
    /// 模板数据
    pub data: HashMap<String, String>,
}

/// 测试SMTP配置请求参数
#[derive(Debug, Deserialize, Validate)]
pub struct TestSmtpParam {
    /// SMTP服务器
    #[validate(length(min = 1, max = 255, message = "SMTP服务器不能为空"))]
    pub host: String,
    
    /// SMTP端口
    #[validate(range(min = 1, max = 65535, message = "端口必须在1-65535之间"))]
    pub port: u16,
    
    /// 用户名
    #[validate(length(min = 1, max = 255, message = "用户名不能为空"))]
    pub username: String,
    
    /// 密码
    #[validate(length(min = 1, max = 255, message = "密码不能为空"))]
    pub password: String,
    
    /// 测试收件人
    #[validate(email(message = "测试收件人邮箱格式不正确"))]
    pub test_to: String,
}

/// 邮件记录查询参数
#[derive(Debug, Deserialize, Clone)]
pub struct EmailRecordQuery {
    /// 收件人邮箱（模糊查询）
    pub to_email: Option<String>,
    
    /// 发送状态
    pub status: Option<i16>,
}
