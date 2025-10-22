# Rules for Local Lore

- Do not add code comments unless strictly necessary.
- When testing the MCP server, use turbomcp-cli:

  ```bash
  turbomcp-cli tools call list_directory --arguments '{"path": "/tmp"}' --command './target/release/local-lore'
  ```
