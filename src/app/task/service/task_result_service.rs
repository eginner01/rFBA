/// 任务结果服务实现
/// 提供任务结果的CRUD、查询等功能

use tracing::{info, error};

use crate::app::task::dto::{
    TaskResultDetailResponse, TaskResultListItem,
    TaskResultListQuery, DeleteTaskRequest,
};
use crate::common::exception::{AppError, ErrorCode};
use crate::database::entity::task_result;
use sea_orm::{
    DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, ColumnTrait, Order, PaginatorTrait,
};

/// 任务结果服务
pub struct TaskResultService {
    db: DatabaseConnection,
}

impl TaskResultService {
    /// 创建新的任务结果服务
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// 获取任务结果详情
    pub async fn get_by_id(&self, id: i64) -> Result<TaskResultDetailResponse, AppError> {
        let result = task_result::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to find task result: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, "Failed to find task result")
            })?
            .ok_or_else(|| AppError::with_message(ErrorCode::NotFound, "Task result not found"))?;

        Ok(TaskResultDetailResponse::from_model(&result))
    }

    /// 获取任务结果列表（分页）
    pub async fn get_list(&self, query: &TaskResultListQuery) -> Result<(Vec<TaskResultListItem>, u64), AppError> {
        let mut select = task_result::Entity::find();

        // 按任务名称筛选
        if let Some(ref name) = query.name {
            select = select.filter(task_result::Column::Name.like(format!("%{}%", name)));
        }

        // 按任务ID筛选
        if let Some(ref task_id) = query.task_id {
            select = select.filter(task_result::Column::TaskId.like(format!("%{}%", task_id)));
        }

        // 按执行状态筛选
        if let Some(ref status) = query.status {
            select = select.filter(task_result::Column::Status.like(format!("%{}%", status)));
        }

        // 计算总数
        let total = select
            .clone()
            .count(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to count task results: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, "Failed to count task results")
            })?;

        // 获取数据
        let results = select
            .order_by(task_result::Column::Id, Order::Desc)
            .all(&self.db)
            .await
            .map_err(|e| {
                error!("Failed to query task results: {:?}", e);
                AppError::with_message(ErrorCode::DatabaseError, "Failed to query task results")
            })?;

        let result_list = results
            .iter()
            .map(TaskResultListItem::from_model)
            .collect();

        Ok((result_list, total))
    }

    /// 批量删除任务结果
    pub async fn delete_batch(&self, request: &DeleteTaskRequest) -> Result<u64, AppError> {
        let mut deleted_count = 0;

        for id in &request.ids {
            let delete_result = task_result::Entity::delete_by_id(*id)
                .exec(&self.db)
                .await
                .map_err(|e| {
                    error!("Failed to delete task result {}: {:?}", id, e);
                    AppError::with_message(ErrorCode::DatabaseError, "Failed to delete task result")
                })?;

            deleted_count += delete_result.rows_affected;
        }

        info!("Deleted {} task results", deleted_count);

        Ok(deleted_count)
    }
}
