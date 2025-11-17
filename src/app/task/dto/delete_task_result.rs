/// 删除任务结果 DTO

use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DeleteTaskRequest {
    /// 任务结果 ID 列表
    #[validate(length(min = 1))]
    pub ids: Vec<i64>,
}

#[derive(Debug, Serialize)]
pub struct DeleteTaskResponse {
    pub message: String,
}
