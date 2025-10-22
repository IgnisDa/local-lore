use sea_orm_migration::prelude::*;

use super::m20251021_create_project::Project;
use super::m20251022_create_dependency::Dependency;

#[derive(DeriveMigrationName)]
pub struct Migration;

pub static FK_PROJECT_DEPENDENCY_DEPENDENCY: &str = "fk_project_dependency_dependency";
pub static FK_PROJECT_DEPENDENCY_PROJECT: &str = "fk_project_dependency_project";
pub static UNIQUE_INDEX_PROJECT_DEPENDENCY_ID: &str =
    "project_dependency_uq_idx_project_dependency_id";

#[derive(Iden)]
pub enum ProjectDependency {
    Table,
    Id,
    ProjectId,
    LastSeenAt,
    FirstSeenAt,
    DependencyId,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ProjectDependency::Table)
                    .col(
                        ColumnDef::new(ProjectDependency::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ProjectDependency::ProjectId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProjectDependency::DependencyId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProjectDependency::FirstSeenAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(ProjectDependency::LastSeenAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name(FK_PROJECT_DEPENDENCY_PROJECT)
                            .from(ProjectDependency::Table, ProjectDependency::ProjectId)
                            .to(Project::Table, Project::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name(FK_PROJECT_DEPENDENCY_DEPENDENCY)
                            .from(ProjectDependency::Table, ProjectDependency::DependencyId)
                            .to(Dependency::Table, Dependency::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name(UNIQUE_INDEX_PROJECT_DEPENDENCY_ID)
                    .table(ProjectDependency::Table)
                    .col(ProjectDependency::ProjectId)
                    .col(ProjectDependency::DependencyId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
