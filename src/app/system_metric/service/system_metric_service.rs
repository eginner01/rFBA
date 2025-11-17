use tracing::{info, warn, error, debug};

/// 系统监控指标服务实现
/// 提供系统指标的收集、查询、统计等功能

use crate::app::system_metric::dto::{
    CreateSystemMetricRequest, BatchCreateSystemMetricRequest,
    BatchCreateSystemMetricResponse, SystemMetricQuery, SystemMetricListResponse,
    SystemMetricListItem, SystemMetricDetailResponse, RealTimeMetricQuery,
    RealTimeMetricResponse, MetricHistoryQuery, MetricHistoryResponse,
    MetricHistoryPoint, MetricStatisticsQuery, MetricStatisticsResponse,
    MetricStatisticsData, MetricTypeStatistics, HostOverview,
};
use crate::common::exception::{AppError, ErrorCode};
use crate::database::DatabaseManager;
use crate::database::entity::system_metric::{self, MetricType};
use sea_orm::{DatabaseConnection, DbErr, EntityTrait, QueryFilter, ColumnTrait, Select, Order};
use std::collections::HashMap;

/// 系统监控指标服务
pub struct SystemMetricService {
    db: DatabaseConnection,
}

impl SystemMetricService {
    /// 创建新的系统监控指标服务
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// 创建系统指标
    pub async fn create_system_metric(
        &self,
        request: &CreateSystemMetricRequest,
    ) -> Result<(), AppError> {
        let active_model = system_metric::ActiveModel {
            metric_id: Default::default(),
            metric_type: sea_orm::Set(request.metric_type),
            metric_name: sea_orm::Set(request.metric_name.clone()),
            metric_value: sea_orm::Set(request.metric_value),
            unit: sea_orm::Set(request.unit.clone()),
            host_name: sea_orm::Set(request.host_name.clone()),
            ip_address: sea_orm::Set(request.ip_address.clone()),
            collection_time: Default::default(),
            remark: sea_orm::Set(request.remark.clone()),
        };

        active_model.insert(&self.db).await.map_err(|e| {
            error!("Failed to create system metric: {:?}", e);
            AppError::new(ErrorCode::DatabaseError, "Failed to create system metric")
        })?;

        Ok(())
    }

    /// 批量创建系统指标
    pub async fn batch_create_system_metrics(
        &self,
        request: &BatchCreateSystemMetricRequest,
    ) -> Result<BatchCreateSystemMetricResponse, AppError> {
        let mut success_count = 0;
        let mut failed_metrics = Vec::new();

        for metric_request in &request.metrics {
            let active_model = system_metric::ActiveModel {
                metric_id: Default::default(),
                metric_type: sea_orm::Set(metric_request.metric_type),
                metric_name: sea_orm::Set(metric_request.metric_name.clone()),
                metric_value: sea_orm::Set(metric_request.metric_value),
                unit: sea_orm::Set(metric_request.unit.clone()),
                host_name: sea_orm::Set(metric_request.host_name.clone()),
                ip_address: sea_orm::Set(metric_request.ip_address.clone()),
                collection_time: Default::default(),
                remark: sea_orm::Set(metric_request.remark.clone()),
            };

            match active_model.insert(&self.db).await {
                Ok(_) => success_count += 1,
                Err(e) => {
                    error!("Failed to create system metric: {:?}", e);
                    failed_metrics.push(metric_request.metric_name.clone());
                }
            }
        }

        Ok(BatchCreateSystemMetricResponse {
            success_count,
            failed_metrics,
        })
    }

