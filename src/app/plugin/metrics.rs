/// 插件性能监控模块
/// 收集和分析插件运行时性能指标

use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use serde::{Serialize, Deserialize};

/// 性能指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMetrics {
    /// 插件名称
    pub plugin_name: String,
    /// 总请求数
    pub total_requests: u64,
    /// 成功请求数
    pub successful_requests: u64,
    /// 失败请求数
    pub failed_requests: u64,
    /// 平均响应时间（毫秒）
    pub avg_response_time_ms: f64,
    /// 最小响应时间（毫秒）
    pub min_response_time_ms: f64,
    /// 最大响应时间（毫秒）
    pub max_response_time_ms: f64,
    /// P50响应时间（毫秒）
    pub p50_response_time_ms: f64,
    /// P95响应时间（毫秒）
    pub p95_response_time_ms: f64,
    /// P99响应时间（毫秒）
    pub p99_response_time_ms: f64,
    /// 内存使用（字节）
    pub memory_usage_bytes: usize,
    /// CPU使用率（百分比）
    pub cpu_usage_percent: f64,
    /// 错误率（百分比）
    pub error_rate_percent: f64,
    /// QPS（每秒查询数）
    pub qps: f64,
    /// 启动时间
    pub started_at: chrono::NaiveDateTime,
    /// 最后活动时间
    pub last_activity: chrono::NaiveDateTime,
}

/// 性能采样数据
#[derive(Debug, Clone)]
struct PerformanceSample {
    #[allow(dead_code)]
    timestamp: Instant,
    response_time: Duration,
    success: bool,
}

/// 性能监控器
pub struct PluginPerformanceMonitor {
    plugin_name: String,
    samples: Arc<RwLock<Vec<PerformanceSample>>>,
    metrics: Arc<RwLock<PluginMetrics>>,
    started_at: Instant,
}

impl PluginPerformanceMonitor {
    /// 创建新的监控器
    pub fn new(plugin_name: String) -> Self {
        let now = chrono::Utc::now().naive_utc();
        
        Self {
            plugin_name: plugin_name.clone(),
            samples: Arc::new(RwLock::new(Vec::new())),
            metrics: Arc::new(RwLock::new(PluginMetrics {
                plugin_name,
                total_requests: 0,
                successful_requests: 0,
                failed_requests: 0,
                avg_response_time_ms: 0.0,
                min_response_time_ms: 0.0,
                max_response_time_ms: 0.0,
                p50_response_time_ms: 0.0,
                p95_response_time_ms: 0.0,
                p99_response_time_ms: 0.0,
                memory_usage_bytes: 0,
                cpu_usage_percent: 0.0,
                error_rate_percent: 0.0,
                qps: 0.0,
                started_at: now,
                last_activity: now,
            })),
            started_at: Instant::now(),
        }
    }

    /// 记录请求
    pub async fn record_request(&self, response_time: Duration, success: bool) {
        let sample = PerformanceSample {
            timestamp: Instant::now(),
            response_time,
            success,
        };

        {
            let mut samples = self.samples.write().await;
            samples.push(sample);

            // 限制样本数量，保留最近1000个
            let len = samples.len();
            if len > 1000 {
                samples.drain(0..len - 1000);
            }
        } // 释放写锁

        // 更新指标
        self.update_metrics().await;
    }

    /// 更新性能指标
    async fn update_metrics(&self) {
        let samples = self.samples.read().await;
        let mut metrics = self.metrics.write().await;

        if samples.is_empty() {
            return;
        }

        // 统计请求数
        metrics.total_requests = samples.len() as u64;
        metrics.successful_requests = samples.iter().filter(|s| s.success).count() as u64;
        metrics.failed_requests = samples.iter().filter(|s| !s.success).count() as u64;

        // 计算响应时间统计
        let mut response_times: Vec<f64> = samples
            .iter()
            .map(|s| s.response_time.as_secs_f64() * 1000.0)
            .collect();
        
        response_times.sort_by(|a, b| a.partial_cmp(b).unwrap());

        metrics.min_response_time_ms = response_times.first().copied().unwrap_or(0.0);
        metrics.max_response_time_ms = response_times.last().copied().unwrap_or(0.0);
        metrics.avg_response_time_ms = response_times.iter().sum::<f64>() / response_times.len() as f64;

        // 计算百分位数
        let len = response_times.len();
        metrics.p50_response_time_ms = response_times[len * 50 / 100];
        metrics.p95_response_time_ms = response_times[len * 95 / 100];
        metrics.p99_response_time_ms = response_times[len * 99 / 100];

        // 计算错误率
        metrics.error_rate_percent = if metrics.total_requests > 0 {
            (metrics.failed_requests as f64 / metrics.total_requests as f64) * 100.0
        } else {
            0.0
        };

        // 计算QPS
        let elapsed = self.started_at.elapsed().as_secs_f64();
        metrics.qps = if elapsed > 0.0 {
            metrics.total_requests as f64 / elapsed
        } else {
            0.0
        };

        metrics.last_activity = chrono::Utc::now().naive_utc();
    }

