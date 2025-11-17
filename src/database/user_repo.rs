use crate::database::entity::user;
use crate::database::DatabaseConnection;
use sea_orm::{ActiveValue, ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter, QuerySelect, QueryOrder};

pub struct UserRepository;

impl UserRepository {
    pub async fn find_by_username(
        username: &str,
        db: &DatabaseConnection,
    ) -> Result<user::Model, DbErr> {
        let user = user::Entity::find()
            .filter(user::Column::Username.eq(username))
            // .filter(user::Column::DelFlag.eq(0)) 
            .one(db)
            .await?;

        user.ok_or(DbErr::RecordNotFound("User not found".to_string()))
    }

    pub async fn find_by_id(
        id: i64,
        db: &DatabaseConnection,
    ) -> Result<user::Model, DbErr> {
        let user = user::Entity::find_by_id(id)
            .filter(user::Column::DelFlag.eq(0))
            .one(db)
            .await?;

        user.ok_or(DbErr::RecordNotFound("User not found".to_string()))
    }


    pub async fn find_by_email(
        email: &str,
        db: &DatabaseConnection,
    ) -> Result<user::Model, DbErr> {
        let user = user::Entity::find()
            .filter(user::Column::Email.eq(email))
            .filter(user::Column::DelFlag.eq(0))
            .one(db)
            .await?;

        user.ok_or(DbErr::RecordNotFound("User not found".to_string()))
    }


    pub async fn find_by_phone(
        phone: &str,
        db: &DatabaseConnection,
    ) -> Result<user::Model, DbErr> {
        let user = user::Entity::find()
            .filter(user::Column::Phone.eq(phone))
            .filter(user::Column::DelFlag.eq(0))
            .one(db)
            .await?;

        user.ok_or(DbErr::RecordNotFound("User not found".to_string()))
    }


