DEFINE TABLE OVERWRITE project_dependency SCHEMAFULL;

DEFINE FIELD OVERWRITE path ON project_dependency TYPE string;
DEFINE FIELD OVERWRITE created_at ON project_dependency TYPE datetime VALUE time::now() READONLY;
DEFINE FIELD OVERWRITE dependency ON project_dependency TYPE record<dependency>;
