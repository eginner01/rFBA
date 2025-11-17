/// 部门查询 DTO

use serde::{Deserialize, Serialize};

/// 部门树查询参数
#[derive(Debug, Deserialize, Default)]
pub struct DeptTreeQuery {
    /// 父部门ID
    pub parent_id: Option<i64>,
}

/// 部门列表查询参数
#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeptListQuery {
    /// 部门名称关键词搜索
    pub dept_name: Option<String>,

    /// 部门状态（0:正常 1:停用）
    pub status: Option<i32>,

    /// 部门负责人
    pub leader: Option<String>,

    /// 父部门ID
    pub parent_id: Option<i64>,

    /// 是否查询包含子部门（用于树形显示）
    pub include_children: Option<bool>,
}

/// 部门状态查询参数（用于状态变更）
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeptStatusQuery {
    /// 部门状态（0:正常 1:停用）
    pub status: i32,
}

/// 部门树节点
#[derive(Debug, Serialize, Deserialize)]
pub struct DeptTreeNode {
    /// 部门ID
    pub id: i64,
    /// 部门名称
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
    /// 部门状态名称
    pub status_name: String,
    /// 子部门
    pub children: Vec<DeptTreeNode>,
}

/// 部门列表项
#[derive(Debug, Serialize, Deserialize)]
pub struct DeptListItem {
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
    /// 创建时间
    pub created_time: chrono::DateTime<chrono::Utc>,
    /// 更新时间
    pub updated_time: Option<chrono::DateTime<chrono::Utc>>,
}
