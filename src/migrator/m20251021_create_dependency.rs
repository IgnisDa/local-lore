use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

pub static UNIQUE_INDEX_LANGUAGE_NAME_VERSION: &str = "dependency_idx_language_name_version";

#[derive(Iden)]
pub enum Dependency {
    Table,
    Id,
    Name,
    Version,
    Language,
    FirstSeenAt,
    LastIndexedAt,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Dependency::Table)
                    .col(
                        ColumnDef::new(Dependency::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Dependency::Language).text().not_null())
                    .col(ColumnDef::new(Dependency::Name).text().not_null())
                    .col(ColumnDef::new(Dependency::Version).text().not_null())
                    .col(
                        ColumnDef::new(Dependency::FirstSeenAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(Dependency::LastIndexedAt).timestamp_with_time_zone())
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name(UNIQUE_INDEX_LANGUAGE_NAME_VERSION)
                    .table(Dependency::Table)
                    .col(Dependency::Language)
                    .col(Dependency::Name)
                    .col(Dependency::Version)
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
