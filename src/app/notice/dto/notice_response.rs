/// 通知公告响应 DTO

use serde::{Deserialize, Serialize};

/// 公告详情响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoticeDetailResponse {
    /// 公告ID
    pub notice_id: i64,
    /// 公告标题
    pub notice_title: String,
    /// 公告类型
    pub notice_type: i32,
    /// 公告类型名称
    pub notice_type_name: String,
    /// 公告内容
    pub notice_content: String,
    /// 公告状态
    pub status: i32,
    /// 公告状态名称
    pub status_name: String,
    /// 创建者
    pub create_by: String,
    /// 创建时间
    pub created_time: chrono::DateTime<chrono::Utc>,
    /// 更新者
    pub update_by: Option<String>,
    /// 更新时间
    pub updated_time: chrono::DateTime<chrono::Utc>,
    /// 备注
    pub remark: Option<String>,
}

/// 公告类型统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoticeTypeStatistics {
    /// 公告类型
    pub notice_type: i32,
    /// 公告类型名称
    pub notice_type_name: String,
    /// 公告数量
    pub count: usize,
}

/// 公告分组统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoticeGroupStatistics {
    /// 正常状态数量
    pub normal_count: usize,
    /// 关闭状态数量
    pub closed_count: usize,
    /// 通知数量
    pub notification_count: usize,
    /// 公告数量
    pub announcement_count: usize,
    /// 总公告数量
    pub total_count: usize,
}
