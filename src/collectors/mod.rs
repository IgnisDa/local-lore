use std::sync::Arc;

use anyhow::Result;
use log::debug;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Set, sea_query::OnConflict};

use crate::{
    context::ProjectContext,
    entities::{dependency, prelude::Dependency},
    models::ProjectLanguage,
};

pub mod cargo_lock;
pub mod package_lock;

#[derive(Debug, Clone)]
pub struct CollectorDependency {
    pub name: String,
    pub version: String,
    pub language: ProjectLanguage,
}

impl CollectorDependency {
    pub fn new(name: String, version: String, language: ProjectLanguage) -> Self {
        Self {
            name,
            version,
            language,
        }
    }
}

pub async fn gather_project_dependencies(path: &str, ctx: &Arc<ProjectContext>) -> Result<()> {
    let mut all_dependencies = Vec::new();

    let rust_deps = cargo_lock::collect_dependencies(path).await?;
    all_dependencies.extend(rust_deps);

    let js_deps = package_lock::collect_dependencies(path).await?;
    all_dependencies.extend(js_deps);

    debug!("Processing {} dependencies", all_dependencies.len());

    let mut total_updated = 0;
    for dep_input in all_dependencies {
        let new_dep = dependency::ActiveModel {
            name: Set(dep_input.name),
            version: Set(dep_input.version),
            language: Set(dep_input.language),
            ..Default::default()
        };
        Dependency::insert(new_dep)
            .on_conflict(
                OnConflict::columns([
                    dependency::Column::Name,
                    dependency::Column::Version,
                    dependency::Column::Language,
                ])
                .update_column(dependency::Column::LastSeenAt)
                .to_owned(),
            )
            .exec(&ctx.db)
            .await?;
        total_updated += 1;
    }

    debug!(
        "Scan completed for: {} and updated {} dependencies",
        path, total_updated
    );

    let dependencies_to_index = Dependency::find()
        .filter(dependency::Column::LastIndexedAt.is_null())
        .all(&ctx.db)
        .await?;

    debug!(
        "Found {} dependencies to index: {:#?}",
        dependencies_to_index.len(),
        dependencies_to_index
    );

    Ok(())
}
