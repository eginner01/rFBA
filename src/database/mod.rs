use sea_orm::{Database, DbErr};
use tokio::sync::OnceCell;
use tracing::{info, error};

use crate::core::SETTINGS;

// 导出实体模型
pub mod entity {
    pub mod prelude;
    pub mod user;
    pub mod role;
    pub mod menu;
    pub mod dept;
    pub mod user_role;
    pub mod data_scope;
    pub mod data_rule;
    pub mod role_data_scope;
    pub mod data_scope_rule;
    pub mod dict_type;
    pub mod opera_log;
    pub mod login_log;
    pub mod task_scheduler;
    pub mod task_result;
}

// 导出Repository
pub mod user_repo;
pub mod role_repo;
pub mod menu_repo;
pub mod user_role_repo;
// pub mod data_scope_repo;  // 已废弃，表结构已更改，service 层直接使用 entity
// pub mod data_rule_repo;  // 已废弃，service 层直接使用 entity

// 导出Redis管理器
pub mod redis;

// 重新导出所有模块
pub use entity::*;
pub use user_repo::*;
pub use role_repo::*;
pub use menu_repo::*;
pub use user_role_repo::*;
// pub use data_scope_repo::*;  // 已废弃
// pub use data_rule_repo::*;  // 已废弃

// 重新导出数据库连接类型
pub use sea_orm::DatabaseConnection;

/// 全局数据库连接
static DATABASE_CONNECTION: OnceCell<DatabaseConnection> = OnceCell::const_new();

/// 数据库连接管理器
pub struct DatabaseManager;

impl DatabaseManager {
    /// 初始化数据库连接
    pub async fn init() -> Result<(), DbErr> {
        info!("正在初始化数据库连接...");

        let database_type = match SETTINGS.database_type {
            crate::core::conf::DatabaseType::PostgreSQL => "PostgreSQL",
            crate::core::conf::DatabaseType::MySQL => "MySQL",
            crate::core::conf::DatabaseType::SQLite => "SQLite",
        };

        info!("数据库类型: {}", database_type);
        info!("主机地址: {}:{}", SETTINGS.database_host, SETTINGS.database_port);
        info!("数据库名: {}", SETTINGS.database_name);
        info!("连接池大小: {}", SETTINGS.database_pool_size);

        let database_url = SETTINGS.database_url();
        info!("数据库 URL: {}", database_url.replace(&SETTINGS.database_password, "****"));

        let connection = Database::connect(database_url.as_str()).await?;

        // 测试连接
        info!("数据库连接建立成功");

        // 设置全局连接
        DATABASE_CONNECTION.set(connection).map_err(|_| {
            error!("设置全局数据库连接失败");
            DbErr::Custom("Failed to set global database connection".to_string())
        })?;


        info!("数据库初始化完成");
        Ok(())
    }


    /// 获取数据库连接
    pub async fn get_connection() -> &'static DatabaseConnection {
        DATABASE_CONNECTION.get().expect("Database not initialized. Call DatabaseManager::init() first.")
    }

    /// 运行数据库迁移
    pub async fn run_migrations() -> Result<(), DbErr> {
        use sea_orm_migration::prelude::*;
        
        info!("正在运行数据库迁移...");
        
        let db = Self::get_connection().await;
        
        struct EmptyMigrator;
        
        #[async_trait::async_trait]
        impl MigratorTrait for EmptyMigrator {
            fn migrations() -> Vec<Box<dyn MigrationTrait>> {
                vec![]
            }
        }
        
        EmptyMigrator::up(db, None).await?;
        
        info!("数据库迁移完成");
        Ok(())
    }
    
    /// 检查数据库迁移状态
    pub async fn check_migration_status() -> Result<(), DbErr> {
        use sea_orm_migration::prelude::*;
        
        let db = Self::get_connection().await;
        
        struct EmptyMigrator;
        
        #[async_trait::async_trait]
        impl MigratorTrait for EmptyMigrator {
            fn migrations() -> Vec<Box<dyn MigrationTrait>> {
                vec![]
            }
        }
        
        EmptyMigrator::status(db).await?;
        
        Ok(())
    }

    /// 关闭数据库连接
    pub async fn close() {
        if let Some(conn) = DATABASE_CONNECTION.get() {
            info!("正在关闭数据库连接...");
            let _ = (*conn).clone().close().await;
            info!("数据库连接已关闭");
        }
    }
}

/// 异步数据库获取

#[allow(dead_code)]
#[allow(async_fn_in_trait)]
pub trait DatabaseService {
    async fn get_db() -> Result<DatabaseConnection, DbErr>;
}


impl DatabaseService for DatabaseConnection {
    async fn get_db() -> Result<DatabaseConnection, DbErr> {
        Ok(DatabaseManager::get_connection().await.clone())
    }
}

/// 数据库获取器
pub type DbService = std::sync::Arc<tokio::sync::Mutex<DatabaseConnection>>;

/// 创建数据库连接池
pub async fn create_connection_pool() -> Result<DbService, DbErr> {
    let conn = DatabaseManager::get_connection().await.clone();
    Ok(std::sync::Arc::new(tokio::sync::Mutex::new(conn)))
}
