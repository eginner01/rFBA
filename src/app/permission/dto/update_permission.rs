/// 更新权限请求 DTO

use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, Default)]
pub struct UpdatePermissionRequest {
    /// 权限名称
    #[validate(length(min = 1, max = 50))]
    pub name: Option<String>,

    /// 权限编码
    #[validate(length(min = 1, max = 100))]
    pub code: Option<String>,

    /// 权限类型（0: 目录, 1: 菜单, 2: 按钮）
    pub permission_type: Option<i32>,

    /// 父权限ID
    pub parent_id: Option<i64>,

    /// 排序
    pub sort: Option<i32>,

    /// 权限描述
    pub remark: Option<String>,

    /// 是否启用
    pub status: Option<i32>,
}
