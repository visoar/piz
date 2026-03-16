# Contributing to piz

Thank you for your interest in contributing to piz! This document provides guidelines and information for contributors.

## Getting Started

1. **Fork** the repository on GitHub
2. **Clone** your fork locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/piz.git
   cd piz
   ```
3. **Create a branch** for your changes:
   ```bash
   git checkout -b feat/your-feature-name
   ```
4. **Build and test**:
   ```bash
   cargo build
   cargo test
   ```

## Development Setup

### Prerequisites

- Rust 1.70 or later
- On Windows: MinGW-w64 toolchain (for `windows-gnu` target) or MSVC

### Build

```bash
cargo build          # Debug build
cargo build --release  # Release build
cargo test           # Run all tests
```

## How to Contribute

### Reporting Bugs

Open an [issue](https://github.com/AriesOxO/piz/issues/new) with:
- piz version (`piz --version`)
- OS and shell
- Steps to reproduce
- Expected vs actual behavior

### Suggesting Features

Open an [issue](https://github.com/AriesOxO/piz/issues/new) describing:
- The problem you're trying to solve
- Your proposed solution
- Any alternatives you considered

### Pull Requests

1. Ensure your code builds without warnings: `cargo build`
2. All tests pass: `cargo test`
3. Format your code: `cargo fmt`
4. Run clippy: `cargo clippy`
5. Write tests for new functionality
6. Keep commits focused — one logical change per commit

#### Commit Message Format

```
<type>: <short description>

<optional body>
```

Types: `feat`, `fix`, `refactor`, `test`, `docs`, `chore`

Examples:
- `feat: add Gemini backend support`
- `fix: handle empty LLM response gracefully`
- `docs: add SiliconFlow config example`

### Areas for Contribution

- **New LLM backends** — Add support for more providers
- **Danger patterns** — Expand regex detection rules
- **i18n** — Add new languages or improve translations
- **Platform support** — Improve Windows/macOS compatibility
- **Tests** — Increase coverage, especially edge cases
- **Documentation** — Improve README, add examples

## Project Structure

```
src/
├── main.rs          # Entry point, CLI dispatch
├── cli.rs           # clap argument definitions
├── config.rs        # Config loading + setup wizard
├── context.rs       # System context (OS, shell, cwd)
├── i18n.rs          # UI translations
├── llm/
│   ├── mod.rs       # LlmBackend trait
│   ├── prompt.rs    # Prompt templates
│   ├── openai.rs    # OpenAI adapter
│   ├── claude.rs    # Claude adapter
│   └── ollama.rs    # Ollama adapter
├── cache.rs         # SQLite cache
├── danger.rs        # Danger detection
├── executor.rs      # Command execution
├── explain.rs       # Explain mode
├── fix.rs           # Fix mode
├── history.rs       # Shell history
└── ui.rs            # Terminal output
```

### Adding a New LLM Backend

1. Create `src/llm/your_backend.rs`
2. Implement the `LlmBackend` trait
3. Add config struct in `config.rs`
4. Register in factory function `create_backend()` in `src/llm/mod.rs`
5. Add setup flow in `config.rs` init wizard
6. Write tests

### Adding a New Language

1. Add a variant to `Lang` enum in `src/i18n.rs`
2. Create a new `static` translation table
3. Add the match arm in `t()` function
4. Update the language selector in `config.rs`

## Code of Conduct

- Be respectful and constructive
- Focus on the code, not the person
- Welcome newcomers and help them get started

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
