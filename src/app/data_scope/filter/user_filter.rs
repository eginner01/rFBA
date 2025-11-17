/// 用户数据权限过滤器
/// 提供针对用户实体的数据权限过滤

use sea_orm::{ColumnTrait, QueryFilter, Select};
use crate::app::data_scope::dto::UserDataScope;
use crate::database::entity::user;

/// 用户数据权限过滤器
pub struct UserDataScopeFilter;

impl UserDataScopeFilter {
    /// 为用户查询添加数据权限过滤条件
    /// 返回添加了权限过滤条件的查询构建器
    pub fn filter_user_query(
        query: Select<user::Entity>,
        user_data_scope: &UserDataScope,
    ) -> Select<user::Entity> {
        // 如果可以查看全部数据，不添加过滤条件
        if user_data_scope.data_scopes.iter().any(|s| s.data_scope == 1) {
            return query;
        }

        // 构建部门ID和用户ID集合
        let mut allowed_dept_ids = Vec::new();
        let mut allowed_user_ids = Vec::new();

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
                // 仅本人数据
                5 => {
                    allowed_user_ids.push(user_data_scope.user_id);
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

        // 构建过滤条件
        let mut conditions = sea_orm::Condition::any();

        if !allowed_dept_ids.is_empty() {
            conditions = conditions.add(
                user::Column::DeptId.is_in(allowed_dept_ids.clone())
            );
        }

        if !allowed_user_ids.is_empty() {
            conditions = conditions.add(
                user::Column::Id.is_in(allowed_user_ids.clone())
            );
        }

        // 如果没有特殊权限，默认只允许查看本人数据
        if allowed_dept_ids.is_empty() && allowed_user_ids.is_empty() {
            conditions = conditions.add(
                user::Column::Id.eq(user_data_scope.user_id)
            );
        }

        query.filter(conditions)
    }

    /// 检查用户是否可以查看另一个用户
    /// 返回是否可以查看
    pub fn can_view_user(
        viewer_data_scope: &UserDataScope,
        target_user: &user::Model,
    ) -> bool {
        // 如果可以查看全部数据
        if viewer_data_scope.data_scopes.iter().any(|s| s.data_scope == 1) {
            return true;
        }

        // 检查本部门权限
        if viewer_data_scope.data_scopes.iter().any(|s| s.data_scope == 3) {
            if let Some(viewer_dept_id) = viewer_data_scope.dept_id {
                if target_user.dept_id == Some(viewer_dept_id) {
                    return true;
                }
            }
        }

        // 检查本部门及以下权限
        if viewer_data_scope.data_scopes.iter().any(|s| s.data_scope == 4) {
            if let Some(viewer_dept_id) = viewer_data_scope.dept_id {
                if target_user.dept_id == Some(viewer_dept_id) {
                    return true;
                }
                // TODO: 检查目标用户是否在下级部门
            }
        }

        // 检查仅本人权限
        if viewer_data_scope.data_scopes.iter().all(|s| s.data_scope == 5) {
            return viewer_data_scope.user_id == target_user.id;
        }

        // 检查自定义权限
        for scope in &viewer_data_scope.data_scopes {
            if scope.data_scope == 2 {
                if let Some(ref custom_data) = scope.custom_data {
                    if let Some(target_dept_id) = target_user.dept_id {
                        if custom_data.contains(&target_dept_id) {
                            return true;
                        }
                    }
                }
            }
        }

        false
    }
}
