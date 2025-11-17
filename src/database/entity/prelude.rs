/// 预导入模块
/// 导出所有常用实体和查询类型

pub use super::user::Entity as User;
pub use super::user::Model as UserModel;
pub use super::user::Column as UserColumn;
pub use super::user::ActiveModel as UserActiveModel;

pub use super::role::Entity as Role;
pub use super::role::Model as RoleModel;
pub use super::role::Column as RoleColumn;
pub use super::role::ActiveModel as RoleActiveModel;

pub use super::menu::Entity as Menu;
pub use super::menu::Model as MenuModel;
pub use super::menu::Column as MenuColumn;
pub use super::menu::ActiveModel as MenuActiveModel;

pub use super::dept::Entity as Dept;
pub use super::dept::Model as DeptModel;
pub use super::dept::Column as DeptColumn;
pub use super::dept::ActiveModel as DeptActiveModel;

// pub use super::dict_data::Entity as DictData; // TODO: Fix dict_data entity
// pub use super::dict_data::Model as DictDataModel;
// pub use super::dict_data::Column as DictDataColumn;
// pub use super::dict_data::ActiveModel as DictDataActiveModel;
// pub use super::dict_data::DictStatus;

pub use super::dict_type::Entity as DictType;
pub use super::dict_type::Model as DictTypeModel;
