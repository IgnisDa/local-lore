use sea_orm::{DeriveActiveEnum, EnumIter};
use sea_orm_migration::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize, EnumIter, DeriveActiveEnum)]
#[sea_orm(
    rs_type = "String",
    rename_all = "snake_case",
    db_type = "String(StringLen::None)"
)]
pub enum ProjectLanguage {
    Rust,
    JavaScript,
}
