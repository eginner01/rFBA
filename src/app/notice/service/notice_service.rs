use tracing::{info, warn, error, debug};

/// 通知公告服务实现
/// 提供通知公告的增删改查、批量操作、统计等功能

use crate::app::notice::dto::{
    CreateNoticeRequest, CreateNoticeResponse, UpdateNoticeRequest, UpdateNoticeResponse,
    NoticeQuery, NoticeListResponse, NoticeListItem, NoticeDetailResponse,
    NoticeTypeStatistics, NoticeGroupStatistics,
};
use crate::common::exception::{AppError, ErrorCode};
use crate::database::DatabaseManager;
use crate::database::entity::notice::{self, NoticeType, NoticeStatus};
use sea_orm::{DatabaseConnection, DbErr, EntityTrait, QueryFilter, ColumnTrait, Select, Order};
use std::collections::HashMap;

/// 通知公告服务
pub struct NoticeService {
    db: DatabaseConnection,
}

impl NoticeService {
    /// 创建新的通知公告服务
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// 创建通知公告
    pub async fn create_notice(
        &self,
        request: &CreateNoticeRequest,
        create_by: &str,
    ) -> Result<CreateNoticeResponse, AppError> {
        let active_model = notice::ActiveModel {
            notice_id: Default::default(),
            notice_title: sea_orm::Set(request.notice_title.clone()),
            notice_type: sea_orm::Set(request.notice_type),
            notice_content: sea_orm::Set(request.notice_content.clone()),
            status: sea_orm::Set(NoticeStatus::from(request.status)),
            create_by: sea_orm::Set(create_by.to_string()),
            created_time: Default::default(),
            update_by: sea_orm::Set(None),
            updated_time: Default::default(),
            remark: sea_orm::Set(request.remark.clone()),
        };

        let saved_notice = active_model.insert(&self.db).await.map_err(|e| {
            error!("Failed to create notice: {:?}", e);
            AppError::new(ErrorCode::DatabaseError, "Failed to create notice")
        })?;

        Ok(CreateNoticeResponse {
            notice_id: saved_notice.notice_id,
            notice_title: saved_notice.notice_title,
            notice_type: saved_notice.notice_type,
            created_time: saved_notice.created_time,
        })
    }

    /// 更新通知公告
    pub async fn update_notice(
        &self,
        request: &UpdateNoticeRequest,
        update_by: Option<&str>,
    ) -> Result<UpdateNoticeResponse, AppError> {
        let existing_notice = notice::Entity::find_by_id(request.notice_id)
            .one(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to find notice: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to find notice")
            })?
            .ok_or_else(|| AppError::new(ErrorCode::NotFound, "Notice not found"))?;

        let mut active_model = existing_notice.into_active_model();
        active_model.notice_title = sea_orm::Set(request.notice_title.clone());
        active_model.notice_type = sea_orm::Set(request.notice_type);
        active_model.notice_content = sea_orm::Set(request.notice_content.clone());
        active_model.status = sea_orm::Set(NoticeStatus::from(request.status));
        active_model.update_by = sea_orm::Set(update_by.map(|s| s.to_string()));
        active_model.remark = sea_orm::Set(request.remark.clone());

        let updated_notice = active_model.update(&self.db).await.map_err(|e| {
            error!("Failed to update notice: {:?}", e);
            AppError::new(ErrorCode::DatabaseError, "Failed to update notice")
        })?;

