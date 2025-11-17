//! 通知公告DTO定义

use serde::{Deserialize, Serialize};
use validator::Validate;

/// 通知公告详情响应 - 匹配Python版本
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NoticeDetail {
    pub id: i64,
    pub title: String,
    #[serde(rename = "type")]
    pub type_: i32,
    pub status: i32,
    pub content: String,
}

impl From<crate::entity::notice::Model> for NoticeDetail {
    fn from(model: crate::entity::notice::Model) -> Self {
        Self {
            id: model.id,
            title: model.title,
            type_: model.type_,
            status: model.status,
            content: model.content,
        }
    }
}

/// 创建通知公告请求参数 - 匹配Python版本
#[derive(Debug, Deserialize, Validate)]
pub struct CreateNoticeParam {
    /// 公告标题
    #[validate(length(min = 1, max = 64, message = "标题长度必须在1-64之间"))]
    pub title: String,
    
    /// 类型（0：通知、1：公告）
    #[serde(rename = "type", default = "default_type")]
    #[validate(range(min = 0, max = 1, message = "类型必须是0或1"))]
    pub type_: i32,
    
    /// 状态（0：隐藏、1：显示）
    #[serde(default)]
    #[validate(range(min = 0, max = 1, message = "状态必须是0或1"))]
    pub status: i32,
    
    /// 公告内容
    #[validate(length(min = 1, max = 50000, message = "内容长度必须在1-50000之间"))]
    pub content: String,
}

/// 更新通知公告请求参数 - 匹配Python版本
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateNoticeParam {
    /// 公告标题
    #[validate(length(min = 1, max = 64, message = "标题长度必须在1-64之间"))]
    pub title: String,
    
    /// 类型（0：通知、1：公告）
    #[serde(rename = "type")]
    #[validate(range(min = 0, max = 1, message = "类型必须是0或1"))]
    pub type_: i32,
    
    /// 状态（0：隐藏、1：显示）
    #[validate(range(min = 0, max = 1, message = "状态必须是0或1"))]
    pub status: i32,
    
    /// 公告内容
    #[validate(length(min = 1, max = 50000, message = "内容长度必须在1-50000之间"))]
    pub content: String,
}

/// 通知公告查询参数 - 匹配Python版本
#[derive(Debug, Deserialize, Clone)]
pub struct NoticeQuery {
    /// 标题（模糊查询）
    pub title: Option<String>,
    
    /// 类型
    #[serde(rename = "type")]
    pub type_: Option<i32>,
    
    /// 状态
    pub status: Option<i32>,
}

fn default_type() -> i32 {
    0 // 默认为通知
}
