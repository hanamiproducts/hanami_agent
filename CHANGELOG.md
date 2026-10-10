# Changelog

Kuzey Agent follows [Semantic Versioning](https://semver.org). The version lives in
`kuzey_agent/Cargo.toml`, and every release is tagged in git as `vX.Y.Z`.

While the version is `0.x`, anything can still change between minor versions.

## [0.1.0] - 2026-10-09

First version.

### Added

- Terminal chat with a provider and model chosen at start-up. API keys are typed in masked and
  never stored.
- Tool calling, up to 10 rounds per message.
- Providers: Google Gemini, Anthropic Claude, OpenAI (Responses API), Ollama, DeepSeek and
  GLM (Z.ai).
- Tools: `get_current_time`, `read_file`, `write_file`, `run_command`.
- Approval prompt before `write_file` and `run_command` run.
- `@path/to/file` mentions with path autocomplete.
- 120 s request timeout and readable provider error messages.
- CI: build, test, clippy and fmt.

[0.1.0]: https://github.com/samliumay/kuzey_agent/releases/tag/v0.1.0
