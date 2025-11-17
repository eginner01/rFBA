/// RBAC 权限控制模块
///
/// 基于角色的访问控制（Role-Based Access Control）
///
/// 该模块目前是占位符，实际的RBAC实现在业务逻辑中

// 占位模块
pub use crate::app::auth::service::rbac_service::{check_permission, has_role, has_any_role, has_all_roles};
