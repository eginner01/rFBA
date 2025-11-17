// FastAPI Best Architecture - Rust
// Enterprise-grade backend architecture solution

// 编译时抑制部分警告（开发中的功能）
#![allow(dead_code)]       // 允许未使用的代码（许多功能正在开发中）
#![allow(unused_variables)] // 允许未使用的变量（开发中的占位符）

// Clippy 警告抑制
#![allow(clippy::empty_line_after_doc_comments)]     // 文档注释后的空行（不影响功能）
#![allow(clippy::field_reassign_with_default)]       // 使用 Default 后的字段赋值（更清晰）
#![allow(clippy::too_many_arguments)]                // 函数参数过多（某些场景必需）
#![allow(clippy::ptr_arg)]                           // Vec 参数（性能影响小）
#![allow(clippy::single_match)]                      // 单一 match（保留注释需要）
#![allow(clippy::redundant_pattern_matching)]        // 冗余模式匹配（可能影响 drop 顺序）

pub mod app;
pub mod common;
pub mod core;
pub mod database;
pub mod middleware;
pub mod plugin;
pub mod utils;
pub mod websocket;

// 重新导出常用类型
pub use common::response::{ResponseModel, api_response, PaginatedData};
pub use common::exception::{AppError, ErrorCode};
pub use core::{SETTINGS, Settings};
// pub use middleware::{RequestContext, RequestState}; // TODO: 暂时注释，这些类型尚未实现
