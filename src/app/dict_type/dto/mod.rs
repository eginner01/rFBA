/// 字典类型 DTO 模块
/// 包含所有的数据传输对象（DTO）

pub mod create_dict_type;
pub mod update_dict_type;
pub mod dict_type_response;
pub mod dict_type_query;

// 导出所有 DTO 类型
pub use create_dict_type::CreateDictTypeRequest;
pub use update_dict_type::UpdateDictTypeRequest;
pub use dict_type_response::{DictTypeResponse, DictTypePageResponse};
pub use dict_type_query::DictTypeQuery;

// 导出常用类型别名
pub type CreateDictType = CreateDictTypeRequest;
pub type UpdateDictType = UpdateDictTypeRequest;
pub type DictTypeListQuery = DictTypeQuery;
