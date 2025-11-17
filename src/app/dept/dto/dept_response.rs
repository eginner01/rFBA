/// 部门响应 DTO

use serde::{Deserialize, Serialize};

/// 部门详情响应
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeptDetailResponse {
    /// 部门ID
    pub id: i64,
    /// 部门名称
    pub name: String,
    /// 父部门ID
    pub parent_id: Option<i64>,
    /// 父部门名称
    pub parent_name: Option<String>,
    /// 显示顺序
    pub sort: i32,
    /// 负责人
    pub leader: Option<String>,
    /// 联系电话
    pub phone: Option<String>,
    /// 邮箱
    pub email: Option<String>,
    /// 部门状态
    pub status: i32,
    /// 部门状态名称
    pub status_name: String,
    /// 创建人
    pub create_by: Option<String>,
    /// 更新人
    pub update_by: Option<String>,
    /// 创建时间
    pub created_time: chrono::DateTime<chrono::Utc>,
    /// 更新时间
    pub updated_time: Option<chrono::DateTime<chrono::Utc>>,
    /// 删除标志
    pub del_flag: bool,
}

// 类型别名以保持API兼容性
pub type DeptResponse = DeptDetailResponse;
