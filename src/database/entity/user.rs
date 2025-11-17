/// 用户实体模型
/// 对应数据库中的 sys_user 表
use sea_orm::DeriveRelation;

use sea_orm::prelude::*;
use sea_orm::EnumIter;
use serde::{Deserialize, Serialize};

/// 用户实体
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "sys_user")]
pub struct Model {
    /// 主键 ID
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    /// UUID 唯一标识
    #[sea_orm(unique, indexed)]
    pub uuid: String,
    /// 用户名
    #[sea_orm(unique, indexed)]
    pub username: String,
    /// 昵称
    pub nickname: String,
    /// 密码（Bcrypt 加密）
    pub password: Option<String>,
    /// 加密盐（BLOB 类型，对应数据库中的 BLOB 列）
    pub salt: Option<Vec<u8>>,
    /// 邮箱
    pub email: Option<String>,
    /// 手机号
    pub phone: Option<String>,
    /// 头像
    pub avatar: Option<String>,
    /// 用户状态（0: 停用, 1: 正常）
    #[sea_orm(indexed)]
    pub status: i32,
    /// 是否超级管理员
    pub is_superuser: bool,
    /// 是否有后台管理权限
    pub is_staff: bool,
    /// 是否允许多端登录
    pub is_multi_login: bool,
    /// 注册时间
    pub join_time: DateTime,
    /// 上次登录时间
    pub last_login_time: Option<DateTime>,
    /// 部门 ID
    pub dept_id: Option<i64>,
    /// 创建时间
    pub created_time: DateTime,
    /// 更新时间
    pub updated_time: DateTime,
    /// 删除标志（0: 未删除, 1: 已删除）
    pub del_flag: i32,
}

/// Empty Relation enum (we'll implement relations manually later)
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

// impl Related<super::dept::Entity> for Entity {
//     fn to() -> RelationDef {
//         Relation::Dept.def()
//     }
// }

// impl Related<super::role::Entity> for Entity {
//     fn to() -> RelationDef {
//         // 用户-角色 多对多关系需要通过关联表定义
//         // 这里暂时不实现多对多关系，Sea-ORM 的多对多关系实现比较复杂
//         // 可以通过查询用户-角色关联表来获取
//         todo!()
//     }
// }

// 使用默认实现
impl ActiveModelBehavior for ActiveModel {}

/// 用户查询模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserQuery {
    /// 用户名
    pub username: Option<String>,
    /// 昵称
    pub nickname: Option<String>,
    /// 状态
    pub status: Option<i32>,
    /// 部门 ID
    pub dept_id: Option<i64>,
    /// 关键词
    pub keyword: Option<String>,
    /// 页码
    pub page: Option<u64>,
    /// 每页数量
    pub size: Option<u64>,
    /// 排序字段
    pub sort: Option<String>,
    /// 排序方向（asc, desc）
    pub order: Option<String>,
}

/// 用户创建模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub nickname: String,
    pub password: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub avatar: Option<String>,
    pub status: Option<i32>,
    pub is_superuser: Option<bool>,
    pub is_staff: Option<bool>,
    pub is_multi_login: Option<bool>,
    pub dept_id: Option<i64>,
    pub role_ids: Option<Vec<i64>>,
}

/// 用户更新模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUser {
    pub nickname: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub avatar: Option<String>,
    pub status: Option<i32>,
    pub is_superuser: Option<bool>,
    pub is_staff: Option<bool>,
    pub is_multi_login: Option<bool>,
    pub dept_id: Option<i64>,
    pub role_ids: Option<Vec<i64>>,
}

/// 用户详情响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDetail {
    pub id: i64,
    pub uuid: String,
    pub username: String,
    pub nickname: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub avatar: Option<String>,
    pub status: i32,
    pub is_superuser: bool,
    pub is_staff: bool,
    pub is_multi_login: bool,
    pub join_time: DateTime,
    pub last_login_time: Option<DateTime>,
    pub dept_id: Option<i64>,
    pub dept_name: Option<String>,
    pub created_time: DateTime,
    pub updated_time: DateTime,
    pub role_ids: Option<Vec<i64>>,
    pub role_names: Option<Vec<String>>,
}
