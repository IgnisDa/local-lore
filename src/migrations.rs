pub static ALL_MIGRATIONS: &[&str] = &[MIGRATION_2025_10_21, MIGRATION_2025_10_22];

static MIGRATION_2025_10_21: &str = r#"
DEFINE TABLE OVERWRITE dependency SCHEMAFULL;

DEFINE FIELD OVERWRITE language ON dependency TYPE string;
DEFINE FIELD OVERWRITE name ON dependency TYPE string;
DEFINE FIELD OVERWRITE version ON dependency TYPE string;
DEFINE FIELD OVERWRITE first_seen_at ON dependency TYPE datetime VALUE time::now() READONLY;
DEFINE FIELD OVERWRITE last_indexed_at ON dependency TYPE option<string>;

DEFINE INDEX OVERWRITE dependency_idx_last_indexed_at ON dependency FIELDS last_indexed_at;
"#;

static MIGRATION_2025_10_22: &str = r#"
DEFINE TABLE OVERWRITE project_dependency SCHEMAFULL;

DEFINE FIELD OVERWRITE dependency_id ON project_dependency TYPE record<dependency>;
DEFINE FIELD OVERWRITE path ON project_dependency TYPE string;
DEFINE FIELD OVERWRITE first_seen_at ON project_dependency TYPE datetime VALUE time::now() READONLY;
DEFINE FIELD OVERWRITE last_seen_at ON project_dependency TYPE datetime VALUE time::now();

DEFINE INDEX OVERWRITE project_dependency_idx_dependency_id ON project_dependency FIELDS dependency_id;
"#;
