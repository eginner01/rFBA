use tracing::{info, error};
use crate::app::monitor::dto::{*, TokenExtraInfo};
use crate::common::exception::{AppError, ErrorCode};
use chrono::Utc;
use std::time::SystemTime;
use redis::Client as RedisClient;
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, PaginatorTrait};
use crate::database::entity::{opera_log, login_log};
use sysinfo::{System, Disks, Pid};

/// 监控服务实现
/// 提供系统监控、性能指标收集、健康检查等功能

pub struct MonitorService {
    pub redis_client: Option<RedisClient>,
}

impl MonitorService {
    /// 创建新的监控服务
    pub fn new(redis_client: Option<RedisClient>) -> Self {
        Self { redis_client }
    }

    /// 获取数据库连接（使用全局连接）
    async fn get_db() -> DatabaseConnection {
        crate::database::DatabaseManager::get_connection().await.clone()
    }

    /// 获取系统状态
    pub async fn get_system_status(&self) -> Result<SystemStatus, AppError> {
        let uptime = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Ok(SystemStatus {
            status: "running".to_string(),
            uptime_seconds: uptime as i64,
            timestamp: Utc::now(),
        })
    }

    /// 获取服务器监控信息
    pub async fn get_system_metrics(&self) -> Result<ServerMetrics, AppError> {
        use std::env;
        
        // 使用 sysinfo 获取真实的系统指标
        let mut sys = System::new_all();
        sys.refresh_all();

        // CPU 信息
        let cpu_usage = if sys.cpus().is_empty() {
            0.0
        } else {
            let total_cpu: f32 = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).sum();
            (total_cpu / sys.cpus().len() as f32) as f64
        };
        
        let cpu = CpuInfo {
            usage: (cpu_usage * 100.0).round() / 100.0,
            logical_num: sys.cpus().len(),
            physical_num: sys.physical_core_count().unwrap_or(sys.cpus().len()),
            max_freq: 0.0,  // sysinfo 不提供此信息
            min_freq: 0.0,
            current_freq: sys.cpus().first().map(|cpu| cpu.frequency() as f64).unwrap_or(0.0),
        };

        // 内存信息（转换为 GB）
        let total_memory_gb = sys.total_memory() as f64 / (1024.0 * 1024.0 * 1024.0);
        let used_memory_gb = sys.used_memory() as f64 / (1024.0 * 1024.0 * 1024.0);
        let free_memory_gb = sys.available_memory() as f64 / (1024.0 * 1024.0 * 1024.0);
        let memory_usage = if sys.total_memory() > 0 {
            (sys.used_memory() as f64 / sys.total_memory() as f64) * 100.0
        } else {
            0.0
        };

        let mem = MemoryInfo {
            total: (total_memory_gb * 100.0).round() / 100.0,
            used: (used_memory_gb * 100.0).round() / 100.0,
            free: (free_memory_gb * 100.0).round() / 100.0,
            usage: (memory_usage * 100.0).round() / 100.0,
        };

        // 系统信息
        let hostname = Self::get_hostname();
        
        // 获取本机 IP 地址
        let local_ip = Self::get_local_ip().unwrap_or_else(|| "127.0.0.1".to_string());
        
        let sys_info = SystemInfo {
            name: hostname,
            ip: local_ip,
            os: Self::get_os_name(),
            arch: env::consts::ARCH.to_string(),
        };

        // 磁盘信息
        let disks = Disks::new_with_refreshed_list();
        let disk_info: Vec<DiskInfo> = disks.iter().map(|disk| {
            let total = disk.total_space();
            let available = disk.available_space();
            // 使用 saturating_sub 避免下溢 panic（某些虚拟文件系统可能 available > total）
            let used = total.saturating_sub(available);
            let usage = if total > 0 {
                (used as f64 / total as f64) * 100.0
            } else {
                0.0
            };

            DiskInfo {
                dir: disk.mount_point().to_string_lossy().to_string(),
                disk_type: format!("{:?}", disk.kind()),
                device: disk.name().to_string_lossy().to_string(),
                total: Self::format_bytes(total as f64),
                free: Self::format_bytes(available as f64),
                used: Self::format_bytes(used as f64),
                usage: format!("{:.2}%", usage),
            }
        }).collect();

