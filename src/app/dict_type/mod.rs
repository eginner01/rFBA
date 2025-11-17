/// 字典类型模块
/// 用于管理字典的类型/分类

pub mod api;
pub mod dto;
pub mod router;
pub mod service;

// 导出所有公共类型和函数
pub use api::*;
pub use dto::*;
pub use router::*;
pub use service::*;

// 模块常量
pub const MODULE_NAME: &str = "dict_type";
pub const API_PREFIX: &str = "/api/v1/dict-types";

/// 字典类型错误代码
pub mod error_codes {
    pub const DICT_TYPE_NOT_FOUND: &str = "DICT_TYPE_NOT_FOUND";
    pub const DICT_TYPE_EXISTS: &str = "DICT_TYPE_EXISTS";
    pub const DICT_TYPE_HAS_DATA: &str = "DICT_TYPE_HAS_DATA";
    pub const INVALID_STATUS: &str = "INVALID_STATUS";
    pub const INVALID_DEFAULT: &str = "INVALID_DEFAULT";
}

/// 字典类型状态枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DictTypeStatus {
    Normal = 0,
    Disabled = 1,
}

impl DictTypeStatus {
    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            0 => Some(DictTypeStatus::Normal),
            1 => Some(DictTypeStatus::Disabled),
            _ => None,
        }
    }

    pub fn to_i32(self) -> i32 {
        self as i32
    }

    pub fn get_text(&self) -> &'static str {
        match self {
            DictTypeStatus::Normal => "正常",
            DictTypeStatus::Disabled => "停用",
        }
    }
}

/// 是否默认枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IsDefault {
    No = 0,
    Yes = 1,
}

impl IsDefault {
    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            0 => Some(IsDefault::No),
            1 => Some(IsDefault::Yes),
            _ => None,
        }
    }

    pub fn to_i32(self) -> i32 {
        self as i32
    }

    pub fn get_text(&self) -> &'static str {
        match self {
            IsDefault::No => "否",
            IsDefault::Yes => "是",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dict_type_status_from_i32() {
        assert_eq!(DictTypeStatus::from_i32(0), Some(DictTypeStatus::Normal));
        assert_eq!(DictTypeStatus::from_i32(1), Some(DictTypeStatus::Disabled));
        assert_eq!(DictTypeStatus::from_i32(2), None);
    }

    #[test]
    fn test_dict_type_status_to_i32() {
        assert_eq!(DictTypeStatus::Normal.to_i32(), 0);
        assert_eq!(DictTypeStatus::Disabled.to_i32(), 1);
    }

    #[test]
    fn test_dict_type_status_get_text() {
        assert_eq!(DictTypeStatus::Normal.get_text(), "正常");
        assert_eq!(DictTypeStatus::Disabled.get_text(), "停用");
    }

    #[test]
    fn test_is_default_from_i32() {
        assert_eq!(IsDefault::from_i32(0), Some(IsDefault::No));
        assert_eq!(IsDefault::from_i32(1), Some(IsDefault::Yes));
        assert_eq!(IsDefault::from_i32(2), None);
    }

    #[test]
    fn test_is_default_to_i32() {
        assert_eq!(IsDefault::No.to_i32(), 0);
        assert_eq!(IsDefault::Yes.to_i32(), 1);
    }

    #[test]
    fn test_is_default_get_text() {
        assert_eq!(IsDefault::No.get_text(), "否");
        assert_eq!(IsDefault::Yes.get_text(), "是");
    }

    #[test]
    fn test_constants() {
        assert_eq!(MODULE_NAME, "dict_type");
        assert_eq!(API_PREFIX, "/api/v1/dict-types");
        assert_eq!(error_codes::DICT_TYPE_NOT_FOUND, "DICT_TYPE_NOT_FOUND");
        assert_eq!(error_codes::DICT_TYPE_EXISTS, "DICT_TYPE_EXISTS");
        assert_eq!(error_codes::DICT_TYPE_HAS_DATA, "DICT_TYPE_HAS_DATA");
    }
}
