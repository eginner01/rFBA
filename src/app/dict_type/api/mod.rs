/// 字典类型 API 模块
/// 包含所有的 API 处理器

pub mod dict_type;

// 导出所有 API 处理器
pub use dict_type::{
    batch_delete_dict_types, create_dict_type, delete_dict_type,
    get_dict_type, get_dict_type_options, get_dict_types,
    update_dict_type,
};
