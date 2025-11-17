/// 系统监控指标实体

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
#[sea_orm(table_name = "sys_system_metric")]
pub struct Model {
    /// 指标ID
    #[sea_orm(primary_key, auto_increment = false)]
    pub metric_id: i64,
    /// 指标类型（1:CPU 2:内存 3:磁盘 4:网络）
    pub metric_type: i32,
    /// 指标名称
    pub metric_name: String,
    /// 指标值
    pub metric_value: f64,
    /// 指标单位
    pub unit: String,
    /// 主机名
    pub host_name: String,
    /// IP地址
    pub ip_address: String,
    /// 采集时间
    pub collection_time: DateTime<Utc>,
    /// 备注
    pub remark: Option<String>,
}

/// 指标类型枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MetricType {
    /// CPU
    Cpu = 1,
    /// 内存
    Memory = 2,
    /// 磁盘
    Disk = 3,
    /// 网络
    Network = 4,
}

impl MetricType {
    /// 获取类型名称
    pub fn get_name(&self) -> &'static str {
        match self {
            MetricType::Cpu => "CPU",
            MetricType::Memory => "内存",
            MetricType::Disk => "磁盘",
            MetricType::Network => "网络",
        }
    }
}

impl From<i32> for MetricType {
    fn from(value: i32) -> Self {
        match value {
            2 => MetricType::Memory,
            3 => MetricType::Disk,
            4 => MetricType::Network,
            _ => MetricType::Cpu,
        }
    }
}

impl From<MetricType> for i32 {
    fn from(value: MetricType) -> Self {
        value as i32
    }
}

impl ActiveModelBehavior for ActiveModel {
    /// 在创建新记录前触发
    fn before_insert(model: sea_orm::ActiveModel<Self>) -> Result<Self, DbErr> {
        let mut model = model;
        if model.metric_id.as_ref() == &0 {
            model.metric_id = Set(chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0) as i64);
        }
        if model.collection_time.as_ref() == &chrono::DateTime::default() {
            model.collection_time = Set(Utc::now());
        }
        Ok(model)
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
#[sea_orm()]
pub enum Relation {
    /// 无关联
    #[sea_orm(name = "sys_system_metric_metric_id_fk")]
    SysSystemMetricMetricIdFk,
}

impl Related<sea_orm::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SysSystemMetricMetricIdFk.def()
    }
}

impl ActiveModel {
    /// 设置指标ID
    pub fn metric_id(mut self, metric_id: i64) -> Self {
        self.metric_id = Set(metric_id);
        self
    }

    /// 设置指标类型
    pub fn metric_type(mut self, metric_type: MetricType) -> Self {
        self.metric_type = Set(metric_type as i32);
        self
    }

    /// 设置指标名称
    pub fn metric_name(mut self, metric_name: String) -> Self {
        self.metric_name = Set(metric_name);
        self
    }

    /// 设置指标值
    pub fn metric_value(mut self, metric_value: f64) -> Self {
        self.metric_value = Set(metric_value);
        self
    }

    /// 设置指标单位
    pub fn unit(mut self, unit: String) -> Self {
        self.unit = Set(unit);
        self
    }

    /// 设置主机名
    pub fn host_name(mut self, host_name: String) -> Self {
        self.host_name = Set(host_name);
        self
    }

    /// 设置IP地址
    pub fn ip_address(mut self, ip_address: String) -> Self {
        self.ip_address = Set(ip_address);
        self
    }

    /// 设置采集时间
    pub fn collection_time(mut self, collection_time: DateTime<Utc>) -> Self {
        self.collection_time = Set(collection_time);
        self
    }

    /// 设置备注
    pub fn remark(mut self, remark: Option<String>) -> Self {
        self.remark = Set(remark);
        self
    }
}

impl Entity {
    /// 根据指标类型查找
    pub fn find_by_metric_type(metric_type: MetricType) -> Select<Self> {
        Self::find().filter(Column::MetricType.eq(metric_type as i32))
    }

    /// 根据主机名查找
    pub fn find_by_host_name(host_name: &str) -> Select<Self> {
        Self::find().filter(Column::HostName.eq(host_name))
    }

    /// 根据时间范围查找
    pub fn find_by_time_range(
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> Select<Self> {
        Self::find()
            .filter(Column::CollectionTime.gte(start_time))
            .filter(Column::CollectionTime.lte(end_time))
    }

    /// 根据主机名和类型查找
    pub fn find_by_host_and_type(
        host_name: &str,
        metric_type: MetricType,
    ) -> Select<Self> {
        Self::find()
            .filter(Column::HostName.eq(host_name))
            .filter(Column::MetricType.eq(metric_type as i32))
    }

    /// 获取最新的指标
    pub fn find_latest_by_host(host_name: &str) -> Select<Self> {
        Self::find()
            .filter(Column::HostName.eq(host_name))
            .order_by(Column::CollectionTime, sea_orm::Order::Desc)
    }
}

impl Model {
    /// 获取指标类型
    pub fn get_metric_type(&self) -> MetricType {
        MetricType::from(self.metric_type)
    }

    /// 检查是否为CPU指标
    pub fn is_cpu(&self) -> bool {
        self.metric_type == 1
    }

    /// 检查是否为内存指标
    pub fn is_memory(&self) -> bool {
        self.metric_type == 2
    }

    /// 检查是否为磁盘指标
    pub fn is_disk(&self) -> bool {
        self.metric_type == 3
    }

    /// 检查是否为网络指标
    pub fn is_network(&self) -> bool {
        self.metric_type == 4
    }

    /// 获取指标值描述
    pub fn get_value_description(&self) -> String {
        format!("{}{}", self.metric_value, self.unit)
    }
}
