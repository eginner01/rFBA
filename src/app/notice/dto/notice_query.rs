/// 通知公告查询 DTO

use serde::{Deserialize, Serialize};
use validator::Validate;

/// 分页查询参数
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct NoticeQuery {
    /// 页码
    #[validate(range(min = 1, message = "页码必须大于0"))]
    pub page_num: Option<usize>,

    /// 每页数量
    #[validate(range(min = 1, max = 100, message = "每页数量必须在1-100之间"))]
    pub page_size: Option<usize>,

    /// 公告标题
    pub notice_title: Option<String>,

    /// 公告类型
    pub notice_type: Option<i32>,

    /// 公告状态
    pub status: Option<i32>,

    /// 创建者
    pub create_by: Option<String>,
}

/// 分页响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoticeListResponse {
    /// 公告列表
    pub list: Vec<NoticeListItem>,
    /// 总数
    pub total: usize,
    /// 页码
    pub page_num: usize,
    /// 每页数量
    pub page_size: usize,
}

/// 公告列表项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoticeListItem {
    /// 公告ID
    pub notice_id: i64,
    /// 公告标题
    pub notice_title: String,
    /// 公告类型
    pub notice_type: i32,
    /// 公告类型名称
    pub notice_type_name: String,
    /// 公告状态
    pub status: i32,
    /// 公告状态名称
    pub status_name: String,
    /// 创建者
    pub create_by: String,
    /// 创建时间
    pub created_time: chrono::DateTime<chrono::Utc>,
    /// 更新时间
    pub updated_time: chrono::DateTime<chrono::Utc>,
    /// 备注
    pub remark: Option<String>,
}
