/// 部门数据权限过滤器
/// 提供针对部门实体的数据权限过滤

use sea_orm::{ColumnTrait, QueryFilter, Select};
use crate::app::data_scope::dto::UserDataScope;
use crate::database::entity::dept;

/// 部门数据权限过滤器
pub struct DeptDataScopeFilter;

impl DeptDataScopeFilter {
    /// 为部门查询添加数据权限过滤条件
    /// 返回添加了权限过滤条件的查询构建器
    pub fn filter_dept_query(
        mut query: Select<dept::Entity>,
        user_data_scope: &UserDataScope,
    ) -> Select<dept::Entity> {
        // 如果可以查看全部数据，不添加过滤条件
        if user_data_scope.data_scopes.iter().any(|s| s.data_scope == 1) {
            return query;
        }

        // 构建部门ID集合
        let mut allowed_dept_ids = Vec::new();

        for scope in &user_data_scope.data_scopes {
            match scope.data_scope {
                // 本部门数据
                3 => {
                    if let Some(dept_id) = user_data_scope.dept_id {
                        allowed_dept_ids.push(dept_id);
                    }
                }
                // 本部门及以下数据
                4 => {
                    if let Some(dept_id) = user_data_scope.dept_id {
                        allowed_dept_ids.push(dept_id);
                        // TODO: 添加下级部门ID
                    }
                }
                // 自定义数据
                2 => {
                    if let Some(ref custom_data) = scope.custom_data {
                        allowed_dept_ids.extend_from_slice(custom_data);
                    }
                }
                _ => {}
            }
        }

        // 如果有允许的部门ID，添加过滤条件
        if !allowed_dept_ids.is_empty() {
            query = query.filter(dept::Column::Id.is_in(allowed_dept_ids));
        }

        query
    }

    /// 检查用户是否可以查看指定部门
    /// 返回是否可以查看
    pub fn can_view_dept(
        viewer_data_scope: &UserDataScope,
        target_dept: &dept::Model,
    ) -> bool {
        // 如果可以查看全部数据
        if viewer_data_scope.data_scopes.iter().any(|s| s.data_scope == 1) {
            return true;
        }

        // 检查本部门权限
        if viewer_data_scope.data_scopes.iter().any(|s| s.data_scope == 3) {
            if let Some(viewer_dept_id) = viewer_data_scope.dept_id {
                if target_dept.id == viewer_dept_id {
                    return true;
                }
            }
        }

        // 检查本部门及以下权限
        if viewer_data_scope.data_scopes.iter().any(|s| s.data_scope == 4) {
            if let Some(viewer_dept_id) = viewer_data_scope.dept_id {
                if target_dept.id == viewer_dept_id {
                    return true;
                }
                // TODO: 检查目标部门是否在下级
            }
        }

        // 检查自定义权限
        for scope in &viewer_data_scope.data_scopes {
            if scope.data_scope == 2 {
                if let Some(ref custom_data) = scope.custom_data {
                    if custom_data.contains(&target_dept.id) {
                        return true;
                    }
                }
            }
        }

        false
    }

    /// 获取用户可以访问的部门ID列表
    /// 返回部门ID列表
    pub fn get_allowed_dept_ids(
        user_data_scope: &UserDataScope,
    ) -> Vec<i64> {
        let mut allowed_dept_ids = Vec::new();

        for scope in &user_data_scope.data_scopes {
            match scope.data_scope {
                // 全部数据，返回所有部门ID（这里需要查询数据库）
                1 => {
                    // TODO: 返回所有部门ID
                    return Vec::new();
                }
                // 本部门数据
                3 => {
                    if let Some(dept_id) = user_data_scope.dept_id {
                        allowed_dept_ids.push(dept_id);
                    }
                }
                // 本部门及以下数据
                4 => {
                    if let Some(dept_id) = user_data_scope.dept_id {
                        allowed_dept_ids.push(dept_id);
                        // TODO: 添加下级部门ID
                    }
                }
                // 自定义数据
                2 => {
                    if let Some(ref custom_data) = scope.custom_data {
                        allowed_dept_ids.extend_from_slice(custom_data);
                    }
                }
                // 仅本人数据，不返回部门
                5 => {}
                _ => {}
            }
        }

        allowed_dept_ids
    }
}
