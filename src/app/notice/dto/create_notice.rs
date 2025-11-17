/// 通知公告创建和更新 DTO

use serde::{Deserialize, Serialize};
use validator::Validate;

/// 创建通知公告请求
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateNoticeRequest {
    /// 公告标题
    #[validate(length(min = 1, max = 100, message = "公告标题长度必须在1-100个字符之间"))]
    pub notice_title: String,

    /// 公告类型（1:通知 2:公告）
    #[validate(range(min = 1, max = 2, message = "公告类型必须是1或2"))]
    pub notice_type: i32,

    /// 公告内容
    #[validate(length(min = 1, message = "公告内容不能为空"))]
    pub notice_content: String,

    /// 公告状态（0:正常 1:关闭）
    #[validate(range(min = 0, max = 1, message = "公告状态必须是0或1"))]
    pub status: i32,

    /// 备注
    pub remark: Option<String>,
}

/// 更新通知公告请求
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateNoticeRequest {
    /// 公告ID
    #[validate(range(min = 1, message = "公告ID必须大于0"))]
    pub notice_id: i64,

    /// 公告标题
    #[validate(length(min = 1, max = 100, message = "公告标题长度必须在1-100个字符之间"))]
    pub notice_title: String,

    /// 公告类型（1:通知 2:公告）
    #[validate(range(min = 1, max = 2, message = "公告类型必须是1或2"))]
    pub notice_type: i32,

    /// 公告内容
    #[validate(length(min = 1, message = "公告内容不能为空"))]
    pub notice_content: String,

    /// 公告状态（0:正常 1:关闭）
    #[validate(range(min = 0, max = 1, message = "公告状态必须是0或1"))]
    pub status: i32,

    /// 备注
    pub remark: Option<String>,
}

/// 删除通知公告请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteNoticeRequest {
    /// 公告ID列表
    pub notice_ids: Vec<i64>,
}

/// 创建通知公告响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateNoticeResponse {
    /// 公告ID
    pub notice_id: i64,
    /// 公告标题
    pub notice_title: String,
    /// 公告类型
    pub notice_type: i32,
    /// 创建时间
    pub created_time: chrono::DateTime<chrono::Utc>,
}

/// 更新通知公告响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateNoticeResponse {
    /// 公告ID
    pub notice_id: i64,
    /// 公告标题
    pub notice_title: String,
    /// 更新时间
    pub updated_time: chrono::DateTime<chrono::Utc>,
}
