# ralph-hook-fmt

A formatting hook plugin for Claude Code that automatically formats files after Write/Edit operations.

## Features

- **Automatic formatting**: Files are formatted immediately after Write/Edit operations
- **Multi-language support**: JavaScript/TypeScript, Rust, Python, Java, Go
- **Smart formatter detection**: Automatically detects and uses the appropriate formatter for your project
- **Non-blocking**: Async, always continues after formatting

## Supported Languages & Formatters

| Language | Formatters (Priority Order) |
|----------|----------------------------|
| JavaScript/TypeScript | `biome format` > `prettier` > `dprint` |
| Rust | `rustfmt` (via `cargo fmt`) |
| Python | `ruff format` > `black` > `autopep8` > `yapf` |
| Java | `spotless` (Maven/Gradle) > `google-java-format` > `palantir-java-format` |
| Go | `goimports` > `gofmt` |
| JSON/JSONC/JSON5 | `oxfmt` (project-local > global) |
| YAML | `oxfmt` (project-local > global) |
| TOML | `oxfmt` (project-local > global) |
| HTML | `oxfmt` (project-local > global) |
| Vue | `oxfmt` (project-local > global) |
| CSS/SCSS/Less | `oxfmt` (project-local > global) |
| Markdown/MDX | `oxfmt` (project-local > global) |
| GraphQL | `oxfmt` (project-local > global) |
| Handlebars | `oxfmt` (project-local > global) |

## Installation

```bash
claude plugin marketplace add chenhunghan/ralph-hook-fmt
claude plugin install ralph-hook-fmt
```

## Update

```bash
# Update the marketplace first (fetches latest from GitHub)
claude plugin marketplace update ralph-hook-fmt

# Then update the plugin
claude plugin update ralph-hook-fmt@ralph-hook-fmt
```

## How It Works

1. When Claude Code performs a Write or Edit operation, the hook is triggered
2. The hook reads the file path from the tool input
3. Based on the file extension, the appropriate formatter is detected
4. The formatter is executed, modifying the file in place
5. A response is returned indicating whether formatting was applied

## Development

```bash
# Run tests
make test

# Run linting
make lint

# Format code
make fmt

# Run all CI checks
make ci
```

## License

MIT