        // 服务信息
        let current_pid = Pid::from_u32(std::process::id());
        let process = sys.process(current_pid);
        let (cpu_usage_str, mem_vms_str, mem_rss_str, startup_str, elapsed_str) = if let Some(proc) = process {
            let cpu_usage = format!("{:.2}%", proc.cpu_usage());
            let mem_vms = Self::format_bytes(proc.virtual_memory() as f64);
            let mem_rss = Self::format_bytes(proc.memory() as f64);
            // 使用 saturating_sub 避免下溢 panic
            let mem_free_val = proc.virtual_memory().saturating_sub(proc.memory());
            let mem_free = Self::format_bytes(mem_free_val as f64);
            let start_time = proc.start_time();
            let startup = chrono::DateTime::from_timestamp(start_time as i64, 0)
                .unwrap_or_else(Utc::now)
                .format("%Y-%m-%d %H:%M:%S")
                .to_string();
            let elapsed = Self::format_duration(Utc::now().timestamp() - start_time as i64);
            
            (cpu_usage, mem_vms, mem_rss, startup, elapsed)
        } else {
            ("0.00%".to_string(), "0 B".to_string(), "0 B".to_string(), 
             Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(), "0 秒".to_string())
        };

        let service = ServiceInfo {
            name: "Rust".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            home: env::current_exe()
                .ok()
                .and_then(|p| p.to_str().map(|s| s.to_string()))
                .unwrap_or_else(|| "unknown".to_string()),
            cpu_usage: cpu_usage_str,
            mem_vms: mem_vms_str,
            mem_rss: mem_rss_str.clone(),
            mem_free: "0 B".to_string(), // 简化
            startup: startup_str,
            elapsed: elapsed_str,
        };

        info!("获取服务器监控信息成功");

