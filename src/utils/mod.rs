/// 工具类库
/// 提供各种通用工具函数和结构体

pub mod time;
pub mod encrypt;
pub mod file;
pub mod validation;
pub mod pagination;
pub mod resp;
pub mod permission;

/// 统一的错误结果类型
pub type Result<T, E = Box<dyn std::error::Error + Send + Sync>> = std::result::Result<T, E>;

/// 分页结果结构
#[derive(Debug, Clone)]
pub struct PaginationResult<T> {
    /// 数据列表
    pub items: Vec<T>,
    /// 总条数
    pub total: u64,
    /// 当前页
    pub page: u64,
    /// 每页数量
    pub size: u64,
    /// 总页数
    pub total_pages: u64,
}

impl<T> PaginationResult<T> {
    /// 创建分页结果
    pub fn new(items: Vec<T>, total: u64, page: u64, size: u64) -> Self {
        let total_pages = if size == 0 { 0 } else { total.div_ceil(size) };
        Self {
            items,
            total,
            page,
            size,
            total_pages,
        }
    }
}

/// 分页参数
#[derive(Debug, Clone)]
pub struct PaginationParams {
    /// 页码（从1开始）
    pub page: u64,
    /// 每页数量
    pub size: u64,
    /// 排序字段
    pub sort: Option<String>,
    /// 排序方向（asc, desc）
    pub order: Option<String>,
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self {
            page: 1,
            size: 20,
            sort: None,
            order: None,
        }
    }
}

impl PaginationParams {
    /// 从查询参数创建分页参数
    pub fn from_query(
        page: Option<u64>,
        size: Option<u64>,
        sort: Option<String>,
        order: Option<String>,
    ) -> Self {
        let page = page.unwrap_or(1);
        let size = size.unwrap_or(20);
        let sort = if sort.is_some() && !sort.as_ref().unwrap().is_empty() {
            sort
        } else {
            None
        };
        let order = if order.is_some() && !order.as_ref().unwrap().is_empty() {
            order
        } else {
            Some("desc".to_string())
        };

        Self { page, size, sort, order }
    }

    /// 获取偏移量
    pub fn offset(&self) -> u64 {
        (self.page - 1) * self.size
    }

    /// 获取限制数量
    pub fn limit(&self) -> u64 {
        self.size
    }
}

/// 工具函数
pub mod common_utils {
    use std::time::{SystemTime, UNIX_EPOCH};

    /// 生成 UUID
    pub fn generate_uuid() -> String {
        uuid::Uuid::new_v4().to_string()
    }

    /// 获取当前时间戳（秒）
    pub fn current_timestamp() -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
    }

    /// 获取当前时间戳（毫秒）
    pub fn current_timestamp_millis() -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64
    }

    /// 安全地获取环境变量
    pub fn safe_get_env(key: &str) -> Option<String> {
        std::env::var(key).ok()
    }

    /// 验证邮箱格式
    pub fn is_valid_email(email: &str) -> bool {
        regex::Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")
            .unwrap()
            .is_match(email)
    }

    /// 验证手机号格式
    pub fn is_valid_phone(phone: &str) -> bool {
        regex::Regex::new(r"^1[3-9]\d{9}$")
            .unwrap()
            .is_match(phone)
    }

    /// 掩码字符串（只显示前几位和后几位）
    pub fn mask_string(s: &str, head: usize, tail: usize) -> String {
        if s.len() <= head + tail {
            return s.to_string();
        }
        format!(
            "{}{}{}",
            &s[..head],
            "*".repeat(s.len() - head - tail),
            &s[s.len() - tail..]
        )
    }
}
