# Changelog

## [0.4.2](https://github.com/chenhunghan/ralph-hook-fmt/compare/ralph-hook-fmt-v0.4.1...ralph-hook-fmt-v0.4.2) (2026-02-06)


### Bug Fixes

* skip formatting package.json ([86c925a](https://github.com/chenhunghan/ralph-hook-fmt/commit/86c925a866a1df717d5a6cf5b2b9676277b159c0))

## [0.4.1](https://github.com/chenhunghan/ralph-hook-fmt/compare/ralph-hook-fmt-v0.4.0...ralph-hook-fmt-v0.4.1) (2026-01-31)


### Bug Fixes

* fix auto update in setup.sh ([4105823](https://github.com/chenhunghan/ralph-hook-fmt/commit/410582370f7ac1b520f3824e2ede22238131e580))

## [0.4.0](https://github.com/chenhunghan/ralph-hook-fmt/compare/ralph-hook-fmt-v0.3.0...ralph-hook-fmt-v0.4.0) (2026-01-31)


### Features

* prioritize oxfmt in JavaScript/TypeScript formatting and update tests ([e1b746b](https://github.com/chenhunghan/ralph-hook-fmt/commit/e1b746baefe0f99c171726d10355a65ff2cafd96))

## [0.3.0](https://github.com/chenhunghan/ralph-hook-fmt/compare/ralph-hook-fmt-v0.2.0...ralph-hook-fmt-v0.3.0) (2026-01-31)


### Features

* improve go formatter selection order ([a792e89](https://github.com/chenhunghan/ralph-hook-fmt/commit/a792e89abd293bd8195d55a4ed07629a2ef0e285))


### Bug Fixes

* fix plugin marketplace/plugin.json format ([10131fd](https://github.com/chenhunghan/ralph-hook-fmt/commit/10131fd88814cdfe2f6a4cf644e3a8c54a284e45))

## [0.2.0](https://github.com/chenhunghan/ralph-hook-fmt/compare/ralph-hook-fmt-v0.1.0...ralph-hook-fmt-v0.2.0) (2026-01-31)


### Features

* init impl of ralph-hook-fmt ([315f1d4](https://github.com/chenhunghan/ralph-hook-fmt/commit/315f1d43052418577ec43dc9179b435d233e5367))

## [0.1.0](https://github.com/chenhunghan/ralph-hook-fmt/releases/tag/v0.1.0) (Unreleased)

### Features

* Initial release
* Support for JavaScript/TypeScript formatting (biome, prettier, dprint)
* Support for Rust formatting (rustfmt, cargo fmt)
* Support for Python formatting (ruff, black, autopep8, yapf)
* Support for Java formatting (spotless, google-java-format, palantir-java-format)
* Support for Go formatting (goimports, gofmt)
* Automatic formatter detection based on project configuration
* Claude Code hook integration for PostToolUse events
