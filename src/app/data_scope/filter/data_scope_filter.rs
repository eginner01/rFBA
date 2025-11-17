/// 数据权限过滤器
/// 提供根据用户数据权限过滤查询结果的功能

use sea_orm::{ColumnTrait, Condition, QueryFilter, Select};
use crate::app::data_scope::dto::{DataScopeFilter, UserDataScope};

/// 数据权限过滤器
pub struct DataScopeFilterBuilder;

impl DataScopeFilterBuilder {
    /// 为用户表构建数据权限过滤条件
    /// 返回添加了权限过滤条件的查询构建器
    pub fn filter_for_user_table<T, C>(
        query: Select<T>,
        user_data_scope: &UserDataScope,
        user_id_col: C,
        dept_id_col: C,
    ) -> Select<T>
    where
        T: sea_orm::EntityTrait,
        C: ColumnTrait + Clone,
    {
        // 获取允许的部门ID和用户ID
        let allowed_dept_ids = &user_data_scope.data_scopes[0].custom_data.clone().unwrap_or_default();
        let allowed_user_ids = &user_data_scope.data_scopes[0].custom_data.clone().unwrap_or_default();

        // 构建过滤条件
        let mut conditions = Condition::any();

        // 如果可以查看全部数据，不添加过滤条件
        if !user_data_scope.data_scopes.is_empty() &&
           user_data_scope.data_scopes[0].data_scope == 1 {
            return query;
        }

        // 添加部门过滤条件
        if !allowed_dept_ids.is_empty() {
            conditions = conditions.add(dept_id_col.is_in(allowed_dept_ids.clone()));
        }

        // 添加用户过滤条件
        if !allowed_user_ids.is_empty() {
            conditions = conditions.add(user_id_col.is_in(allowed_user_ids.clone()));
        }

        // 如果没有特殊权限，只允许查看本人数据
        if allowed_dept_ids.is_empty() && allowed_user_ids.is_empty() {
            conditions = conditions.add(user_id_col.eq(user_data_scope.user_id));
        }

        query.filter(conditions)
    }

    /// 为用户表构建数据权限过滤条件（简化版）
    /// 返回添加了权限过滤条件的查询构建器
    pub fn filter_for_user_table_simple<T, C>(
        query: Select<T>,
        data_scope: &DataScopeFilter,
        user_id_col: C,
        dept_id_col: C,
    ) -> Select<T>
    where
        T: sea_orm::EntityTrait,
        C: ColumnTrait + Clone,
    {
        // 如果可以查看全部数据，不添加过滤条件
        if data_scope.can_view_all {
            return query;
        }

        // 构建过滤条件
        let mut conditions = Condition::any();

        // 添加部门过滤条件
        if !data_scope.allowed_dept_ids.is_empty() {
            conditions = conditions.add(dept_id_col.is_in(data_scope.allowed_dept_ids.clone()));
        }

        // 添加用户过滤条件
        if !data_scope.allowed_user_ids.is_empty() {
            conditions = conditions.add(user_id_col.is_in(data_scope.allowed_user_ids.clone()));
        }

        // 如果没有特殊权限，只允许查看本人数据
        if data_scope.can_view_self {
            conditions = conditions.add(user_id_col.is_in(data_scope.allowed_user_ids.clone()));
        }

        query.filter(conditions)
    }

    /// 检查用户是否有权限查看指定用户的数据
    /// 返回是否有权限
    pub fn can_view_user(
        user_data_scope: &UserDataScope,
        target_user_id: i64,
        target_dept_id: Option<i64>,
    ) -> bool {
        // 如果可以查看全部数据，有权限
        if user_data_scope.data_scopes.iter().any(|s| s.data_scope == 1) {
            return true;
        }

        // 检查是否有权限查看指定部门的数据
        if let Some(dept_id) = target_dept_id {
            for scope in &user_data_scope.data_scopes {
                if let Some(ref custom_data) = scope.custom_data {
                    // 如果自定义数据中包含目标部门ID，有权限
                    if custom_data.contains(&dept_id) {
                        return true;
                    }
                }
            }
        }

        // 检查是否只能查看本人数据
        if user_data_scope.data_scopes.iter().all(|s| s.data_scope == 5) {
            return user_data_scope.user_id == target_user_id;
        }

        false
    }

    /// 检查用户是否有权限编辑指定用户的数据
    /// 返回是否有权限
    pub fn can_edit_user(
        user_data_scope: &UserDataScope,
        target_user_id: i64,
        target_dept_id: Option<i64>,
    ) -> bool {
        // 编辑权限等同于查看权限
        Self::can_view_user(user_data_scope, target_user_id, target_dept_id)
    }

    /// 检查用户是否有权限删除指定用户的数据
    /// 返回是否有权限
    pub fn can_delete_user(
        user_data_scope: &UserDataScope,
        target_user_id: i64,
        target_dept_id: Option<i64>,
    ) -> bool {
        // 删除权限要求更高，通常只有本部门或更高权限
        for scope in &user_data_scope.data_scopes {
            match scope.data_scope {
                // 全部数据可以删除
                1 => return true,
                // 本部门及以下数据可以删除
                4 => {
                    if let Some(dept_id) = target_dept_id {
                        if let Some(ref custom_data) = scope.custom_data {
                            if custom_data.contains(&dept_id) {
                                return true;
                            }
                        }
                    }
                }
                // 仅本人数据只能删除自己的
                5 => return user_data_scope.user_id == target_user_id,
                _ => {}
            }
        }

        false
    }
}
