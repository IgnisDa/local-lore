use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::ProjectLanguage;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "dependency")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub version: String,
    pub language: ProjectLanguage,
    pub last_seen_at: DateTimeUtc,
    pub first_seen_at: DateTimeUtc,
    pub last_indexed_at: Option<DateTimeUtc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::project_dependency::Entity")]
    ProjectDependency,
}

impl Related<super::project_dependency::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ProjectDependency.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
