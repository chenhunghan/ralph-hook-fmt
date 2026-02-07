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
| JavaScript/TypeScript | `oxfmt` > `biome format` > `prettier` > `dprint`                          |
| Rust                  | `rustfmt` (via `cargo fmt`)                                               |
| Python                | `ruff format` > `black` > `autopep8` > `yapf`                             |
| Java                  | `spotless` (Maven/Gradle) > `google-java-format` > `palantir-java-format` |
| Go                    | `goimports + gofumpt` > `gofumpt` > `goimports` > `gofmt`                 |
| JSON/JSONC/JSON5      | `oxfmt`                                                                   |
| YAML                  | `oxfmt`                                                                   |
| TOML                  | `oxfmt`                                                                   |
| HTML                  | `oxfmt`                                                                   |
| Vue                   | `oxfmt`                                                                   |
| CSS/SCSS/Less         | `oxfmt`                                                                   |
| Markdown/MDX          | `oxfmt`                                                                   |
| GraphQL               | `oxfmt`                                                                   |
| Handlebars            | `oxfmt`                                                                   |

## Installation

```bash
claude plugin marketplace add chenhunghan/ralph-hook-fmt
claude plugin install ralph-hook-fmt
```

## Update Plugin

```bash
claude plugin marketplace update ralph-hook-fmt
claude plugin update ralph-hook-fmt@ralph-hook-fmt
```

## Debug Mode

By default, the hook runs without `--debug`:

```json
"command": "${CLAUDE_PLUGIN_ROOT}/bin/ralph-hook-fmt"
```

Without `--debug`, continue responses are compact (`{"continue":true}`) and do not include `systemMessage`.

To see formatter diagnostics in `systemMessage` (for example formatter selected/skipped/error details), manually add `--debug` in `hooks.json`:

1. Open `~/.claude/plugins/ralph-hook-fmt/hooks/hooks.json`
2. Change the command from:
   ```json
   "command": "${CLAUDE_PLUGIN_ROOT}/bin/ralph-hook-fmt"
   ```
   to:
   ```json
   "command": "${CLAUDE_PLUGIN_ROOT}/bin/ralph-hook-fmt --debug"
   ```

## Development

```bash
make test
make lint
make fmt
make ci
```

## License

MIT
