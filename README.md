# Local Lore

A self-hosted MCP server that indexes your project's dependencies and exposes
version-specific documentation for LLM agents—your local "lore" for code wisdom.

Inspired by Context7, but offline-first: Scan `package.json` or `requirements.txt`,
fetch/cache docs, and serve via MCP tools like `search_deps` or `get_doc`.
