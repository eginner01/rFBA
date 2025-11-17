/// 权限管理模块
/// 提供权限CRUD、权限树、角色-权限关联等功能

pub mod dto;
pub mod service;
pub mod api;
pub mod router;

pub use dto::*;
pub use service::*;
pub use router::*;