        Ok(UpdateNoticeResponse {
            notice_id: updated_notice.notice_id,
            notice_title: updated_notice.notice_title,
            updated_time: updated_notice.updated_time,
        })
    }

    /// 删除通知公告（批量）
    pub async fn delete_notices(&self, notice_ids: &[i64]) -> Result<(), AppError> {
        if notice_ids.is_empty() {
            return Ok(());
        }

        // 批量查询公告信息
        let notices = notice::Entity::find()
            .filter(notice::Column::NoticeId.is_in(notice_ids.to_vec()))
            .all(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to find notices for deletion: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to find notices")
            })?;

        if notices.is_empty() {
            return Err(AppError::new(ErrorCode::NotFound, "Notices not found"));
        }

        // 批量删除
        for notice in notices {
            notice.delete(&self.db).await.map_err(|e| {
                error!("Failed to delete notice: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to delete notice")
            })?;
        }

        Ok(())
    }

    /// 获取公告列表（分页）
    pub async fn get_notice_list(
        &self,
        query: &NoticeQuery,
    ) -> Result<NoticeListResponse, AppError> {
        let mut select = notice::Entity::find();

        // 添加查询条件
        if let Some(notice_title) = &query.notice_title {
            select = select.filter(notice::Column::NoticeTitle.like(format!(
                "%{}%",
                notice_title
            )));
        }

        if let Some(notice_type) = query.notice_type {
            select = select.filter(notice::Column::NoticeType.eq(notice_type));
        }

        if let Some(status) = query.status {
            select = select.filter(notice::Column::Status.eq(status));
        }

        if let Some(create_by) = &query.create_by {
            select = select.filter(notice::Column::CreateBy.like(format!("%{}%", create_by)));
        }

        // 按创建时间倒序
        select = select.order_by(notice::Column::CreatedTime, Order::Desc);

        // 分页
        let page_size = query.page_size.unwrap_or(20);
        let page_num = query.page_num.unwrap_or(1);
        let offset = (page_num - 1) * page_size;

        let notices = select
            .offset(offset as u64)
            .limit(page_size as u64)
            .all(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to query notices: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to query notices")
            })?;

        let total = notice::Entity::find()
            .count(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to count notices: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to count notices")
            })?;

        let list = notices
            .into_iter()
            .map(|n| {
                let notice_type_name = match NoticeType::from(n.notice_type) {
                    NoticeType::Notification => "通知",
                    NoticeType::Announcement => "公告",
                };

                let status_name = match NoticeStatus::from(n.status) {
                    NoticeStatus::Normal => "正常",
                    NoticeStatus::Closed => "关闭",
                };

                NoticeListItem {
                    notice_id: n.notice_id,
                    notice_title: n.notice_title,
                    notice_type: n.notice_type,
                    notice_type_name: notice_type_name.to_string(),
                    status: n.status,
                    status_name: status_name.to_string(),
                    create_by: n.create_by,
                    created_time: n.created_time,
                    updated_time: n.updated_time,
                    remark: n.remark,
                }
            })
            .collect();

        Ok(NoticeListResponse {
            list,
            total: total as usize,
            page_num,
            page_size,
        })
    }

    /// 获取公告详情
    pub async fn get_notice_detail(&self, notice_id: i64) -> Result<NoticeDetailResponse, AppError> {
        let n = notice::Entity::find_by_id(notice_id)
            .one(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to find notice: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to find notice")
            })?
            .ok_or_else(|| AppError::new(ErrorCode::NotFound, "Notice not found"))?;

        let notice_type_name = match NoticeType::from(n.notice_type) {
            NoticeType::Notification => "通知",
            NoticeType::Announcement => "公告",
        };

        let status_name = match NoticeStatus::from(n.status) {
            NoticeStatus::Normal => "正常",
            NoticeStatus::Closed => "关闭",
        };

        Ok(NoticeDetailResponse {
            notice_id: n.notice_id,
            notice_title: n.notice_title,
            notice_type: n.notice_type,
            notice_type_name: notice_type_name.to_string(),
            notice_content: n.notice_content,
            status: n.status,
            status_name: status_name.to_string(),
            create_by: n.create_by,
            created_time: n.created_time,
            update_by: n.update_by,
            updated_time: n.updated_time,
            remark: n.remark,
        })
    }

    /// 获取正常状态公告列表
    pub async fn get_normal_notices(&self) -> Result<Vec<NoticeListItem>, AppError> {
        let notices = notice::Entity::find_normal()
            .order_by(notice::Column::CreatedTime, Order::Desc)
            .all(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to query normal notices: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to query normal notices")
            })?;

        let list = notices
            .into_iter()
            .map(|n| {
                let notice_type_name = match NoticeType::from(n.notice_type) {
                    NoticeType::Notification => "通知",
                    NoticeType::Announcement => "公告",
                };

                NoticeListItem {
                    notice_id: n.notice_id,
                    notice_title: n.notice_title,
                    notice_type: n.notice_type,
                    notice_type_name: notice_type_name.to_string(),
                    status: n.status,
                    status_name: "正常".to_string(),
                    create_by: n.create_by,
                    created_time: n.created_time,
                    updated_time: n.updated_time,
                    remark: n.remark,
                }
            })
            .collect();

        Ok(list)
    }

    /// 关闭公告
    pub async fn close_notice(&self, notice_id: i64, update_by: Option<&str>) -> Result<(), AppError> {
        let existing_notice = notice::Entity::find_by_id(notice_id)
            .one(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to find notice: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to find notice")
            })?
            .ok_or_else(|| AppError::new(ErrorCode::NotFound, "Notice not found"))?;

        if existing_notice.status == 1 {
            return Err(AppError::new(
                ErrorCode::Conflict,
                "Notice is already closed",
            ));
        }

        let mut active_model = existing_notice.into_active_model();
        active_model.status = sea_orm::Set(NoticeStatus::Closed);
        active_model.update_by = sea_orm::Set(update_by.map(|s| s.to_string()));

        active_model.update(&self.db).await.map_err(|e| {
            error!("Failed to close notice: {:?}", e);
            AppError::new(ErrorCode::DatabaseError, "Failed to close notice")
        })?;

        Ok(())
    }

    /// 重新发布公告
    pub async fn reopen_notice(&self, notice_id: i64, update_by: Option<&str>) -> Result<(), AppError> {
        let existing_notice = notice::Entity::find_by_id(notice_id)
            .one(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to find notice: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to find notice")
            })?
            .ok_or_else(|| AppError::new(ErrorCode::NotFound, "Notice not found"))?;

        if existing_notice.status == 0 {
            return Err(AppError::new(
                ErrorCode::Conflict,
                "Notice is already normal",
            ));
        }

        let mut active_model = existing_notice.into_active_model();
        active_model.status = sea_orm::Set(NoticeStatus::Normal);
        active_model.update_by = sea_orm::Set(update_by.map(|s| s.to_string()));

        active_model.update(&self.db).await.map_err(|e| {
            error!("Failed to reopen notice: {:?}", e);
            AppError::new(ErrorCode::DatabaseError, "Failed to reopen notice")
        })?;

        Ok(())
    }

    /// 获取公告类型统计
    pub async fn get_notice_type_statistics(&self) -> Result<Vec<NoticeTypeStatistics>, AppError> {
        let notices = notice::Entity::find()
            .all(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to query notices: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to query notices")
            })?;

        let mut stats_map: HashMap<i32, usize> = HashMap::new();

        for notice in notices {
            *stats_map.entry(notice.notice_type).or_insert(0) += 1;
        }

        let mut stats = Vec::new();
        for (notice_type, count) in stats_map {
            let notice_type_name = match NoticeType::from(notice_type) {
                NoticeType::Notification => "通知",
                NoticeType::Announcement => "公告",
            };

            stats.push(NoticeTypeStatistics {
                notice_type,
                notice_type_name: notice_type_name.to_string(),
                count,
            });
        }

        Ok(stats)
    }

    /// 获取公告分组统计
    pub async fn get_notice_group_statistics(&self) -> Result<NoticeGroupStatistics, AppError> {
        let notices = notice::Entity::find()
            .all(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to query notices: {:?}", e);
                AppError::new(ErrorCode::DatabaseError, "Failed to query notices")
            })?;

        let mut normal_count = 0;
        let mut closed_count = 0;
        let mut notification_count = 0;
        let mut announcement_count = 0;

        for notice in notices {
            if notice.status == 0 {
                normal_count += 1;
            } else {
                closed_count += 1;
            }

            if notice.notice_type == 1 {
                notification_count += 1;
            } else {
                announcement_count += 1;
            }
        }

        Ok(NoticeGroupStatistics {
            normal_count,
            closed_count,
            notification_count,
            announcement_count,
            total_count: normal_count + closed_count,
        })
    }
}
