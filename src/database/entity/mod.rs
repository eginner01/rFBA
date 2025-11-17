/// 数据库实体模块
/// 定义所有数据库表对应的实体模型

pub mod prelude;

pub mod user;
pub mod role;
pub mod menu;
pub mod dept;
pub mod data_scope;
pub mod data_rule;
pub mod role_menu;
pub mod login_log;
pub mod opera_log;
// pub mod dict_data; // TODO: Fix dict_data entity
pub mod dict_type;
pub mod task_scheduler;
pub mod task_result;

pub use prelude::*;
