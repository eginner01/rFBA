/// 创建菜单请求 DTO

use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateMenuRequest {
    /// 菜单标题（多语言 key）
    #[validate(length(min = 1, max = 100))]
    pub title: String,

    /// 菜单名称
    #[validate(length(min = 1, max = 50))]
    pub name: String,

    /// 父菜单ID
    pub parent_id: Option<i64>,

    /// 显示顺序
    pub sort: Option<i32>,

    /// 路由路径
    pub path: Option<String>,

    /// 组件路径
    pub component: Option<String>,

    /// 菜单类型（0: 目录, 1: 菜单, 2: 按钮）
    pub menu_type: i32,

    /// 权限编码
    pub perms: Option<String>,

    /// 菜单图标
    pub icon: Option<String>,

    /// 是否显示
    pub display: Option<bool>,

    /// 是否缓存
    pub cache: Option<bool>,

    /// 状态（0: 禁用, 1: 启用）
    pub status: Option<i32>,

    /// 外链地址
    pub link: Option<String>,

    /// 备注
    pub remark: Option<String>,
}

/// 菜单创建响应 DTO
#[derive(Debug, Serialize)]
pub struct CreateMenuResponse {
    /// 菜单ID
    pub id: i64,
    /// 菜单标题
    pub title: String,
    /// 菜单名称
    pub name: String,
    /// 菜单类型
    pub menu_type: i32,
    /// 菜单状态
    pub status: i32,
    /// 创建时间
    pub created_time: chrono::DateTime<chrono::Utc>,
}
