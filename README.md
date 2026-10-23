# ✿ Hanami Agent

[![CI](https://github.com/samliumay/hanami_agent/actions/workflows/ci.yml/badge.svg)](https://github.com/samliumay/hanami_agent/actions/workflows/ci.yml)
![version](https://img.shields.io/badge/version-0.1.0-blue)

A terminal AI agent written from scratch in Rust. You chat with a model, and it can use tools on
your machine (read and write files, run shell commands) to get things done.

> **Done with curiosity and Rust.** This is a learning project. The code is full of notes and open
> questions, and it is not hardened. PRs are very welcome.

## Features

- Pick a provider and model in the terminal. API keys are typed in masked and never stored.
- Multi-turn chat with tool calling (up to 10 tool rounds per message).
- `@path/to/file` in a message attaches that file, with path autocomplete.
- `write_file` and `run_command` ask for your approval before they run.
- Minimal by design: no web search tool. The model can use `curl` through `run_command` when it
  needs the web.

## Providers

| Provider                          | Status          |
| --------------------------------- | --------------- |
| Google Gemini                     | ✅ chat + tools |
| Anthropic Claude                  | ✅ chat + tools |
| OpenAI (Responses API)            | ✅ chat + tools |
| Ollama (local, `localhost:11434`) | ✅ chat + tools |
| DeepSeek                          | ✅ chat + tools |
| GLM (Z.ai)                        | ✅ chat + tools |

## Tools

| Tool               | What it does                                    |
| ------------------ | ----------------------------------------------- |
| `get_current_time` | Returns the local date and time                 |
| `read_file`        | Reads a text file                               |
| `write_file`       | Creates or overwrites a file (asks first)       |
| `run_command`      | Runs a shell command, 30 s timeout (asks first) |

## Quick start

Needs Rust 1.85 or newer ([rustup](https://rustup.rs)).

```bash
git clone https://github.com/samliumay/hanami_agent.git
cd hanami_agent/hanami_agent
cargo run --release
```

Choose a provider, enter your API key (not needed for Ollama), choose a model and start chatting.
Lines starting with `[tool]` show the tools the model runs. Type `/exit` to quit.

`run_command` uses `sh`, so it works on Linux and macOS but not on Windows yet. Even with the
approval prompts, it's best to run Hanami Agent in a folder you don't mind changing.

Requests to providers time out after 120 s. GLM uses the standard Z.ai endpoint; GLM Coding Plan
keys need the `api/coding/paas/v4` URL in `src/providers/glm.rs`.

## Branches

- `main` is always the usable version. Clone this one.
- `dev` is where new work is tested before it is merged into `main`.

## Roadmap

- [x] Basic function calls and the base of the project.
- [x] Chat loop, tool calling, approval prompts, `@file` mentions
- [x] Gemini, Claude, OpenAI, Ollama, DeepSeek and GLM providers
- [x] `edit_file` tool
- [ ] API keys from environment variables
- [ ] Windows support, streaming responses, better tests
