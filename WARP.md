# WARP.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Core Development Commands

### Build and Test
- Build: `cargo build`
- Test: `cargo test`
- Format: `cargo fmt --all`
- Lint: `cargo clippy --all-targets --all-features -- -D warnings`
- CI test runner: `cargo nextest run --profile ci`

### CLI Development
- Install CLI locally: `cargo install --path .`
- Run CLI without installing: `cargo run --bin cull-gmail -- --help`
- Test a single test by pattern: `cargo test test_rules_new_creates_default_rules`

### Special Tests
- Run ignored Gmail API integration test: `cargo test --test gmail_message_list_integration -- --ignored`
  - Requires valid OAuth2 credentials configured
  - Only use with test Gmail account

## High-level Architecture

### Crates
- **Library**: `cull_gmail` - Core Gmail management functionality
- **Binary**: `cull-gmail` - CLI application

### Core Components
- **GmailClient**: Async Gmail API client with OAuth2 authentication, refresh token persistence, and Gmail API request helpers
- **Rules/EolRule**: Rule-based message lifecycle engine that loads from TOML configuration and evaluates messages to yield actions (trash/delete)
- **ClientConfig**: Configuration loader that merges TOML files with environment variables using `APP_` prefix
- **MessageList**: Abstraction over sets of Gmail message IDs with support for batch list, trash, and delete operations
- **CLI**: Command-line interface with subcommands for `labels`, `messages`, `rules`, and `token`; implements dry-run-first behavior for safety

### CLI Subcommands
- `labels` - List and inspect available Gmail labels
- `messages` - Query, filter, and perform batch operations on Gmail messages
- `rules` - Configure and execute automated message retention rules
- `token` - Export/import OAuth2 tokens for ephemeral environments

### Runtime
- **Async/await**: All network operations use Tokio async runtime
- **OAuth2**: Uses installed application flow with token caching to `~/.cull-gmail/gmail1/`
- **Safety**: Dry-run mode by default; destructive actions require explicit execution flags

## Configuration

### Directory Structure
- **Config directory**: `~/.cull-gmail/`
- **Main config**: `cull-gmail.toml` 
- **Rules config**: `rules.toml`
- **OAuth2 tokens**: `gmail1/` subdirectory

### Configuration Sources (in precedence order)
1. Environment variables with `APP_` prefix
2. TOML configuration files
3. Built-in defaults

### OAuth2 Setup Requirements
1. Google Cloud Console project with Gmail API enabled
2. OAuth2 desktop application credentials
3. Download credential JSON to config directory
4. First run triggers browser authentication flow

## Testing

### Test Organization
- **Unit tests**: Alongside modules in `src/`
- **Integration tests**: In `tests/` directory
- **CI runner**: Uses nextest with `ci` profile from `.config/nextest.toml`

### Test Categories
- Standard tests: `cargo test` or `cargo nextest run --profile ci`
- Gmail API integration test (ignored by default): Requires valid credentials and network access

### Important Notes
- **MSRV**: Rust 1.88+ (edition 2024)
- **Required runtime**: Tokio for all async operations
- **Gmail API**: Requires valid OAuth2 credentials for real testing
- **Logging**: Use `RUST_LOG=cull_gmail=debug` for detailed operation logs
- **Safety**: All operations default to dry-run mode; use explicit execution flags for actual changes