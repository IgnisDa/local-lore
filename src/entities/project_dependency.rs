use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "project_dependency")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub path: String,
    pub dependency_id: Uuid,
    pub last_seen_at: DateTimeUtc,
    pub first_seen_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::dependency::Entity",
        from = "Column::DependencyId",
        to = "super::dependency::Column::Id"
    )]
    Dependency,
}

impl Related<super::dependency::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Dependency.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
