/// 角色管理模块
/// 提供角色CRUD、权限分配、用户关联等功能

pub mod dto;
pub mod service;
pub mod api;
pub mod router;

pub use dto::*;
pub use service::*;
pub use router::*;
