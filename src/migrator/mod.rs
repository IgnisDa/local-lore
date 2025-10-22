use sea_orm_migration::prelude::*;

mod m20251021_create_dependency;
mod m20251022_create_project_dependency;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20251021_create_dependency::Migration),
            Box::new(m20251022_create_project_dependency::Migration),
        ]
    }
}