    pub async fn create(
        user: user::ActiveModel,
        db: &DatabaseConnection,
    ) -> Result<user::Model, DbErr> {
        let insert_result = user::Entity::insert(user).exec(db).await?;
        let user_id = insert_result.last_insert_id;

        user::Entity::find_by_id(user_id)
            .filter(user::Column::DelFlag.eq(0))
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("User not found after creation".to_string()))
    }


    pub async fn update(
        id: i64,
        mut user: user::ActiveModel,
        db: &DatabaseConnection,
    ) -> Result<user::Model, DbErr> {
        user.id = ActiveValue::Set(id);
        let updated_user = user::Entity::update(user).exec(db).await?;

        Ok(updated_user)
    }


    pub async fn delete(
        id: i64,
        db: &DatabaseConnection,
    ) -> Result<(), DbErr> {
        let user: user::ActiveModel = user::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("User not found".to_string()))?
            .into();

        let mut update_user = user.clone();
        update_user.del_flag = ActiveValue::Set(1);

        user::Entity::update(update_user)
            .exec(db)
            .await?;

        Ok(())
    }


    pub async fn exists_by_username(
        username: &str,
        db: &DatabaseConnection,
    ) -> Result<bool, DbErr> {
        let count = user::Entity::find()
            .filter(user::Column::Username.eq(username))
            .filter(user::Column::DelFlag.eq(0))
            .count(db)
            .await?;

        Ok(count > 0)
    }


    pub async fn exists_by_email(
        email: &str,
        db: &DatabaseConnection,
    ) -> Result<bool, DbErr> {
        let count = user::Entity::find()
            .filter(user::Column::Email.eq(email))
            .filter(user::Column::DelFlag.eq(0))
            .count(db)
            .await?;

        Ok(count > 0)
    }


    pub async fn update_last_login(
        id: i64,
        db: &DatabaseConnection,
    ) -> Result<(), DbErr> {
        let user: user::ActiveModel = user::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("User not found".to_string()))?
            .into();

        let mut update_user = user.clone();
        update_user.last_login_time = ActiveValue::Set(Some(chrono::Utc::now().naive_utc()));

        user::Entity::update(update_user)
            .exec(db)
            .await?;

        Ok(())
    }


    pub async fn find_with_pagination(
        _repo: &UserRepository,
        query: &crate::app::user::dto::UserPaginationQuery,
        db: &DatabaseConnection,
    ) -> Result<(Vec<user::Model>, usize), DbErr> {
        let mut select = user::Entity::find()
            .filter(user::Column::DelFlag.eq(0));

        if let Some(keyword) = &query.keyword {
            select = select.filter(
                sea_orm::Condition::any()
                    .add(user::Column::Username.like(format!("%{}%", keyword)))
                    .add(user::Column::Nickname.like(format!("%{}%", keyword)))
                    .add(user::Column::Email.like(format!("%{}%", keyword)))
                    .add(user::Column::Phone.like(format!("%{}%", keyword))),
            );
        }

        if let Some(username) = &query.username {
            select = select.filter(user::Column::Username.like(format!("%{}%", username)));
        }

        if let Some(nickname) = &query.nickname {
            select = select.filter(user::Column::Nickname.like(format!("%{}%", nickname)));
        }

        if let Some(email) = &query.email {
            select = select.filter(user::Column::Email.like(format!("%{}%", email)));
        }

        if let Some(phone) = &query.phone {
            select = select.filter(user::Column::Phone.like(format!("%{}%", phone)));
        }

        if let Some(status) = query.status {
            select = select.filter(user::Column::Status.eq(status));
        }

        if let Some(dept_id) = query.dept_id {
            select = select.filter(user::Column::DeptId.eq(dept_id));
        }

        if let Some(is_superuser) = query.is_superuser {
            select = select.filter(user::Column::IsSuperuser.eq(is_superuser));
        }

        if let Some(is_staff) = query.is_staff {
            select = select.filter(user::Column::IsStaff.eq(is_staff));
        }

        if let Some(start_time) = query.created_time_start {
            select = select.filter(user::Column::CreatedTime.gte(start_time.naive_utc()));
        }

        if let Some(end_time) = query.created_time_end {
            select = select.filter(user::Column::CreatedTime.lte(end_time.naive_utc()));
        }

        if let Some(sort_by) = &query.sort_by {
            let order = query.sort_order.as_ref().unwrap_or(&crate::app::user::dto::SortOrder::Desc);

            match sort_by {
                crate::app::user::dto::UserSortField::Id => {
                    select = select.order_by(user::Column::Id, match order {
                        crate::app::user::dto::SortOrder::Asc => sea_orm::Order::Asc,
                        crate::app::user::dto::SortOrder::Desc => sea_orm::Order::Desc,
                    });
                }
                crate::app::user::dto::UserSortField::Username => {
                    select = select.order_by(user::Column::Username, match order {
                        crate::app::user::dto::SortOrder::Asc => sea_orm::Order::Asc,
                        crate::app::user::dto::SortOrder::Desc => sea_orm::Order::Desc,
                    });
                }
                crate::app::user::dto::UserSortField::Nickname => {
                    select = select.order_by(user::Column::Nickname, match order {
                        crate::app::user::dto::SortOrder::Asc => sea_orm::Order::Asc,
                        crate::app::user::dto::SortOrder::Desc => sea_orm::Order::Desc,
                    });
                }
                crate::app::user::dto::UserSortField::Status => {
                    select = select.order_by(user::Column::Status, match order {
                        crate::app::user::dto::SortOrder::Asc => sea_orm::Order::Asc,
                        crate::app::user::dto::SortOrder::Desc => sea_orm::Order::Desc,
                    });
                }
                crate::app::user::dto::UserSortField::CreatedTime => {
                    select = select.order_by(user::Column::CreatedTime, match order {
                        crate::app::user::dto::SortOrder::Asc => sea_orm::Order::Asc,
                        crate::app::user::dto::SortOrder::Desc => sea_orm::Order::Desc,
                    });
                }
                crate::app::user::dto::UserSortField::UpdatedTime => {
                    select = select.order_by(user::Column::UpdatedTime, match order {
                        crate::app::user::dto::SortOrder::Asc => sea_orm::Order::Asc,
                        crate::app::user::dto::SortOrder::Desc => sea_orm::Order::Desc,
                    });
                }
                crate::app::user::dto::UserSortField::LastLoginTime => {
                    select = select.order_by(user::Column::LastLoginTime, match order {
                        crate::app::user::dto::SortOrder::Asc => sea_orm::Order::Asc,
                        crate::app::user::dto::SortOrder::Desc => sea_orm::Order::Desc,
                    });
                }
            }
        } else {
            select = select.order_by(user::Column::CreatedTime, sea_orm::Order::Desc);
        }

        let page = query.page.unwrap_or(1);
        let size = query.size.unwrap_or(20);
        let offset = (page - 1) * size;

        let mut total_select = user::Entity::find()
            .filter(user::Column::DelFlag.eq(0));

        if let Some(kw) = &query.keyword {
            total_select = total_select.filter(
                sea_orm::Condition::any()
                    .add(user::Column::Username.like(format!("%{}%", kw)))
                    .add(user::Column::Nickname.like(format!("%{}%", kw)))
                    .add(user::Column::Email.like(format!("%{}%", kw)))
                    .add(user::Column::Phone.like(format!("%{}%", kw))),
            );
        }
        if let Some(u) = &query.username {
            total_select = total_select.filter(user::Column::Username.like(format!("%{}%", u)));
        }
        if let Some(n) = &query.nickname {
            total_select = total_select.filter(user::Column::Nickname.like(format!("%{}%", n)));
        }
        if let Some(e) = &query.email {
            total_select = total_select.filter(user::Column::Email.like(format!("%{}%", e)));
        }
        if let Some(p) = &query.phone {
            total_select = total_select.filter(user::Column::Phone.like(format!("%{}%", p)));
        }
        if let Some(s) = query.status {
            total_select = total_select.filter(user::Column::Status.eq(s));
        }
        if let Some(d) = query.dept_id {
            total_select = total_select.filter(user::Column::DeptId.eq(d));
        }
        if let Some(s) = query.is_superuser {
            total_select = total_select.filter(user::Column::IsSuperuser.eq(s));
        }
        if let Some(s) = query.is_staff {
            total_select = total_select.filter(user::Column::IsStaff.eq(s));
        }
        if let Some(st) = query.created_time_start {
            total_select = total_select.filter(user::Column::CreatedTime.gte(st.naive_utc()));
        }
        if let Some(et) = query.created_time_end {
            total_select = total_select.filter(user::Column::CreatedTime.lte(et.naive_utc()));
        }

        let total = total_select.count(db).await?;

        // 查询列表
        let list = select
            .offset(offset as u64)
            .limit(size as u64)
            .all(db)
            .await?;

        Ok((list, total as usize))
    }
}

pub use UserRepository as UserCrud;
