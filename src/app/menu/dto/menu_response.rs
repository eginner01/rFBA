/// 菜单响应 DTO

use serde::{Deserialize, Serialize};

/// 菜单详情响应
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MenuDetailResponse {
    /// 菜单ID
    pub id: i64,
    /// 菜单标题（多语言 key）
    pub title: String,
    /// 菜单名称
    pub name: String,
    /// 父菜单ID
    pub parent_id: Option<i64>,
    /// 显示顺序
    pub sort: i32,
    /// 路由路径
    pub path: Option<String>,
    /// 组件路径
    pub component: Option<String>,
    /// 菜单类型
    #[serde(rename = "type")]
    pub menu_type: i32,
    /// 权限编码
    pub perms: Option<String>,
    /// 菜单图标
    pub icon: Option<String>,
    /// 是否显示
    pub display: bool,
    /// 是否缓存
    pub cache: bool,
    /// 状态
    pub status: i32,
    /// 外链地址
    pub link: Option<String>,
    /// 备注
    pub remark: Option<String>,
    /// 创建时间
    pub created_time: chrono::DateTime<chrono::Utc>,
    /// 更新时间
    pub updated_time: Option<chrono::DateTime<chrono::Utc>>,
}

/// 菜单列表项响应
#[derive(Debug, Serialize, Deserialize)]
pub struct MenuListItem {
    /// 菜单ID
    pub id: i64,
    /// 菜单标题（多语言 key）
    pub title: String,
    /// 菜单名称
    pub name: String,
    /// 父菜单ID
    pub parent_id: Option<i64>,
    /// 显示顺序
    pub sort: i32,
    /// 路由路径
    pub path: Option<String>,
    /// 组件路径
    pub component: Option<String>,
    /// 菜单类型
    #[serde(rename = "type")]
    pub menu_type: i32,
    /// 权限编码
    pub perms: Option<String>,
    /// 菜单图标
    pub icon: Option<String>,
    /// 是否显示
    pub display: bool,
    /// 是否缓存
    pub cache: bool,
    /// 状态
    pub status: i32,
    /// 外链地址
    pub link: Option<String>,
    /// 备注
    pub remark: Option<String>,
    /// 创建时间
    pub created_time: chrono::DateTime<chrono::Utc>,
}

/// 菜单树节点
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MenuTreeNode {
    /// 菜单ID
    pub id: i64,
    /// 菜单标题（多语言 key）
    pub title: String,
    /// 菜单名称
    pub name: String,
    /// 父菜单ID
    pub parent_id: Option<i64>,
    /// 显示顺序
    pub sort: i32,
    /// 路由路径
    pub path: Option<String>,
    /// 组件路径
    pub component: Option<String>,
    /// 菜单类型
    pub menu_type: i32,
    /// 权限编码
    pub perms: Option<String>,
    /// 菜单图标
    pub icon: Option<String>,
    /// 是否显示
    pub display: bool,
    /// 是否缓存
    pub cache: bool,
    /// 状态
    pub status: i32,
    /// 外链地址
    pub link: Option<String>,
    /// 备注
    pub remark: Option<String>,
    /// 子菜单
    pub children: Vec<MenuTreeNode>,
}
