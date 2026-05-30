# Project Instructions

This file provides context for AI assistants working on this project.

## Project Type: Rust

### Commands
- Build: `cargo build`
- Test: `cargo test`
- Run: `cargo run`
- Check: `cargo check`
- Format: `cargo fmt`
- Lint: `cargo clippy`

### Project: leptos-question-bank

### Documentation
See README.md for project overview.

### Version Control
This project uses Git. See .gitignore for excluded files.


## Guidelines

- Follow existing code style and patterns
- Write tests for new functionality
- Keep changes focused and atomic
- Document public APIs

## Important Notes

### Playwright MCP

This project includes `@playwright/mcp` (v0.0.75) for browser automation via MCP.

**Config:** `mcp.json` at the project root.

**MCP client setup** — point your MCP client to `mcp.json` or configure manually:

```json
{
  "mcpServers": {
    "playwright": {
      "command": "npx",
      "args": ["-y", "@playwright/mcp@latest"]
    }
  }
}
```

**Usage:** ensure the dev server is running (`cargo leptos watch --split`), then the MCP
server can navigate to `http://127.0.0.1:3000` and interact with the app.
