/// 角色-权限关联管理模块
/// 提供角色权限分配、权限角色查询等功能

pub mod dto;
pub mod service;
pub mod api;
pub mod router;

pub use dto::*;
pub use service::*;
pub use router::*;
