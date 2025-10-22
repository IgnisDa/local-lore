use std::sync::Arc;

use anyhow::Result;
use log::debug;
use sea_orm::{EntityTrait, Set, sea_query::OnConflict};

use crate::{
    context::LocalLoreContext,
    entities::{
        dependency,
        prelude::{Dependency, ProjectDependency},
        project_dependency,
    },
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

pub async fn gather_project_dependencies(path: &str, ctx: &Arc<LocalLoreContext>) -> Result<()> {
    let mut all_dependencies = Vec::new();

    let rust_deps = cargo_lock::collect_dependencies(path).await?;
    all_dependencies.extend(rust_deps);

    let js_deps = package_lock::collect_dependencies(path).await?;
    all_dependencies.extend(js_deps);

    debug!("Processing {} dependencies", all_dependencies.len());

    let mut total_updated = 0;
    for dep_input in all_dependencies {
        let dep_name = dep_input.name;
        let dep_version = dep_input.version;
        let dep_language = dep_input.language;

        let new_dep = dependency::ActiveModel {
            name: Set(dep_name.clone()),
            version: Set(dep_version.clone()),
            language: Set(dep_language.clone()),
            ..Default::default()
        };
        let dependency_record = Dependency::insert(new_dep)
            .on_conflict(
                OnConflict::columns([
                    dependency::Column::Name,
                    dependency::Column::Version,
                    dependency::Column::Language,
                ])
                .update_column(dependency::Column::LastSeenAt)
                .to_owned(),
            )
            .exec_with_returning(&ctx.db)
            .await?;

        let new_project_dep = project_dependency::ActiveModel {
            path: Set(path.to_string()),
            dependency_id: Set(dependency_record.id),
            ..Default::default()
        };
        ProjectDependency::insert(new_project_dep)
            .on_conflict(
                OnConflict::columns([
                    project_dependency::Column::Path,
                    project_dependency::Column::DependencyId,
                ])
                .update_column(project_dependency::Column::LastSeenAt)
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

    Ok(())
}
