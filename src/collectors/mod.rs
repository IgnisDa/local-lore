use std::sync::Arc;

use anyhow::Result;
use log::debug;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};

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

pub async fn scan_directory(path: &str, ctx: &Arc<ProjectContext>) -> Result<()> {
    let mut all_dependencies = Vec::new();

    let rust_deps = cargo_lock::collect_dependencies(path).await?;
    all_dependencies.extend(rust_deps);

    let js_deps = package_lock::collect_dependencies(path).await?;
    all_dependencies.extend(js_deps);

    debug!("Processing {} dependencies", all_dependencies.len());

    let mut total_upsert = 0;
    for dep_input in all_dependencies {
        let existing = Dependency::find()
            .filter(dependency::Column::Name.eq(&dep_input.name))
            .filter(dependency::Column::Version.eq(&dep_input.version))
            .filter(dependency::Column::Language.eq(dep_input.language.clone()))
            .one(&ctx.db)
            .await?;

        if existing.is_none() {
            let new_dep = dependency::ActiveModel {
                name: Set(dep_input.name),
                version: Set(dep_input.version),
                language: Set(dep_input.language),
                ..Default::default()
            };
            new_dep.insert(&ctx.db).await?;
            total_upsert += 1;
        }
    }

    debug!(
        "Scan completed for: {} and inserted {} new dependencies",
        path, total_upsert
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