        Ok(ServerMetrics {
            cpu,
            mem,
            sys: sys_info,
            disk: disk_info,
            service,
        })
    }

    /// 格式化字节大小
    fn format_bytes(size: f64) -> String {
        let units = ["B", "KB", "MB", "GB", "TB", "PB"];
        let mut size = size;
        let mut unit_index = 0;
        
        while size >= 1024.0 && unit_index < units.len() - 1 {
            size /= 1024.0;
            unit_index += 1;
        }
        
        format!("{:.2} {}", size, units[unit_index])
    }

    /// 格式化时间长度
    fn format_duration(seconds: i64) -> String {
        let days = seconds / 86400;
        let hours = (seconds % 86400) / 3600;
        let minutes = (seconds % 3600) / 60;
        let secs = seconds % 60;
        
        let mut parts = Vec::new();
        if days > 0 { parts.push(format!("{} 天", days)); }
        if hours > 0 { parts.push(format!("{} 小时", hours)); }
        if minutes > 0 { parts.push(format!("{} 分钟", minutes)); }
        if secs > 0 || parts.is_empty() { parts.push(format!("{} 秒", secs)); }
        
        parts.join(" ")
    }

    /// 获取主机名
    fn get_hostname() -> String {
        #[cfg(target_os = "windows")]
        {
            use std::process::Command;
            Command::new("hostname")
                .output()
                .ok()
                .and_then(|output| String::from_utf8(output.stdout).ok())
                .map(|s| s.trim().to_string())
                .unwrap_or_else(|| "unknown".to_string())
        }
        
        #[cfg(not(target_os = "windows"))]
        {
            use std::process::Command;
            Command::new("hostname")
                .output()
                .ok()
                .and_then(|output| String::from_utf8(output.stdout).ok())
                .map(|s| s.trim().to_string())
                .unwrap_or_else(|| "unknown".to_string())
        }
    }

    /// 获取操作系统名称
    fn get_os_name() -> String {
        #[cfg(target_os = "windows")]
        {
            "Windows".to_string()
        }
        
        #[cfg(target_os = "linux")]
        {
            "Linux".to_string()
        }
        
        #[cfg(target_os = "macos")]
        {
            "macOS".to_string()
        }
        
        #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
        {
            std::env::consts::OS.to_string()
        }
    }

    /// 获取本机 IP 地址（非阻塞）
    fn get_local_ip() -> Option<String> {
        use std::net::{TcpStream, SocketAddr};
        use std::time::Duration;
        
        // 尝试连接到外部地址以获取本机 IP，设置超时避免阻塞
        let socket = match TcpStream::connect_timeout(
            &"8.8.8.8:80".parse().ok()?,
            Duration::from_millis(500)
        ) {
            Ok(s) => s,
            Err(_) => return None,
        };
        
        match socket.local_addr() {
            Ok(SocketAddr::V4(addr)) => Some(addr.ip().to_string()),
            Ok(SocketAddr::V6(addr)) => Some(addr.ip().to_string()),
            Err(_) => None,
        }
    }

    /// 获取Redis监控信息
    pub async fn get_redis_metrics(&self) -> Result<RedisMetrics, AppError> {
        use std::collections::HashMap;
        use crate::app::monitor::dto::RedisCommandStat;
        
        let Some(client) = &self.redis_client else {
            return Err(AppError::with_message(ErrorCode::BadRequest, "Redis客户端未配置"));
        };

        let mut conn = client.get_connection().map_err(|e| {
            error!("连接Redis失败: {:?}", e);
            AppError::with_message(ErrorCode::DatabaseError, "连接Redis失败")
        })?;

        // 获取Redis INFO
        let info_str: String = redis::cmd("INFO")
            .query(&mut conn)
            .map_err(|e| {
                error!("获取Redis INFO失败: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, "获取Redis信息失败")
            })?;

        // 解析 INFO 为 HashMap<String, String>
        let mut info_map: HashMap<String, String> = HashMap::new();
        for line in info_str.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some((key, value)) = line.split_once(':') {
                info_map.insert(key.to_string(), value.to_string());
            }
        }

        // 获取数据库大小
        let db_size: i64 = redis::cmd("DBSIZE")
            .query(&mut conn)
            .unwrap_or(0);
        info_map.insert("keys_num".to_string(), db_size.to_string());

        // 格式化运行时间
        if let Some(uptime) = info_map.get("uptime_in_seconds") {
            if let Ok(seconds) = uptime.parse::<i64>() {
                let formatted = Self::format_uptime(seconds);
                info_map.insert("uptime_in_seconds".to_string(), formatted);
            }
        }

        // 获取命令统计信息
        let commandstats_str: String = redis::cmd("INFO")
            .arg("commandstats")
            .query(&mut conn)
            .unwrap_or_default();

        let mut stats: Vec<RedisCommandStat> = Vec::new();
        for line in commandstats_str.lines() {
            let line = line.trim();
            if line.starts_with("cmdstat_") {
                if let Some((key, value)) = line.split_once(':') {
                    // 提取命令名称，例如 "cmdstat_get" -> "get"
                    let command_name = key.strip_prefix("cmdstat_").unwrap_or(key);
                    
                    // 从值中提取 calls 数量，例如 "calls=123,usec=456,..."
                    let calls = value
                        .split(',')
                        .find(|part| part.starts_with("calls="))
                        .and_then(|part| part.strip_prefix("calls="))
                        .unwrap_or("0");
                    
                    stats.push(RedisCommandStat {
                        name: command_name.to_string(),
                        value: calls.to_string(),
                    });
                }
            }
        }

        info!("获取Redis监控信息成功: {} 个 keys", db_size);

        Ok(RedisMetrics {
            info: info_map,
            stats,
        })
    }

    /// 格式化运行时间
    fn format_uptime(seconds: i64) -> String {
        let days = seconds / 86400;
        let hours = (seconds % 86400) / 3600;
        let minutes = (seconds % 3600) / 60;
        let secs = seconds % 60;
        
        if days > 0 {
            format!("{}天{}小时{}分{}秒", days, hours, minutes, secs)
        } else if hours > 0 {
            format!("{}小时{}分{}秒", hours, minutes, secs)
        } else if minutes > 0 {
            format!("{}分{}秒", minutes, secs)
        } else {
            format!("{}秒", secs)
        }
    }

    /// 获取在线用户列表
    pub async fn get_online_sessions(&self, username: Option<String>) -> Result<Vec<OnlineSession>, AppError> {
        use crate::core::conf::SETTINGS;
        use crate::utils::encrypt::CryptoUtils;
        
        let Some(client) = &self.redis_client else {
            return Err(AppError::with_message(ErrorCode::BadRequest, "Redis客户端未配置"));
        };

        let mut conn = client.get_connection().map_err(|e| {
            error!("连接Redis失败: {:?}", e);
            AppError::with_message(ErrorCode::DatabaseError, "连接Redis失败")
        })?;
        
        // 1. 获取所有 token keys（fba:token:*）
        let token_pattern = format!("{}:*", SETTINGS.token_redis_prefix);
        info!("正在查询 Redis keys，模式: {}", token_pattern);
        
        let token_keys: Vec<String> = redis::cmd("KEYS")
            .arg(&token_pattern)
            .query(&mut conn)
            .unwrap_or_default();
        
        info!("找到 {} 个 token keys", token_keys.len());
        if token_keys.is_empty() {
            // 尝试查看 Redis 中实际存在的所有 keys（仅用于调试）
            let all_keys: Vec<String> = redis::cmd("KEYS")
                .arg("*token*")
                .query(&mut conn)
                .unwrap_or_default();
            info!("Redis 中包含 'token' 的所有 keys: {:?}", all_keys.iter().take(10).collect::<Vec<_>>());
        }

        // 2. 获取在线客户端列表
        let online_clients: Vec<String> = redis::cmd("SMEMBERS")
            .arg(&SETTINGS.token_online_redis_prefix)
            .query(&mut conn)
            .unwrap_or_default();
        
        info!("找到 {} 个在线客户端", online_clients.len());

        let mut sessions = Vec::new();

        // 3. 遍历所有 token
        for key in token_keys {
            // 获取 token 值
            let token: String = match redis::cmd("GET").arg(&key).query(&mut conn) {
                Ok(t) => t,
                Err(_) => continue,
            };

            // 解码 token 获取 payload
            let token_payload = match CryptoUtils::verify_jwt(&token, &SETTINGS.token_secret_key) {
                Ok(payload) => payload,
                Err(_) => continue,
            };

            // 解析 user_id（从 sub 字段，sub 是字符串格式的用户ID）
            let user_id: i64 = match token_payload.sub.parse() {
                Ok(id) => id,
                Err(_) => continue,
            };
            let session_uuid = token_payload.session_uuid;

            // 创建基础 token_detail
            let mut token_detail = OnlineSession {
                id: user_id,
                session_uuid: session_uuid.clone(),
                username: "未知".to_string(),
                nickname: "未知".to_string(),
                ip: "未知".to_string(),
                os: "未知".to_string(),
                browser: "未知".to_string(),
                device: "未知".to_string(),
                status: if online_clients.contains(&session_uuid) { 1 } else { 0 },
                last_login_time: "未知".to_string(),
                expire_time: chrono::DateTime::from_timestamp(token_payload.exp as i64, 0)
                    .unwrap_or_else(Utc::now),
            };

            // 4. 获取 token 额外信息
            let extra_info_key = format!("{}:{}:{}", SETTINGS.token_extra_info_redis_prefix, user_id, session_uuid);
            let extra_info_str: Option<String> = redis::cmd("GET")
                .arg(&extra_info_key)
                .query(&mut conn)
                .ok();

            if let Some(extra_info_str) = extra_info_str {
                // 解析额外信息
                if let Ok(extra_info) = serde_json::from_str::<TokenExtraInfo>(&extra_info_str) {
                    // 排除 swagger 登录生成的 token
                    if extra_info.swagger.is_some() {
                        continue;
                    }

                    // 更新 token_detail 的信息
                    let extra_username = extra_info.username.as_deref().unwrap_or("未知");
                    
                    // 如果指定了 username 过滤，只返回匹配的
                    if let Some(ref filter_username) = username {
                        if filter_username != extra_username {
                            continue;
                        }
                    }

                    token_detail.username = extra_username.to_string();
                    token_detail.nickname = extra_info.nickname.as_deref().unwrap_or("未知").to_string();
                    token_detail.ip = extra_info.ip.as_deref().unwrap_or("未知").to_string();
                    token_detail.os = extra_info.os.as_deref().unwrap_or("未知").to_string();
                    token_detail.browser = extra_info.browser.as_deref().unwrap_or("未知").to_string();
                    token_detail.device = extra_info.device.as_deref().unwrap_or("未知").to_string();
                    token_detail.last_login_time = extra_info.last_login_time.as_deref().unwrap_or("未知").to_string();

                    sessions.push(token_detail);
                } else {
                    // 如果解析失败，仍然添加基础信息
                    sessions.push(token_detail);
                }
            } else {
                // 没有额外信息，直接添加基础信息
                sessions.push(token_detail);
            }
        }

        info!("获取在线用户列表成功，共 {} 个用户", sessions.len());

        Ok(sessions)
    }

    /// 踢出指定在线用户
    pub async fn kick_out_session(&self, session_id: &str) -> Result<String, AppError> {
        let Some(client) = &self.redis_client else {
            return Err(AppError::with_message(ErrorCode::BadRequest, "Redis客户端未配置"));
        };

        let mut conn = client.get_connection().map_err(|e| {
            error!("连接Redis失败: {:?}", e);
            AppError::with_message(ErrorCode::DatabaseError, "连接Redis失败")
        })?;

        // 删除会话
        let result: Result<i64, redis::RedisError> = redis::cmd("DEL")
            .arg(format!("online_sessions:{}", session_id))
            .query(&mut conn);

        match result {
            Ok(deleted_count) if deleted_count > 0 => {
                info!("踢出在线用户成功: {}", session_id);
                Ok(format!("成功踢出用户: {}", session_id))
            }
            Ok(_) => {
                Err(AppError::with_message(ErrorCode::NotFound, "用户不在线或已退出"))
            }
            Err(e) => {
                error!("踢出用户失败: {:?}", e);
                Err(AppError::with_message(ErrorCode::DatabaseError, "踢出用户失败"))
            }
        }
    }

    /// 获取已注册任务列表
    pub async fn get_registered_tasks(&self) -> Result<Vec<TaskInfo>, AppError> {
        // TODO: 实际实现中应该从任务调度器获取真实数据
        // 这里返回模拟数据
        let tasks = vec![
            TaskInfo {
                id: 1,
                name: "数据备份".to_string(),
                description: "每日数据备份任务".to_string(),
                schedule: "0 2 * * *".to_string(),
                status: "运行中".to_string(),
                next_run: Some(Utc::now()),
            },
            TaskInfo {
                id: 2,
                name: "日志清理".to_string(),
                description: "清理过期日志".to_string(),
                schedule: "0 0 * * 0".to_string(),
                status: "运行中".to_string(),
                next_run: Some(Utc::now()),
            },
        ];

        Ok(tasks)
    }

    /// 获取API指标
    pub async fn get_api_metrics(&self) -> Result<ApiMetrics, AppError> {
        let db = Self::get_db().await;
        let base_query = opera_log::Entity::find();

        let total_requests = base_query.clone().count(&db).await.map_err(|e| {
            error!("获取总请求数失败: {:?}", e);
            AppError::with_message(ErrorCode::DatabaseError, "获取总请求数失败")
        })?;

        let success_requests = base_query
            .clone()
            .filter(opera_log::Column::Status.eq(0))
            .count(&db)
            .await
            .map_err(|e| {
                error!("获取成功请求数失败: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, "获取成功请求数失败")
            })?;

        let error_requests = total_requests - success_requests;

        let avg_response_time = {
            let logs = base_query
                .all(&db)
                .await
                .map_err(|e| {
                    error!("获取操作日志失败: {:?}", e);
                    AppError::with_message(ErrorCode::DatabaseError, "获取操作日志失败")
                })?;

            if logs.is_empty() {
                0.0
            } else {
                let total_time: f32 = logs
                    .iter()
                    .map(|log| log.cost_time)
                    .sum();
                total_time as f64 / logs.len() as f64
            }
        };

        info!("获取API指标成功");

        Ok(ApiMetrics {
            total_requests,
            success_requests,
            error_requests,
            avg_response_time,
            requests_per_minute: 500, // 这个值需要从实际统计中获取
            timestamp: Utc::now(),
        })
    }

    /// 健康检查
    pub async fn health_check(&self) -> Result<HealthStatus, AppError> {
        let db = Self::get_db().await;
        // 检查数据库连接
        let db_healthy = match login_log::Entity::find()
            .count(&db)
            .await
        {
            Ok(_) => "connected",
            Err(_) => "disconnected",
        };

        // 检查Redis连接
        let redis_healthy = if let Some(client) = &self.redis_client {
            match client.get_connection() {
                Ok(_) => "connected",
                Err(_) => "disconnected",
            }
        } else {
            "not configured"
        };

        Ok(HealthStatus {
            status: "healthy".to_string(),
            version: "0.1.0".to_string(),
            database: db_healthy.to_string(),
            redis: redis_healthy.to_string(),
            timestamp: Utc::now(),
        })
    }
}

/// 解析Redis INFO响应
fn parse_redis_info(info: &str) -> Result<RedisServerInfo, AppError> {
    let mut connected_clients = 0;
    let mut used_memory = 0;
    let mut used_memory_peak = 0;
    let mut uptime_in_seconds = 0;
    let hit_rate = 0.0;

    for line in info.lines() {
        if let Some((key, value)) = line.split_once(':') {
            match key {
                "connected_clients" => connected_clients = value.parse().unwrap_or(0),
                "used_memory" => used_memory = value.parse().unwrap_or(0),
                "used_memory_peak" => used_memory_peak = value.parse().unwrap_or(0),
                "uptime_in_seconds" => uptime_in_seconds = value.parse().unwrap_or(0),
                "keyspace_hits" | "keyspace_misses" => {
                    // 计算命中率需要keyspace_hits和keyspace_misses
                }
                _ => {}
            }
        }
    }

    Ok(RedisServerInfo {
        version: "7.0.0".to_string(),
        connected_clients,
        used_memory,
        used_memory_peak,
        uptime_in_seconds,
        hit_rate,
    })
}
