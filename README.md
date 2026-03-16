<h1 align="center">piz</h1>

<p align="center">
  <strong>Intelligent terminal command assistant</strong><br>
  Translate natural language to shell commands with AI
</p>

<p align="center">
  <a href="https://github.com/AriesOxO/piz/actions"><img src="https://github.com/AriesOxO/piz/workflows/CI/badge.svg" alt="CI"></a>
  <a href="https://github.com/AriesOxO/piz/releases"><img src="https://img.shields.io/github/v/release/AriesOxO/piz" alt="Release"></a>
  <a href="https://github.com/AriesOxO/piz/blob/main/LICENSE"><img src="https://img.shields.io/github/license/AriesOxO/piz" alt="License"></a>
</p>

<p align="center">
  <a href="./README.md">English</a> |
  <a href="./README_ZH.md">简体中文</a>
</p>

---

## What is piz?

**piz** solves one problem: you know *what* you want to do, but not the exact command. Describe it in plain language, and piz translates it into the right shell command for your OS and shell.

```
$ piz list all files larger than 100MB
  ➜ find . -size +100M -type f
  [Y] Execute  [n] Cancel  [e] Edit
```

## Features

- **Natural Language to Command** - Describe what you want, get the exact command
- **Multi-Backend LLM** - OpenAI, Claude, Ollama, or any OpenAI-compatible API (DeepSeek, SiliconFlow, Moonshot, etc.)
- **Danger Detection** - Dual-layer protection: regex patterns + LLM classification. Dangerous commands require explicit confirmation
- **Command Explain** - Break down any command into its components with `piz -e`
- **Command Fix** - Auto-diagnose and fix failed commands with `piz fix`
- **Local Cache** - SQLite cache with TTL, repeated queries return instantly
- **Multi-Language UI** - Chinese, English, Japanese interface
- **Cross-Platform** - Windows (PowerShell/cmd), macOS, Linux (bash/zsh)

## Quick Start

### Install

**macOS / Linux (one-liner):**

```bash
curl -fsSL https://raw.githubusercontent.com/AriesOxO/piz/main/install.sh | bash
```

**Windows (PowerShell):**

```powershell
iwr -useb https://raw.githubusercontent.com/AriesOxO/piz/main/install.ps1 | iex
```

**Cargo (any platform):**

```bash
cargo install piz
```

**Manual download:**

Download binaries, `.msi` (Windows) or `.deb` (Debian/Ubuntu) from [Releases](https://github.com/AriesOxO/piz/releases).

| Platform | Downloads |
|----------|-----------|
| Windows x86_64 | `.msi` `.zip` |
| macOS x86_64 | `.tar.gz` |
| macOS ARM64 (Apple Silicon) | `.tar.gz` |
| Linux x86_64 | `.tar.gz` `.deb` |
| Linux ARM64 | `.tar.gz` |

### Setup

Run any command and the interactive setup wizard will start automatically:

```
$ piz list files
No configuration found. Let's set up piz for the first time.

  ⚙ piz configuration wizard

? Select language: 中文 / English / 日本語
? Select default LLM backend: openai (DeepSeek, Moonshot, SiliconFlow, ...)
? Select API provider: SiliconFlow (api.siliconflow.cn)
? API base URL: https://api.siliconflow.cn
? API key: sk-xxxxx
? Model name: Qwen/Qwen3-8B
? Auto-execute safe commands without confirmation? Yes

  ✔ Config saved
```

Or manually: `piz config --init`

## Usage

### Translate natural language

```bash
piz show disk usage                    # → df -h
piz find all rust files modified today # → find . -name "*.rs" -mtime 0
piz compress the src folder            # → tar -czf src.tar.gz src/
```

### Explain a command

```bash
$ piz -e 'tar -czf archive.tar.gz src/'
📖 Command explanation:

  tar  — tape archive tool
  -c   — create a new archive
  -z   — compress with gzip
  -f   — specify output filename
  src/ — directory to archive
```

### Fix failed commands

```bash
$ npm install
→ EACCES: permission denied...

$ piz fix
🔧 Diagnosis: Permission denied writing to global node_modules
  ➜ sudo npm install
```

### Other options

```bash
piz --backend ollama list files    # Use specific backend
piz --no-cache show memory         # Skip cache
piz clear-cache                    # Clear all cached commands
piz config --init                  # Re-run setup wizard
piz --version                     # Show version
```

## Configuration

Config file: `~/.piz/config.toml`

```toml
default_backend = "openai"
cache_ttl_hours = 168          # Cache TTL (7 days)
auto_confirm_safe = true       # Auto-execute safe commands
language = "zh"                # UI language: zh / en / ja

[openai]
api_key = "sk-your-key"
model = "gpt-4o-mini"
# base_url = "https://api.openai.com"    # Custom for third-party APIs

# [claude]
# api_key = "sk-ant-xxx"
# model = "claude-sonnet-4-20250514"
# base_url = "https://api.anthropic.com"

# [ollama]
# host = "http://localhost:11434"
# model = "llama3"
```

### Third-Party API Examples

<details>
<summary>DeepSeek</summary>

```toml
[openai]
api_key = "sk-your-deepseek-key"
model = "deepseek-chat"
base_url = "https://api.deepseek.com"
```
</details>

<details>
<summary>SiliconFlow</summary>

```toml
[openai]
api_key = "sk-your-key"
model = "Qwen/Qwen3-8B"
base_url = "https://api.siliconflow.cn"
```
</details>

<details>
<summary>Moonshot</summary>

```toml
[openai]
api_key = "sk-your-key"
model = "moonshot-v1-8k"
base_url = "https://api.moonshot.cn"
```
</details>

## Danger Detection

piz uses dual-layer danger detection to protect you:

| Level | Behavior | Example |
|-------|----------|---------|
| **Safe** | Auto-execute (if configured) | `ls`, `df -h`, `git status` |
| **Warning** | Prompt for confirmation | `sudo apt install`, `chmod 755`, `git push` |
| **Dangerous** | Red warning + explicit confirmation (cannot skip) | `rm -rf /`, `mkfs`, `DROP TABLE` |

Regex-based detection runs locally (no LLM needed) and catches patterns like `rm -rf /`, `mkfs`, `dd of=/dev/`, `FORMAT C:`, `DROP TABLE`, etc.

## Architecture

```
piz/
├── src/
│   ├── main.rs          # Entry point, CLI dispatch, response parsing
│   ├── cli.rs           # clap argument definitions
│   ├── config.rs        # TOML config + interactive setup wizard
│   ├── context.rs       # System context collection (OS, shell, cwd)
│   ├── i18n.rs          # Multi-language translations (zh/en/ja)
│   ├── llm/
│   │   ├── mod.rs       # LlmBackend trait + factory
│   │   ├── prompt.rs    # System prompt templates
│   │   ├── openai.rs    # OpenAI-compatible adapter
│   │   ├── claude.rs    # Claude adapter
│   │   └── ollama.rs    # Ollama adapter
│   ├── cache.rs         # SQLite cache with SHA256 keys + TTL
│   ├── danger.rs        # Regex-based danger detection
│   ├── executor.rs      # User confirmation + command execution
│   ├── explain.rs       # Command explain mode
│   ├── fix.rs           # Command fix mode
│   ├── history.rs       # Shell history reader
│   └── ui.rs            # Terminal output formatting
└── tests/
    └── integration.rs   # Integration tests
```

## Building from Source

```bash
# Prerequisites: Rust 1.70+
git clone https://github.com/AriesOxO/piz.git
cd piz

# Build
cargo build --release

# Run tests
cargo test

# Install to PATH
cargo install --path .
```

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
