# ralph-hook-fmt

A formatting hook plugin for Claude Code that automatically formats files after Write/Edit operations.

See lint hook: [ralph-hook-lint](https://github.com/chenhunghan/ralph-hook-lint)

## Features

- **Automatic formatting**: Files are formatted immediately after Write/Edit operations
- **Multi-language support**: JavaScript/TypeScript, Rust, Python, Java, Go
- **Smart formatter detection**: Automatically detects and uses the appropriate formatter for your project
- **Non-blocking**: Async, always continues after formatting

## Supported Languages & Formatters

| Language              | Formatters (Priority Order)                                               |
| --------------------- | ------------------------------------------------------------------------- |
| JavaScript/TypeScript | `biome format` > `prettier` > `dprint`                                    |
| Rust                  | `rustfmt` (via `cargo fmt`)                                               |
| Python                | `ruff format` > `black` > `autopep8` > `yapf`                             |
| Java                  | `spotless` (Maven/Gradle) > `google-java-format` > `palantir-java-format` |
| Go                    | `goimports + gofumpt` > `gofumpt` > `goimports` > `gofmt`                 |
| JSON/JSONC/JSON5      | `oxfmt` (project-local > global)                                          |
| YAML                  | `oxfmt` (project-local > global)                                          |
| TOML                  | `oxfmt` (project-local > global)                                          |
| HTML                  | `oxfmt` (project-local > global)                                          |
| Vue                   | `oxfmt` (project-local > global)                                          |
| CSS/SCSS/Less         | `oxfmt` (project-local > global)                                          |
| Markdown/MDX          | `oxfmt` (project-local > global)                                          |
| GraphQL               | `oxfmt` (project-local > global)                                          |
| Handlebars            | `oxfmt` (project-local > global)                                          |

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
