use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct DeleteLoginLogsRequest {
    pub ids: Vec<i64>,
}

/// 删除登录日志响应
#[derive(Debug, Serialize)]
pub struct DeleteLoginLogsResponse {
    pub count: usize,
}
