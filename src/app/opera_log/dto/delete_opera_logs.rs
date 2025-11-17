use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct DeleteOperaLogsRequest {
    pub ids: Vec<i64>,
}

/// 删除操作日志响应
#[derive(Debug, Serialize)]
pub struct DeleteOperaLogsResponse {
    pub count: usize,
}
