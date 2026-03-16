# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/), and this project adheres to [Semantic Versioning](https://semver.org/).

## [0.2.0] - 2026-03-16

### Added
- Interactive configuration wizard with provider presets (OpenAI, DeepSeek, SiliconFlow, Moonshot)
- Multi-language UI support: Chinese (zh), English (en), Japanese (ja)
- `--version` / `-V` flag for version display
- Claude backend `base_url` support for third-party proxies
- Auto-trigger setup wizard on first use
- Config overwrite confirmation
- Config validation after creation
- Prompt optimization: few-shot examples, shell-specific syntax hints, explicit language directives
- 110 tests (102 unit + 8 integration)

### Changed
- `auto_confirm_safe` only affects safe commands; dangerous commands always require explicit confirmation
- Prompts now specify response language explicitly instead of "follow user input"

## [0.1.0] - 2026-03-16

### Added
- Core natural language to shell command translation
- 4-level LLM response parsing fallback (JSON → embedded JSON → backtick → raw text)
- Multi-backend LLM support: OpenAI (with custom base_url), Claude, Ollama
- Dual danger detection: regex patterns + LLM classification
- Three danger levels: safe, warning, dangerous
- Command explain mode (`piz -e`)
- Command fix mode (`piz fix`) with last_exec.json + shell history fallback
- SQLite cache with SHA256 key and configurable TTL
- Interactive confirmation UI (Y/n/e) with editor support
- TOML configuration file (`~/.piz/config.toml`)
- System context injection (OS, shell, cwd) into prompts
- Cross-platform support: Windows (PowerShell/cmd), macOS, Linux (bash/zsh)