    /// 获取指标列表（分页）
    pub async fn get_system_metric_list(
        &self,
        query: &SystemMetricQuery,
    ) -> Result<SystemMetricListResponse, AppError> {
        let mut select = system_metric::Entity::find();

        // 添加查询条件
        if let Some(metric_type) = query.metric_type {
            select = select.filter(system_metric::Column::MetricType.eq(metric_type));
        }

        if let Some(metric_name) = &query.metric_name {
            select = select.filter(system_metric::Column::MetricName.like(format!(
                "%{}%",
                metric_name
            )));
        }

        if let Some(host_name) = &query.host_name {
            select = select.filter(system_metric::Column::HostName.like(format!(
                "%{}%",
                host_name
            )));
        }

        if let Some(ip_address) = &query.ip_address {
            select = select.filter(system_metric::Column::IpAddress.like(format!(
                "%{}%",
                ip_address
            )));
        }

        if let Some(start_time) = query.start_time {
            select = select.filter(system_metric::Column::CollectionTime.gte(start_time));
        }

        if let Some(end_time) = query.end_time {
            select = select.filter(system_metric::Column::CollectionTime.lte(end_time));
        }

        // 按采集时间倒序
        select = select.order_by(system_metric::Column::CollectionTime, Order::Desc);

        // 分页
        let page_size = query.page_size.unwrap_or(20);
        let page_num = query.page_num.unwrap_or(1);
        let offset = (page_num - 1) * page_size;

        let metrics = select
            .offset(offset as u64)
            .limit(page_size as u64)
            .all(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to query system metrics: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to query system metrics")
            })?;

