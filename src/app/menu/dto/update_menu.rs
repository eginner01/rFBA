/// 更新菜单请求 DTO

use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, Default)]
pub struct UpdateMenuRequest {
    /// 菜单标题（多语言 key）
    #[validate(length(min = 1, max = 100))]
    pub title: Option<String>,

    /// 菜单名称
    #[validate(length(min = 1, max = 50))]
    pub name: Option<String>,

    /// 父菜单ID
    pub parent_id: Option<i64>,

    /// 显示顺序
    pub sort: Option<i32>,

    /// 路由路径
    pub path: Option<String>,

    /// 组件路径
    pub component: Option<String>,

    /// 菜单类型（0: 目录, 1: 菜单, 2: 按钮）
    pub menu_type: Option<i32>,

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
