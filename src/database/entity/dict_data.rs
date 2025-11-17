//! 数据字典实体 - sys_dict_data表

use chrono::{DateTime, Utc};
use sea_orm::entity::{
    ActiveModelBehavior, ActiveModelTrait, ColumnTrait, DbErr, DeriveEntityModel,
    DerivePrimaryKey, DeriveRelationColumn, EntityFilter, EntityModel, EqTrait, ModelTrait,
    PrimaryKeyTrait, QueryFilter, QuerySelect, Related, RelationDef, RelationTrait, Set,
};
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize,
)]
#[sea_orm(table_name = "sys_dict_data")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub dict_code: i64,
    pub dict_sort: i32,
    pub dict_label: String,
    pub dict_value: String,
    pub dict_type: String,
    pub css_class: Option<String>,
    pub list_class: Option<String>,
    pub is_default: i32,
    pub status: i32,
    pub remark: Option<String>,
    pub created_time: DateTime<Utc>,
    pub updated_time: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DictStatus {
    Normal = 0,
    Disabled = 1,
}

impl DictStatus {
    /// 获取状态名称
    pub fn get_name(&self) -> &'static str {
        match self {
            DictStatus::Normal => "正常",
            DictStatus::Disabled => "停用",
        }
    }
}

impl From<i32> for DictStatus {
    fn from(value: i32) -> Self {
        match value {
            1 => DictStatus::Disabled,
            _ => DictStatus::Normal,
        }
    }
}

impl From<DictStatus> for i32 {
    fn from(value: DictStatus) -> Self {
        value as i32
    }
}

impl ActiveModelBehavior for ActiveModel {
    /// 在创建新记录前触发
    fn before_insert(model: sea_orm::ActiveModel<Self>) -> Result<Self, DbErr> {
        let mut model = model;
        if model.dict_code.as_ref() == &0 {
            model.dict_code = Set(chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0) as i64);
        }
        if model.created_time.as_ref() == &chrono::DateTime::default() {
            model.created_time = Set(Utc::now());
        }
        if model.updated_time.as_ref() == &chrono::DateTime::default() {
            model.updated_time = Set(Utc::now());
        }
        Ok(model)
    }

    /// 在更新记录前触发
    fn before_update(model: sea_orm::ActiveModel<Self>) -> Result<Self, DbErr> {
        let mut model = model;
        model.updated_time = Set(Utc::now());
        Ok(model)
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
#[sea_orm()]
pub enum Relation {
    /// 无关联
    #[sea_orm(name = "sys_dict_data_dict_code_fk")]
    SysDictDataDictCodeFk,
}

impl Related<sea_orm::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SysDictDataDictCodeFk.def()
    }
}

impl ActiveModel {
    /// 设置字典编码
    pub fn dict_code(mut self, dict_code: i64) -> Self {
        self.dict_code = Set(dict_code);
        self
    }

    /// 设置字典排序
    pub fn dict_sort(mut self, dict_sort: i32) -> Self {
        self.dict_sort = Set(dict_sort);
        self
    }

    /// 设置字典标签
    pub fn dict_label(mut self, dict_label: String) -> Self {
        self.dict_label = Set(dict_label);
        self
    }

    /// 设置字典键值
    pub fn dict_value(mut self, dict_value: String) -> Self {
        self.dict_value = Set(dict_value);
        self
    }

    /// 设置字典类型编码
    pub fn dict_type(mut self, dict_type: String) -> Self {
        self.dict_type = Set(dict_type);
        self
    }

    /// 设置样式属性
    pub fn css_class(mut self, css_class: Option<String>) -> Self {
        self.css_class = Set(css_class);
        self
    }

    /// 设置表格回显样式
    pub fn list_class(mut self, list_class: Option<String>) -> Self {
        self.list_class = Set(list_class);
        self
    }

    /// 设置是否默认
    pub fn is_default(mut self, is_default: i32) -> Self {
        self.is_default = Set(is_default);
        self
    }

    /// 设置状态
    pub fn status(mut self, status: DictStatus) -> Self {
        self.status = Set(status as i32);
        self
    }

    /// 设置备注
    pub fn remark(mut self, remark: Option<String>) -> Self {
        self.remark = Set(remark);
        self
    }

    /// 设置创建时间
    pub fn created_time(mut self, created_time: DateTime<Utc>) -> Self {
        self.created_time = Set(created_time);
        self
    }

    /// 设置更新时间
    pub fn updated_time(mut self, updated_time: DateTime<Utc>) -> Self {
        self.updated_time = Set(updated_time);
        self
    }
}

impl Entity {
    /// 根据字典类型查找
    pub fn find_by_dict_type(dict_type: &str) -> Select<Self> {
        Self::find().filter(Column::DictType.eq(dict_type))
    }

    /// 根据字典类型和状态查找
    pub fn find_by_dict_type_and_status(
        dict_type: &str,
        status: DictStatus,
    ) -> Select<Self> {
        Self::find()
            .filter(Column::DictType.eq(dict_type))
            .filter(Column::Status.eq(status as i32))
    }

    /// 根据字典键值查找
    pub fn find_by_dict_value(
        dict_type: &str,
        dict_value: &str,
    ) -> Select<Self> {
        Self::find()
            .filter(Column::DictType.eq(dict_type))
            .filter(Column::DictValue.eq(dict_value))
    }

    /// 根据状态查找
    pub fn find_by_status(status: DictStatus) -> Select<Self> {
        Self::find().filter(Column::Status.eq(status as i32))
    }

    /// 查找正常状态的数据
    pub fn find_normal() -> Select<Self> {
        Self::find().filter(Column::Status.eq(0))
    }
}

impl Model {
    /// 获取状态
    pub fn get_status(&self) -> DictStatus {
        DictStatus::from(self.status)
    }

    /// 检查是否为默认
    pub fn is_default(&self) -> bool {
        self.is_default == 1
    }

    /// 检查是否为正常状态
    pub fn is_normal(&self) -> bool {
        self.status == 0
    }

    /// 检查是否为停用状态
    pub fn is_disabled(&self) -> bool {
        self.status == 1
    }
}