    /// 获取当前指标
    pub async fn get_metrics(&self) -> PluginMetrics {
        self.metrics.read().await.clone()
    }

    /// 重置指标
    pub async fn reset(&self) {
        let mut samples = self.samples.write().await;
        samples.clear();

        let mut metrics = self.metrics.write().await;
        let now = chrono::Utc::now().naive_utc();
        
        *metrics = PluginMetrics {
            plugin_name: self.plugin_name.clone(),
            total_requests: 0,
            successful_requests: 0,
            failed_requests: 0,
            avg_response_time_ms: 0.0,
            min_response_time_ms: 0.0,
            max_response_time_ms: 0.0,
            p50_response_time_ms: 0.0,
            p95_response_time_ms: 0.0,
            p99_response_time_ms: 0.0,
            memory_usage_bytes: 0,
            cpu_usage_percent: 0.0,
            error_rate_percent: 0.0,
            qps: 0.0,
            started_at: now,
            last_activity: now,
        };
    }

    /// 记录内存使用
    pub async fn record_memory_usage(&self, bytes: usize) {
        let mut metrics = self.metrics.write().await;
        metrics.memory_usage_bytes = bytes;
    }

    /// 记录CPU使用
    pub async fn record_cpu_usage(&self, percent: f64) {
        let mut metrics = self.metrics.write().await;
        metrics.cpu_usage_percent = percent;
    }
}

/// 性能监控管理器
pub struct MetricsManager {
    monitors: Arc<RwLock<HashMap<String, Arc<PluginPerformanceMonitor>>>>,
}

impl MetricsManager {
    /// 创建新的管理器
    pub fn new() -> Self {
        Self {
            monitors: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// 为插件创建监控器
    pub async fn create_monitor(&self, plugin_name: String) -> Arc<PluginPerformanceMonitor> {
        let monitor = Arc::new(PluginPerformanceMonitor::new(plugin_name.clone()));
        
        let mut monitors = self.monitors.write().await;
        monitors.insert(plugin_name, monitor.clone());
        
        monitor
    }

    /// 获取插件监控器
    pub async fn get_monitor(&self, plugin_name: &str) -> Option<Arc<PluginPerformanceMonitor>> {
        let monitors = self.monitors.read().await;
        monitors.get(plugin_name).cloned()
    }

    /// 移除插件监控器
    pub async fn remove_monitor(&self, plugin_name: &str) {
        let mut monitors = self.monitors.write().await;
        monitors.remove(plugin_name);
        tracing::info!("Removed metrics monitor for plugin {}", plugin_name);
    }

    /// 获取所有插件的指标
    pub async fn get_all_metrics(&self) -> Vec<PluginMetrics> {
        let monitors = self.monitors.read().await;
        let mut metrics = Vec::new();

        for monitor in monitors.values() {
            metrics.push(monitor.get_metrics().await);
        }

        metrics
    }

    /// 导出指标为JSON
    pub async fn export_metrics_json(&self) -> Result<String, serde_json::Error> {
        let metrics = self.get_all_metrics().await;
        serde_json::to_string_pretty(&metrics)
    }
}

impl Default for MetricsManager {
    fn default() -> Self {
        Self::new()
    }
}

/// 性能计时器
/// 用于简化请求时间测量
pub struct RequestTimer {
    start: Instant,
    monitor: Arc<PluginPerformanceMonitor>,
}

impl RequestTimer {
    /// 开始计时
    pub fn start(monitor: Arc<PluginPerformanceMonitor>) -> Self {
        Self {
            start: Instant::now(),
            monitor,
        }
    }

    /// 结束计时并记录
    pub async fn finish(self, success: bool) {
        let duration = self.start.elapsed();
        self.monitor.record_request(duration, success).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test_performance_monitor() {
        let monitor = PluginPerformanceMonitor::new("test_plugin".to_string());
        
        monitor.record_request(Duration::from_millis(100), true).await;
        monitor.record_request(Duration::from_millis(200), true).await;
        monitor.record_request(Duration::from_millis(50), false).await;
        
        let metrics = monitor.get_metrics().await;
        assert_eq!(metrics.total_requests, 3);
        assert_eq!(metrics.successful_requests, 2);
        assert_eq!(metrics.failed_requests, 1);
    }

    #[tokio::test]
    async fn test_metrics_manager() {
        let manager = MetricsManager::new();
        
        let monitor = manager.create_monitor("test_plugin".to_string()).await;
        monitor.record_request(Duration::from_millis(100), true).await;
        
        let metrics = manager.get_all_metrics().await;
        assert_eq!(metrics.len(), 1);
        assert_eq!(metrics[0].total_requests, 1);
    }
}
