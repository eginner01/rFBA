//! 文件信息实体 - sys_file_info表

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
#[sea_orm(table_name = "sys_file_info")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub file_id: i64,
    pub file_name: String,
    pub original_name: String,
    pub file_suffix: String,
    pub file_size: i64,
    pub content_type: String,
    pub file_path: String,
    pub storage_type: i32,
    pub file_hash: Option<String>,
    pub uploader: String,
    pub access_permission: i32,
    pub download_count: i32,
    pub is_deleted: i32,
    pub upload_time: DateTime<Utc>,
    pub updated_time: DateTime<Utc>,
    pub remark: Option<String>,
}

/// 存储类型枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StorageType {
    /// 本地存储
    Local = 1,
    /// 阿里云OSS
    Oss = 2,
    /// 亚马逊S3
    S3 = 3,
}

impl StorageType {
    /// 获取类型名称
    pub fn get_name(&self) -> &'static str {
        match self {
            StorageType::Local => "本地存储",
            StorageType::Oss => "阿里云OSS",
            StorageType::S3 => "亚马逊S3",
        }
    }
}

impl From<i32> for StorageType {
    fn from(value: i32) -> Self {
        match value {
            2 => StorageType::Oss,
            3 => StorageType::S3,
            _ => StorageType::Local,
        }
    }
}

impl From<StorageType> for i32 {
    fn from(value: StorageType) -> Self {
        value as i32
    }
}

/// 访问权限枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccessPermission {
    /// 私有
    Private = 1,
    /// 公开
    Public = 2,
    /// 组织内
    Organization = 3,
}

impl AccessPermission {
    /// 获取权限名称
    pub fn get_name(&self) -> &'static str {
        match self {
            AccessPermission::Private => "私有",
            AccessPermission::Public => "公开",
            AccessPermission::Organization => "组织内",
        }
    }
}

impl From<i32> for AccessPermission {
    fn from(value: i32) -> Self {
        match value {
            2 => AccessPermission::Public,
            3 => AccessPermission::Organization,
            _ => AccessPermission::Private,
        }
    }
}

impl From<AccessPermission> for i32 {
    fn from(value: AccessPermission) -> Self {
        value as i32
    }
}

impl ActiveModelBehavior for ActiveModel {
    /// 在创建新记录前触发
    fn before_insert(model: sea_orm::ActiveModel<Self>) -> Result<Self, DbErr> {
        let mut model = model;
        if model.file_id.as_ref() == &0 {
            model.file_id = Set(chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0) as i64);
        }
        if model.upload_time.as_ref() == &chrono::DateTime::default() {
            model.upload_time = Set(Utc::now());
        }
        if model.updated_time.as_ref() == &chrono::DateTime::default() {
            model.updated_time = Set(Utc::now());
        }
        if model.download_count.as_ref() == &0 {
            model.download_count = Set(0);
        }
        if model.is_deleted.as_ref() == &0 {
            model.is_deleted = Set(0);
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
    #[sea_orm(name = "sys_file_info_file_id_fk")]
    SysFileInfoFileIdFk,
}

impl Related<sea_orm::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SysFileInfoFileIdFk.def()
    }
}

impl ActiveModel {
    /// 设置文件ID
    pub fn file_id(mut self, file_id: i64) -> Self {
        self.file_id = Set(file_id);
        self
    }

    /// 设置文件名
    pub fn file_name(mut self, file_name: String) -> Self {
        self.file_name = Set(file_name);
        self
    }

    /// 设置原始文件名
    pub fn original_name(mut self, original_name: String) -> Self {
        self.original_name = Set(original_name);
        self
    }

    /// 设置文件后缀
    pub fn file_suffix(mut self, file_suffix: String) -> Self {
        self.file_suffix = Set(file_suffix);
        self
    }

    /// 设置文件大小
    pub fn file_size(mut self, file_size: i64) -> Self {
        self.file_size = Set(file_size);
        self
    }

    /// 设置MIME类型
    pub fn content_type(mut self, content_type: String) -> Self {
        self.content_type = Set(content_type);
        self
    }

    /// 设置文件路径
    pub fn file_path(mut self, file_path: String) -> Self {
        self.file_path = Set(file_path);
        self
    }