        let total = system_metric::Entity::find()
            .count(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to count system metrics: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to count system metrics")
            })?;

        let list = metrics
            .into_iter()
            .map(|m| {
                let metric_type_name = match MetricType::from(m.metric_type) {
                    MetricType::Cpu => "CPU",
                    MetricType::Memory => "内存",
                    MetricType::Disk => "磁盘",
                    MetricType::Network => "网络",
                };

                let value_description = format!("{}{}", m.metric_value, m.unit);

                SystemMetricListItem {
                    metric_id: m.metric_id,
                    metric_type: m.metric_type,
                    metric_type_name: metric_type_name.to_string(),
                    metric_name: m.metric_name,
                    metric_value: m.metric_value,
                    unit: m.unit.clone(),
                    value_description,
                    host_name: m.host_name,
                    ip_address: m.ip_address,
                    collection_time: m.collection_time,
                    remark: m.remark,
                }
            })
            .collect();

        Ok(SystemMetricListResponse {
            list,
            total: total as usize,
            page_num,
            page_size,
        })
    }

    /// 获取指标详情
    pub async fn get_system_metric_detail(
        &self,
        metric_id: i64,
    ) -> Result<SystemMetricDetailResponse, AppError> {
        let m = system_metric::Entity::find_by_id(metric_id)
            .one(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to find system metric: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to find system metric")
            })?
            .ok_or_else(|| AppError::new(ErrorCode::NotFound, "System metric not found"))?;

        let metric_type_name = match MetricType::from(m.metric_type) {
            MetricType::Cpu => "CPU",
            MetricType::Memory => "内存",
            MetricType::Disk => "磁盘",
            MetricType::Network => "网络",
        };

        let value_description = format!("{}{}", m.metric_value, m.unit);

        Ok(SystemMetricDetailResponse {
            metric_id: m.metric_id,
            metric_type: m.metric_type,
            metric_type_name: metric_type_name.to_string(),
            metric_name: m.metric_name,
            metric_value: m.metric_value,
            unit: m.unit.clone(),
            value_description,
            host_name: m.host_name,
            ip_address: m.ip_address,
            collection_time: m.collection_time,
            remark: m.remark,
        })
    }

    /// 获取实时指标
    pub async fn get_real_time_metrics(
        &self,
        query: &RealTimeMetricQuery,
    ) -> Result<RealTimeMetricResponse, AppError> {
        let mut select = system_metric::Entity::find();

        if let Some(host_name) = &query.host_name {
            select = select.filter(system_metric::Column::HostName.eq(host_name));
        }

        if let Some(metric_types) = &query.metric_types {
            select = select.filter(system_metric::Column::MetricType.is_in(metric_types.clone()));
        }

        // 获取最新数据
        select = select
            .order_by(system_metric::Column::CollectionTime, Order::Desc)
            .limit(50);

        let metrics = select
            .all(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to query real time metrics: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to query real time metrics")
            })?;

        let mut host_name = "unknown".to_string();
        let mut ip_address = "unknown".to_string();

        let list = metrics
            .into_iter()
            .map(|m| {
                host_name = m.host_name.clone();
                ip_address = m.ip_address.clone();

                let metric_type_name = match MetricType::from(m.metric_type) {
                    MetricType::Cpu => "CPU",
                    MetricType::Memory => "内存",
                    MetricType::Disk => "磁盘",
                    MetricType::Network => "网络",
                };

                let value_description = format!("{}{}", m.metric_value, m.unit);

                SystemMetricListItem {
                    metric_id: m.metric_id,
                    metric_type: m.metric_type,
                    metric_type_name: metric_type_name.to_string(),
                    metric_name: m.metric_name,
                    metric_value: m.metric_value,
                    unit: m.unit.clone(),
                    value_description,
                    host_name: m.host_name,
                    ip_address: m.ip_address,
                    collection_time: m.collection_time,
                    remark: m.remark,
                }
            })
            .collect();

        Ok(RealTimeMetricResponse {
            host_name,
            ip_address,
            metrics: list,
        })
    }

    /// 获取指标历史数据
    pub async fn get_metric_history(
        &self,
        query: &MetricHistoryQuery,
    ) -> Result<MetricHistoryResponse, AppError> {
        let end_time = query.end_time.unwrap_or_else(|| chrono::Utc::now());
        let start_time = query
            .start_time
            .unwrap_or(end_time - chrono::Duration::hours(24));

        let mut select = system_metric::Entity::find()
            .filter(system_metric::Column::HostName.eq(&query.host_name))
            .filter(system_metric::Column::MetricName.eq(&query.metric_name))
            .filter(system_metric::Column::CollectionTime.gte(start_time))
            .filter(system_metric::Column::CollectionTime.lte(end_time))
            .order_by(system_metric::Column::CollectionTime, Order::Asc);

        let metrics = select
            .all(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to query metric history: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to query metric history")
            })?;

        let history = metrics
            .into_iter()
            .map(|m| MetricHistoryPoint {
                time: m.collection_time,
                value: m.metric_value,
            })
            .collect();

        // 获取指标类型和单位（从最新记录）
        let latest_metric = system_metric::Entity::find()
            .filter(system_metric::Column::HostName.eq(&query.host_name))
            .filter(system_metric::Column::MetricName.eq(&query.metric_name))
            .order_by(system_metric::Column::CollectionTime, Order::Desc)
            .one(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to query latest metric: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to query latest metric")
            })?;

        let (metric_type, unit) = if let Some(m) = latest_metric {
            (m.metric_type, m.unit)
        } else {
            return Err(AppError::new(ErrorCode::NotFound, "Metric not found"));
        };

        Ok(MetricHistoryResponse {
            host_name: query.host_name.clone(),
            metric_name: query.metric_name.clone(),
            metric_type,
            unit,
            history,
        })
    }

    /// 获取指标统计
    pub async fn get_metric_statistics(
        &self,
        query: &MetricStatisticsQuery,
    ) -> Result<MetricStatisticsResponse, AppError> {
        let end_time = chrono::Utc::now();
        let start_time = end_time - chrono::Duration::hours(query.time_range_hours as i64);

        let mut select = system_metric::Entity::find()
            .filter(system_metric::Column::MetricType.eq(query.metric_type));

        if let Some(host_name) = &query.host_name {
            select = select.filter(system_metric::Column::HostName.eq(host_name));
        }

        select = select
            .filter(system_metric::Column::CollectionTime.gte(start_time))
            .filter(system_metric::Column::CollectionTime.lte(end_time))
            .order_by(system_metric::Column::CollectionTime, Order::Asc);

        let metrics = select
            .all(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to query metric statistics: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to query metric statistics")
            })?;

        if metrics.is_empty() {
            return Err(AppError::new(
                ErrorCode::NotFound,
                "No metrics found for statistics",
            ));
        }

        let mut min_value = f64::MAX;
        let mut max_value = f64::MIN;
        let mut sum_value = 0.0;
        let current_value = metrics.last().unwrap().metric_value;

        for metric in &metrics {
            min_value = min_value.min(metric.metric_value);
            max_value = max_value.max(metric.metric_value);
            sum_value += metric.metric_value;
        }

        let avg_value = sum_value / metrics.len() as f64;

        let metric_type_name = match MetricType::from(query.metric_type) {
            MetricType::Cpu => "CPU",
            MetricType::Memory => "内存",
            MetricType::Disk => "磁盘",
            MetricType::Network => "网络",
        };

        let statistics = MetricStatisticsData {
            min_value,
            max_value,
            avg_value,
            current_value,
            data_count: metrics.len(),
        };

        Ok(MetricStatisticsResponse {
            host_name: query.host_name.clone().unwrap_or_else(|| "all".to_string()),
            metric_type: query.metric_type,
            metric_type_name: metric_type_name.to_string(),
            time_range_hours: query.time_range_hours,
            statistics,
        })
    }

    /// 获取指标类型统计
    pub async fn get_metric_type_statistics(&self) -> Result<Vec<MetricTypeStatistics>, AppError> {
        let metrics = system_metric::Entity::find()
            .all(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to query system metrics: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to query system metrics")
            })?;

        let mut stats_map: HashMap<i32, (usize, usize)> = HashMap::new();
        let mut host_set = std::collections::HashSet::new();

        for metric in metrics {
            let entry = stats_map
                .entry(metric.metric_type)
                .or_insert((0, 0));
            entry.0 += 1; // metric count
            host_set.insert(metric.host_name);
            entry.1 = host_set.len(); // unique host count
        }

        let mut stats = Vec::new();
        for (metric_type, (count, host_count)) in stats_map {
            let metric_type_name = match MetricType::from(metric_type) {
                MetricType::Cpu => "CPU",
                MetricType::Memory => "内存",
                MetricType::Disk => "磁盘",
                MetricType::Network => "网络",
            };

            stats.push(MetricTypeStatistics {
                metric_type,
                metric_type_name: metric_type_name.to_string(),
                count,
                host_count,
            });
        }

        Ok(stats)
    }

    /// 获取主机概览
    pub async fn get_host_overview(&self) -> Result<Vec<HostOverview>, AppError> {
        let metrics = system_metric::Entity::find()
            .order_by(system_metric::Column::CollectionTime, Order::Desc)
            .all(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to query system metrics: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to query system metrics")
            })?;

        let mut host_map: HashMap<String, HostOverview> = HashMap::new();

        for metric in metrics {
            let host_name = metric.host_name.clone();
            let overview = host_map
                .entry(host_name.clone())
                .or_insert(HostOverview {
                    host_name: host_name.clone(),
                    ip_address: metric.ip_address.clone(),
                    last_collection_time: metric.collection_time,
                    cpu_usage: None,
                    memory_usage: None,
                    disk_usage: None,
                });

            // 更新各类型指标的最新值
            match MetricType::from(metric.metric_type) {
                MetricType::Cpu => overview.cpu_usage = Some(metric.metric_value),
                MetricType::Memory => overview.memory_usage = Some(metric.metric_value),
                MetricType::Disk => overview.disk_usage = Some(metric.metric_value),
                MetricType::Network => {}
            }

            // 更新最后采集时间
            if metric.collection_time > overview.last_collection_time {
                overview.last_collection_time = metric.collection_time;
            }
        }

        let mut result: Vec<HostOverview> = host_map.into_values().collect();
        result.sort_by(|a, b| b.last_collection_time.cmp(&a.last_collection_time));

        Ok(result)
    }

    /// 清理过期数据
    pub async fn cleanup_old_metrics(&self, days: i32) -> Result<usize, AppError> {
        let cutoff_time =
            chrono::Utc::now() - chrono::Duration::days(days as i64);

        let delete_result = system_metric::Entity::delete_many()
            .filter(system_metric::Column::CollectionTime.lt(cutoff_time))
            .exec(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to cleanup old metrics: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to cleanup old metrics")
            })?;

        info!("Cleaned up {} old metrics", delete_result.rows_affected);

        Ok(delete_result.rows_affected as usize)
    }
}
