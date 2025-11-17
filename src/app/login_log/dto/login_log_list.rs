use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct LoginLogQuery {
    pub username: Option<String>,
    pub ip: Option<String>,
    pub status: Option<i32>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
}

/// 登录日志列表项（旧版结构，保留以兼容历史，但目前未在新服务中使用）
#[derive(Debug, Serialize)]
pub struct LoginLogListItem {
    pub id: i64,
    pub user_id: Option<i64>,
    pub username: String,
    pub ip: Option<String>,
    pub location: Option<String>,
    pub device: Option<String>,
    pub os: Option<String>,
    pub browser: Option<String>,
    pub status: i32,
    pub message: Option<String>,
    pub created_time: String,
}