    /// 设置存储类型
    pub fn storage_type(mut self, storage_type: StorageType) -> Self {
        self.storage_type = Set(storage_type as i32);
        self
    }

    /// 设置文件哈希
    pub fn file_hash(mut self, file_hash: Option<String>) -> Self {
        self.file_hash = Set(file_hash);
        self
    }

    /// 设置上传者
    pub fn uploader(mut self, uploader: String) -> Self {
        self.uploader = Set(uploader);
        self
    }

    /// 设置访问权限
    pub fn access_permission(mut self, access_permission: AccessPermission) -> Self {
        self.access_permission = Set(access_permission as i32);
        self
    }

    /// 设置下载次数
    pub fn download_count(mut self, download_count: i32) -> Self {
        self.download_count = Set(download_count);
        self
    }

    /// 设置是否删除
    pub fn is_deleted(mut self, is_deleted: i32) -> Self {
        self.is_deleted = Set(is_deleted);
        self
    }

    /// 设置上传时间
    pub fn upload_time(mut self, upload_time: DateTime<Utc>) -> Self {
        self.upload_time = Set(upload_time);
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
    /// 根据上传者查找
    pub fn find_by_uploader(uploader: &str) -> Select<Self> {
        Self::find().filter(Column::Uploader.eq(uploader))
    }

    /// 根据存储类型查找
    pub fn find_by_storage_type(storage_type: StorageType) -> Select<Self> {
        Self::find().filter(Column::StorageType.eq(storage_type as i32))
    }

    /// 根据访问权限查找
    pub fn find_by_access_permission(access_permission: AccessPermission) -> Select<Self> {
        Self::find().filter(Column::AccessPermission.eq(access_permission as i32))
    }

    /// 查找未删除的文件
    pub fn find_not_deleted() -> Select<Self> {
        Self::find().filter(Column::IsDeleted.eq(0))
    }

    /// 根据文件后缀查找
    pub fn find_by_suffix(file_suffix: &str) -> Select<Self> {
        Self::find().filter(Column::FileSuffix.eq(file_suffix))
    }

    /// 根据上传者查找未删除文件
    pub fn find_by_uploader_not_deleted(uploader: &str) -> Select<Self> {
        Self::find_by_uploader(uploader)
            .filter(Column::IsDeleted.eq(0))
    }

    /// 按上传时间倒序查找
    pub fn find_order_by_upload_time() -> Select<Self> {
        Self::find_not_deleted().order_by(Column::UploadTime, sea_orm::Order::Desc)
    }

    /// 按下载次数倒序查找
    pub fn find_order_by_download_count() -> Select<Self> {
        Self::find_not_deleted().order_by(Column::DownloadCount, sea_orm::Order::Desc)
    }
}

impl Model {
    /// 获取存储类型
    pub fn get_storage_type(&self) -> StorageType {
        StorageType::from(self.storage_type)
    }

    /// 获取访问权限
    pub fn get_access_permission(&self) -> AccessPermission {
        AccessPermission::from(self.access_permission)
    }

    /// 检查是否为私有文件
    pub fn is_private(&self) -> bool {
        self.access_permission == 1
    }

    /// 检查是否为公开文件
    pub fn is_public(&self) -> bool {
        self.access_permission == 2
    }

    /// 检查是否为组织内文件
    pub fn is_organization(&self) -> bool {
        self.access_permission == 3
    }

    /// 检查是否已删除
    pub fn is_deleted(&self) -> bool {
        self.is_deleted == 1
    }

    /// 获取文件大小描述
    pub fn get_size_description(&self) -> String {
        if self.file_size < 1024 {
            format!("{} B", self.file_size)
        } else if self.file_size < 1024 * 1024 {
            format!("{:.2} KB", self.file_size as f64 / 1024.0)
        } else if self.file_size < 1024 * 1024 * 1024 {
            format!("{:.2} MB", self.file_size as f64 / (1024.0 * 1024.0))
        } else {
            format!(
                "{:.2} GB",
                self.file_size as f64 / (1024.0 * 1024.0 * 1024.0)
            )
        }
    }
}
