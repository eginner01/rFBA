/// 数据权限查询 DTO

use serde::{Deserialize, Serialize};

/// 用户数据权限查询结果
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserDataScope {
    /// 用户ID
    pub user_id: i64,
    /// 用户名
    pub username: String,
    /// 角色ID列表
    pub role_ids: Vec<i64>,
    /// 部门ID
    pub dept_id: Option<i64>,
    /// 数据权限配置
    pub data_scopes: Vec<UserDataScopeItem>,
}

/// 单个角色的数据权限
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserDataScopeItem {
    /// 角色ID
    pub role_id: i64,
    /// 角色名称
    pub role_name: String,
    /// 数据权限类型
    pub data_scope: i32,
    /// 数据权限类型名称
    pub data_scope_name: String,
    /// 自定义数据范围（部门ID列表）
    pub custom_data: Option<Vec<i64>>,
}

/// 数据范围过滤条件
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DataScopeFilter {
    /// 允许访问的部门ID列表
    pub allowed_dept_ids: Vec<i64>,
    /// 允许访问的用户ID列表
    pub allowed_user_ids: Vec<i64>,
    /// 是否查看全部数据
    pub can_view_all: bool,
    /// 是否只能查看本人数据
    pub can_view_self: bool,
}

/// 数据权限查询参数
#[derive(Debug, Deserialize, Default)]
pub struct DataScopeQueryParams {
    /// 用户ID
    pub user_id: Option<i64>,
    /// 角色ID
    pub role_id: Option<i64>,
    /// 数据权限类型
    pub data_scope: Option<i32>,
    /// 页码
    pub page: Option<u64>,
    /// 每页数量
    pub size: Option<u64>,
}

/// 数据权限树节点
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DataScopeTreeNode {
    /// 部门ID
    pub dept_id: i64,
    /// 部门名称
    pub dept_name: String,
    /// 父部门ID
    pub parent_id: Option<i64>,
    /// 是否可选
    pub selectable: bool,
    /// 子部门
    pub children: Vec<DataScopeTreeNode>,
}
