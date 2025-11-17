/// 通知公告实体

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
#[sea_orm(table_name = "sys_notice")]
pub struct Model {
    /// 公告ID
    #[sea_orm(primary_key, auto_increment = false)]
    pub notice_id: i64,
    /// 公告标题
    pub notice_title: String,
    /// 公告类型（1:通知 2:公告）
    pub notice_type: i32,
    /// 公告内容
    pub notice_content: String,
    /// 公告状态（0:正常 1:关闭）
    pub status: i32,
    /// 创建者
    pub create_by: String,
    /// 创建时间
    pub created_time: DateTime<Utc>,
    /// 更新者
    pub update_by: Option<String>,
    /// 更新时间
    pub updated_time: DateTime<Utc>,
    /// 备注
    pub remark: Option<String>,
}

/// 公告类型枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NoticeType {
    /// 通知
    Notification = 1,
    /// 公告
    Announcement = 2,
}

impl NoticeType {
    /// 获取类型名称
    pub fn get_name(&self) -> &'static str {
        match self {
            NoticeType::Notification => "通知",
            NoticeType::Announcement => "公告",
        }
    }
}

impl From<i32> for NoticeType {
    fn from(value: i32) -> Self {
        match value {
            2 => NoticeType::Announcement,
            _ => NoticeType::Notification,
        }
    }
}

impl From<NoticeType> for i32 {
    fn from(value: NoticeType) -> Self {
        value as i32
    }
}

/// 公告状态枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NoticeStatus {
    /// 正常
    Normal = 0,
    /// 关闭
    Closed = 1,
}

impl NoticeStatus {
    /// 获取状态名称
    pub fn get_name(&self) -> &'static str {
        match self {
            NoticeStatus::Normal => "正常",
            NoticeStatus::Closed => "关闭",
        }
    }
}

impl From<i32> for NoticeStatus {
    fn from(value: i32) -> Self {
        match value {
            1 => NoticeStatus::Closed,
            _ => NoticeStatus::Normal,
        }
    }
}

impl From<NoticeStatus> for i32 {
    fn from(value: NoticeStatus) -> Self {
        value as i32
    }
}

impl ActiveModelBehavior for ActiveModel {
    /// 在创建新记录前触发
    fn before_insert(model: sea_orm::ActiveModel<Self>) -> Result<Self, DbErr> {
        let mut model = model;
        if model.notice_id.as_ref() == &0 {
            model.notice_id = Set(chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0) as i64);
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
    #[sea_orm(name = "sys_notice_notice_id_fk")]
    SysNoticeNoticeIdFk,
}

impl Related<sea_orm::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SysNoticeNoticeIdFk.def()
    }
}

impl ActiveModel {
    /// 设置公告ID
    pub fn notice_id(mut self, notice_id: i64) -> Self {
        self.notice_id = Set(notice_id);
        self
    }

    /// 设置公告标题
    pub fn notice_title(mut self, notice_title: String) -> Self {
        self.notice_title = Set(notice_title);
        self
    }

    /// 设置公告类型
    pub fn notice_type(mut self, notice_type: NoticeType) -> Self {
        self.notice_type = Set(notice_type as i32);
        self
    }

    /// 设置公告内容
    pub fn notice_content(mut self, notice_content: String) -> Self {
        self.notice_content = Set(notice_content);
        self
    }

    /// 设置公告状态
    pub fn status(mut self, status: NoticeStatus) -> Self {
        self.status = Set(status as i32);
        self
    }

    /// 设置创建者
    pub fn create_by(mut self, create_by: String) -> Self {
        self.create_by = Set(create_by);
        self
    }

    /// 设置创建时间
    pub fn created_time(mut self, created_time: DateTime<Utc>) -> Self {
        self.created_time = Set(created_time);
        self
    }

    /// 设置更新者
    pub fn update_by(mut self, update_by: Option<String>) -> Self {
        self.update_by = Set(update_by);
        self
    }

    /// 设置更新时间
    pub fn updated_time(mut self, updated_time: DateTime<Utc>) -> Self {
        self.updated_time = Set(updated_time);
        self
    }

    /// 设置备注
    pub fn remark(mut self, remark: Option<String>) -> Self {
        self.remark = Set(remark);
        self
    }
}

impl Entity {
    /// 根据公告类型查找
    pub fn find_by_notice_type(notice_type: NoticeType) -> Select<Self> {
        Self::find().filter(Column::NoticeType.eq(notice_type as i32))
    }

    /// 根据状态查找
    pub fn find_by_status(status: NoticeStatus) -> Select<Self> {
        Self::find().filter(Column::Status.eq(status as i32))
    }

    /// 查找正常状态的公告
    pub fn find_normal() -> Select<Self> {
        Self::find().filter(Column::Status.eq(0))
    }

    /// 根据创建者查找
    pub fn find_by_create_by(create_by: &str) -> Select<Self> {
        Self::find().filter(Column::CreateBy.eq(create_by))
    }
}

impl Model {
    /// 获取公告类型
    pub fn get_notice_type(&self) -> NoticeType {
        NoticeType::from(self.notice_type)
    }

    /// 获取公告状态
    pub fn get_status(&self) -> NoticeStatus {
        NoticeStatus::from(self.status)
    }

    /// 检查是否为正常状态
    pub fn is_normal(&self) -> bool {
        self.status == 0
    }

    /// 检查是否为关闭状态
    pub fn is_closed(&self) -> bool {
        self.status == 1
    }

    /// 检查是否为通知
    pub fn is_notification(&self) -> bool {
        self.notice_type == 1
    }

    /// 检查是否为公告
    pub fn is_announcement(&self) -> bool {
        self.notice_type == 2
    }
}
