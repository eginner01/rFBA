pub use sea_orm_migration::prelude::*;

// 在这里添加所有的迁移模块
// mod m20240101_000001_create_users_table;
// mod m20240102_000001_create_roles_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            // 在这里按顺序添加迁移
            // Box::new(m20240101_000001_create_users_table::Migration),
            // Box::new(m20240102_000001_create_roles_table::Migration),
        ]
    }
}
