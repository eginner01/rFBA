/// 字典类型路由模块

pub mod dict_type_router;

// 导出路由函数
pub use dict_type_router::{create_dict_type_router, get_dict_type_routes};

/// 字典类型路由别名
pub type DictTypeRouter = axum::Router;
