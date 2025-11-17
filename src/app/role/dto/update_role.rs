/// 更新角色请求 DTO

use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, Default)]
pub struct UpdateRoleRequest {
    /// 角色名称
    #[validate(length(min = 1, max = 50))]
    pub name: Option<String>,

    /// 角色编码
    #[validate(length(min = 1, max = 50))]
    pub code: Option<String>,

    /// 排序
    pub sort: Option<i32>,

    /// 是否启用数据权限过滤
    pub is_filter_scopes: Option<bool>,

    /// 状态（0: 禁用, 1: 启用）
    pub status: Option<i32>,

    /// 角色描述
    pub remark: Option<String>,
}
