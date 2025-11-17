//! 通知公告服务层 - 匹配Python版本

use sea_orm::*;
use crate::entity::notice;
use crate::dto::*;
use crate::error::NoticeError;

/// 通知公告服务
pub struct NoticeService;

impl NoticeService {
    /// 获取所有通知公告
    pub async fn get_all(db: &DatabaseConnection) -> Result<Vec<NoticeDetail>, NoticeError> {
        let notices = notice::Entity::find()
            .order_by_desc(notice::Column::Id)
            .all(db)
            .await
            .map_err(|e| NoticeError::DatabaseError(e.to_string()))?;
        
        Ok(notices.into_iter().map(NoticeDetail::from).collect())
    }
    
    /// 根据ID获取通知公告
    pub async fn get_by_id(
        db: &DatabaseConnection,
        id: i64,
    ) -> Result<NoticeDetail, NoticeError> {
        let notice = notice::Entity::find_by_id(id)
            .one(db)
            .await
            .map_err(|e| NoticeError::DatabaseError(e.to_string()))?
            .ok_or(NoticeError::NotFound("通知公告不存在".to_string()))?;
        
        Ok(NoticeDetail::from(notice))
    }
    
    /// 获取显示的通知公告列表（公开接口）
    pub async fn get_visible(db: &DatabaseConnection) -> Result<Vec<NoticeDetail>, NoticeError> {
        let notices = notice::Entity::find_visible()
            .all(db)
            .await
            .map_err(|e| NoticeError::DatabaseError(e.to_string()))?;
        
        Ok(notices.into_iter().map(NoticeDetail::from).collect())
    }
    
    /// 分页查询通知公告
    pub async fn get_list(
        db: &DatabaseConnection,
        query: NoticeQuery,
        pagination: PaginationQuery,
    ) -> Result<PageData<NoticeDetail>, NoticeError> {
        let mut select = notice::Entity::find();
        
        // 构建查询条件
        if let Some(title) = &query.title {
            select = select.filter(notice::Column::Title.contains(title));
        }
        if let Some(type_) = query.type_ {
            select = select.filter(notice::Column::Type.eq(type_));
        }
        if let Some(status) = query.status {
            select = select.filter(notice::Column::Status.eq(status));
        }
        
        // 查询总数
        let total = select
            .clone()
            .count(db)
            .await
            .map_err(|e| NoticeError::DatabaseError(e.to_string()))?;
        
        // 分页查询（按ID倒序）
        let items = select
            .order_by_desc(notice::Column::Id)
            .offset(pagination.offset())
            .limit(pagination.limit())
            .all(db)
            .await
            .map_err(|e| NoticeError::DatabaseError(e.to_string()))?
            .into_iter()
            .map(NoticeDetail::from)
            .collect();
        
        Ok(PageData::new(items, total, pagination.page, pagination.size))
    }
    
    /// 创建通知公告
    pub async fn create(
        db: &DatabaseConnection,
        param: CreateNoticeParam,
    ) -> Result<NoticeDetail, NoticeError> {
        let notice = notice::ActiveModel {
            title: Set(param.title),
            type_: Set(param.type_),
            status: Set(param.status),
            content: Set(param.content),
            ..Default::default()
        };
        
        let result = notice
            .insert(db)
            .await
            .map_err(|e| NoticeError::DatabaseError(e.to_string()))?;
        
        Ok(NoticeDetail::from(result))
    }
    
    /// 更新通知公告
    pub async fn update(
        db: &DatabaseConnection,
        id: i64,
        param: UpdateNoticeParam,
    ) -> Result<u64, NoticeError> {
        let notice = notice::Entity::find_by_id(id)
            .one(db)
            .await
            .map_err(|e| NoticeError::DatabaseError(e.to_string()))?
            .ok_or(NoticeError::NotFound("通知公告不存在".to_string()))?;
        
        let mut notice: notice::ActiveModel = notice.into();
        notice.title = Set(param.title);
        notice.type_ = Set(param.type_);
        notice.status = Set(param.status);
        notice.content = Set(param.content);
        
        notice
            .update(db)
            .await
            .map_err(|e| NoticeError::DatabaseError(e.to_string()))?;
        
        Ok(1)
    }
    
    /// 批量删除通知公告
    pub async fn delete_batch(
        db: &DatabaseConnection,
        ids: Vec<i64>,
    ) -> Result<u64, NoticeError> {
        if ids.is_empty() {
            return Ok(0);
        }
        
        let result = notice::Entity::delete_many()
            .filter(notice::Column::Id.is_in(ids))
            .exec(db)
            .await
            .map_err(|e| NoticeError::DatabaseError(e.to_string()))?;
        
        Ok(result.rows_affected)
    }
}
