/// 更新字典类型请求 DTO
/// 与Python版本一致：name, code, remark

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateDictTypeRequest {
    pub name: String,
    pub code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
}
