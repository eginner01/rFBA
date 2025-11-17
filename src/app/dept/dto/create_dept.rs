/// 创建部门 DTO

use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateDeptRequest {
    /// 部门名称
    #[validate(length(min = 1, max = 64))]
    pub name: String,

    /// 父部门ID
    pub parent_id: Option<i64>,

    /// 显示顺序
    pub sort: i32,

    /// 负责人
    pub leader: Option<String>,

    /// 联系电话
    pub phone: Option<String>,

    /// 邮箱
    pub email: Option<String>,

    /// 部门状态（0: 正常, 1: 停用）
    pub status: i32,
}

/// 部门创建响应
#[derive(Debug, Serialize)]
pub struct CreateDeptResponse {
    /// 部门ID
    pub id: i64,
    /// 部门名称
    pub name: String,
    /// 父部门ID
    pub parent_id: Option<i64>,
    /// 创建时间
    pub created_time: chrono::DateTime<chrono::Utc>,
}

/// 部门更新请求
#[derive(Debug, Deserialize, Validate)]
#[serde(default)]
pub struct UpdateDeptRequest {
    /// 部门名称
    #[validate(length(min = 1, max = 64))]
    pub name: String,

    /// 父部门ID
    pub parent_id: Option<i64>,

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
}

impl Default for UpdateDeptRequest {
    fn default() -> Self {
        Self {
            name: String::new(),
            parent_id: None,
            sort: 0,
            leader: None,
            phone: None,
            email: None,
            status: 1, // 默认启用状态
        }
    }
}

/// 部门更新响应
#[derive(Debug, Serialize)]
pub struct UpdateDeptResponse {
    /// 部门ID
    pub id: i64,
    /// 部门名称
    pub name: String,
    /// 更新时间
    pub updated_time: chrono::DateTime<chrono::Utc>,
}
