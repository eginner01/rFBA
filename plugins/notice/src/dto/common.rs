//! 通用DTO定义

use serde::{Deserialize, Serialize};

/// 分页查询参数
#[derive(Debug, Deserialize, Clone)]
pub struct PaginationQuery {
    /// 页码（从1开始）
    #[serde(default = "default_page")]
    pub page: u64,
    
    /// 每页数量
    #[serde(default = "default_size")]
    pub size: u64,
}

fn default_page() -> u64 {
    1
}

fn default_size() -> u64 {
    10
}

impl PaginationQuery {
    /// 获取offset
    pub fn offset(&self) -> u64 {
        (self.page - 1) * self.size
    }
    
    /// 获取limit
    pub fn limit(&self) -> u64 {
        self.size
    }
}

/// 分页响应数据
#[derive(Debug, Serialize)]
pub struct PageData<T> {
    /// 总记录数
    pub total: u64,
    
    /// 当前页数据
    pub items: Vec<T>,
    
    /// 当前页码
    pub page: u64,
    
    /// 每页数量
    pub size: u64,
    
    /// 总页数
    pub pages: u64,
}

impl<T> PageData<T> {
    pub fn new(items: Vec<T>, total: u64, page: u64, size: u64) -> Self {
        let pages = if total == 0 {
            0
        } else {
            (total + size - 1) / size
        };
        
        Self {
            total,
            items,
            page,
            size,
            pages,
        }
    }
}

/// API响应结构
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    /// 状态码
    pub code: i32,
    
    /// 消息
    pub msg: String,
    
    /// 数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            code: 200,
            msg: "操作成功".to_string(),
            data: Some(data),
        }
    }
    
    pub fn success_with_msg(msg: impl Into<String>, data: T) -> Self {
        Self {
            code: 200,
            msg: msg.into(),
            data: Some(data),
        }
    }
}

impl ApiResponse<()> {
    pub fn success_msg(msg: impl Into<String>) -> Self {
        Self {
            code: 200,
            msg: msg.into(),
            data: None,
        }
    }
    
    pub fn error(code: i32, msg: impl Into<String>) -> Self {
        Self {
            code,
            msg: msg.into(),
            data: None,
        }
    }
}

/// 批量删除请求参数
#[derive(Debug, Deserialize)]
pub struct DeleteBatchParam {
    /// ID列表
    pub ids: Vec<i64>,
}
