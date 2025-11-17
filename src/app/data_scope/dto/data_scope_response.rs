/// 数据权限响应 DTO

use serde::{Deserialize, Serialize};

/// 数据权限列表响应（匹配 Python 的 PageData）
#[derive(Debug, Serialize)]
pub struct DataScopeListResponse {
    /// 数据权限配置列表（Python 中是 items）
    pub items: Vec<DataScopeDetailResponse>,
    /// 总数量
    pub total: usize,
    /// 当前页码
    pub page: usize,
    /// 每页数量
    pub size: usize,
    /// 总页数
    pub total_pages: usize,
}

/// 数据权限详情响应（匹配 Python 的 GetDataScopeDetail）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DataScopeDetailResponse {
    /// ID
    pub id: i64,
    /// 名称
    pub name: String,
    /// 状态（0停用 1正常）
    pub status: i32,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<chrono::DateTime<chrono::Utc>>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<chrono::DateTime<chrono::Utc>>,
}

/// 数据权限检查结果
#[derive(Debug, Serialize, Deserialize)]
pub struct DataScopeCheckResult {
    /// 是否有权限查看
    pub can_view: bool,
    /// 是否有权限编辑
    pub can_edit: bool,
    /// 是否有权限删除
    pub can_delete: bool,
    /// 数据范围过滤条件
    pub filter: Option<DataScopeCheckFilter>,
}

/// 数据权限检查过滤条件
#[derive(Debug, Serialize, Deserialize)]
pub struct DataScopeCheckFilter {
    /// 允许访问的部门ID列表
    pub dept_ids: Option<Vec<i64>>,
    /// 允许访问的用户ID列表
    pub user_ids: Option<Vec<i64>>,
    /// 是否查看全部数据
    pub view_all: bool,
    /// 是否查看本人数据
    pub view_self: bool,
}
