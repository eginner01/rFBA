/// 数据权限提取器
/// 从HTTP请求中提取数据权限信息

use axum::extract::Request;
use crate::app::data_scope::dto::UserDataScope;

/// 数据权限提取器
pub struct DataScopeExtractor;

impl DataScopeExtractor {
    /// 从请求中提取用户数据权限
    /// 返回用户数据权限
    pub fn extract_user_data_scope(request: &Request) -> Option<&UserDataScope> {
        request.extensions().get::<UserDataScope>()
    }

    /// 从请求中提取用户ID
    /// 返回用户ID
    pub fn extract_user_id(request: &Request) -> Option<i64> {
        Self::extract_user_data_scope(request).map(|user_data_scope| user_data_scope.user_id)
    }

    /// 从请求中提取部门ID
    /// 返回部门ID
    pub fn extract_dept_id(request: &Request) -> Option<i64> {
        if let Some(user_data_scope) = Self::extract_user_data_scope(request) {
            user_data_scope.dept_id
        } else {
            None
        }
    }

    /// 从请求中提取用户角色ID列表
    /// 返回角色ID列表
    pub fn extract_role_ids(request: &Request) -> Option<&Vec<i64>> {
        if let Some(user_data_scope) = Self::extract_user_data_scope(request) {
            Some(&user_data_scope.role_ids)
        } else {
            None
        }
    }

    /// 从请求中提取数据权限配置
    /// 返回数据权限配置列表
    pub fn extract_data_scopes(request: &Request) -> Option<&Vec<crate::app::data_scope::dto::UserDataScopeItem>> {
        if let Some(user_data_scope) = Self::extract_user_data_scope(request) {
            Some(&user_data_scope.data_scopes)
        } else {
            None
        }
    }

    /// 从请求中检查是否有全部数据权限
    /// 返回是否有全部数据权限
    pub fn can_view_all_data(request: &Request) -> bool {
        if let Some(data_scopes) = Self::extract_data_scopes(request) {
            data_scopes.iter().any(|s| s.data_scope == 1)
        } else {
            false
        }
    }

    /// 从请求中检查是否只能查看本人数据
    /// 返回是否只能查看本人数据
    pub fn can_view_only_self(request: &Request) -> bool {
        if let Some(data_scopes) = Self::extract_data_scopes(request) {
            data_scopes.iter().all(|s| s.data_scope == 5)
        } else {
            false
        }
    }

    /// 从请求中检查是否有指定部门的数据权限
    /// 返回是否有权限
    pub fn can_view_dept(request: &Request, dept_id: i64) -> bool {
        if let Some(data_scopes) = Self::extract_data_scopes(request) {
            for scope in data_scopes {
                match scope.data_scope {
                    // 全部数据
                    1 => return true,
                    // 本部门数据
                    3 => {
                        if let Some(user_dept_id) = Self::extract_dept_id(request) {
                            if user_dept_id == dept_id {
                                return true;
                            }
                        }
                    }
                    // 自定义数据
                    2 => {
                        if let Some(ref custom_data) = scope.custom_data {
                            if custom_data.contains(&dept_id) {
                                return true;
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        false
    }

    /// 从请求中检查是否有指定用户的数据权限
    /// 返回是否有权限
    pub fn can_view_user(
        request: &Request,
        target_user_id: i64,
        target_dept_id: Option<i64>,
    ) -> bool {
        // 如果可以查看全部数据
        if Self::can_view_all_data(request) {
            return true;
        }

        // 如果只能查看本人数据
        if Self::can_view_only_self(request) {
            return Self::extract_user_id(request) == Some(target_user_id);
        }

        // 检查是否有指定部门的数据权限
        if let Some(dept_id) = target_dept_id {
            if Self::can_view_dept(request, dept_id) {
                return true;
            }
        }

        false
    }

    /// 从请求中获取允许的部门ID列表
    /// 返回部门ID列表
    pub fn get_allowed_dept_ids(request: &Request) -> Vec<i64> {
        let mut dept_ids = Vec::new();

        if let Some(data_scopes) = Self::extract_data_scopes(request) {
            for scope in data_scopes {
                match scope.data_scope {
                    // 全部数据，返回空列表（需要查询所有部门）
                    1 => return Vec::new(),
                    // 本部门数据
                    3 => {
                        if let Some(dept_id) = Self::extract_dept_id(request) {
                            dept_ids.push(dept_id);
                        }
                    }
                    // 本部门及以下数据
                    4 => {
                        if let Some(dept_id) = Self::extract_dept_id(request) {
                            dept_ids.push(dept_id);
                            // TODO: 添加下级部门ID
                        }
                    }
                    // 自定义数据
                    2 => {
                        if let Some(ref custom_data) = scope.custom_data {
                            dept_ids.extend_from_slice(custom_data);
                        }
                    }
                    // 仅本人数据，不返回部门
                    5 => {}
                    _ => {}
                }
            }
        }

        dept_ids
    }

    /// 从请求中获取允许的用户ID列表
    /// 返回用户ID列表
    pub fn get_allowed_user_ids(request: &Request) -> Vec<i64> {
        let mut user_ids = Vec::new();

        if let Some(data_scopes) = Self::extract_data_scopes(request) {
            for scope in data_scopes {
                match scope.data_scope {
                    // 仅本人数据
                    5 => {
                        if let Some(user_id) = Self::extract_user_id(request) {
                            user_ids.push(user_id);
                        }
                    }
                    _ => {}
                }
            }
        }

        user_ids
    }
}
