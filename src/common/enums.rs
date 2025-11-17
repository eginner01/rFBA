/// 用户权限类型枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum UserPermissionType {
    /// 超级用户权限
    Superuser,
    /// 员工权限
    Staff,
    /// 状态权限
    Status,
    /// 多点登录权限
    MultiLogin,
}

impl std::fmt::Display for UserPermissionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserPermissionType::Superuser => write!(f, "superuser"),
            UserPermissionType::Staff => write!(f, "staff"),
            UserPermissionType::Status => write!(f, "status"),
            UserPermissionType::MultiLogin => write!(f, "multi_login"),
        }
    }
}

impl From<UserPermissionType> for String {
    fn from(val: UserPermissionType) -> Self {
        val.to_string()
    }
}
