/// 分页相关结构体和工具函数

use serde::{Serialize, Deserialize};

/// 分页链接
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationLinks {
    /// 首页链接
    pub first: String,
    /// 尾页链接
    pub last: String,
    /// 当前页链接
    #[serde(rename = "self")]
    pub self_link: String,
    /// 下一页链接
    pub next: Option<String>,
    /// 上一页链接
    pub prev: Option<String>,
}

/// 分页数据 - 与 Python 版本完全一致
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageData<T> {
    /// 当前页数据列表
    pub items: Vec<T>,
    /// 数据总条数
    pub total: i64,
    /// 当前页码
    pub page: i64,
    /// 每页数量
    pub size: i64,
    /// 总页数
    pub total_pages: i64,
    /// 分页链接
    pub links: PaginationLinks,
}

impl<T> PageData<T> {
    /// 创建分页数据
    pub fn new(items: Vec<T>, total: i64, page: i64, size: i64) -> Self {
        let total_pages = if total > 0 {
            (total as f64 / size as f64).ceil() as i64
        } else {
            1
        };

        let links = PaginationLinks {
            first: format!("?page=1&size={}", size),
            last: format!("?page={}&size={}", total_pages, size),
            self_link: format!("?page={}&size={}", page, size),
            next: if page < total_pages {
                Some(format!("?page={}&size={}", page + 1, size))
            } else {
                None
            },
            prev: if page > 1 {
                Some(format!("?page={}&size={}", page - 1, size))
            } else {
                None
            },
        };

        Self {
            items,
            total,
            page,
            size,
            total_pages,
            links,
        }
    }
}
