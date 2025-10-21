DEFINE TABLE OVERWRITE dependency SCHEMAFULL;

DEFINE FIELD OVERWRITE language ON dependency TYPE string;
DEFINE FIELD OVERWRITE name ON dependency TYPE string;
DEFINE FIELD OVERWRITE version ON dependency TYPE string;
DEFINE FIELD OVERWRITE first_seen_at ON dependency TYPE datetime VALUE time::now() READONLY;
DEFINE FIELD OVERWRITE last_indexed_at ON dependency TYPE option<string>;
