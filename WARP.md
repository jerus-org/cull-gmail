# WARP.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Development Commands

### Prerequisites

Install cargo-nextest (used for testing):
```bash
cargo install cargo-nextest --locked
```

### Build and Check
```bash
cargo build --workspace --all-targets
cargo check --workspace --all-targets
```

### Code Quality (matches CI)
```bash
# Format check
cargo fmt --all -- --check

# Linting
cargo clippy --workspace --all-targets

# Apply formatting
cargo fmt --all
```

### Testing
```bash
# Run all tests (CI profile)
cargo nextest run --profile ci

# Run all tests (default profile)
cargo nextest run

# Run a single test by name
cargo nextest run "client_config::tests::test_configuration_priority"

# Run tests matching a pattern
cargo nextest run -E 'test(rules::tests::)'

# Run ignored integration tests (requires Gmail API setup)
cargo test --test gmail_message_list_integration -- --ignored
```

### CLI Usage
```bash
# Show help
cargo run --bin cull-gmail -- --help

# List Gmail labels (safe, read-only)
cargo run --bin cull-gmail -- labels

# List messages (safe, read-only with dry-run default)
cargo run --bin cull-gmail -- messages -m 10 list

# Preview rules execution (dry-run mode)
cargo run --bin cull-gmail -- rules run

# Get subcommand help
cargo run --bin cull-gmail -- messages --help
cargo run --bin cull-gmail -- rules --help
```

## Architecture Overview

### Core Components

- **Gmail API Client** (`src/gmail_client.rs`): OAuth2-authenticated Gmail API client with label mapping
- **Rules Engine** (`src/rules.rs`, `src/rule_processor.rs`): Message retention rules with age-based filtering and actions (trash/delete)
- **CLI Interface** (`src/cli/main.rs`): Clap-based argument parsing with subcommands for labels, messages, rules, and token management
- **Configuration** (`src/client_config.rs`): OAuth2 credential management and app configuration loading

### Key Entry Points

- **Library**: `src/lib.rs` exports the main types (`GmailClient`, `Rules`, `ClientConfig`)
- **CLI Binary**: `src/cli/main.rs` provides the `cull-gmail` command-line interface
- **Message Processing**: `src/rule_processor.rs` implements the `RuleProcessor` trait for batch operations
- **Retention Logic**: `src/retention.rs` and `src/eol_action.rs` define message age criteria and actions

### Runtime and Error Handling

- **Async Runtime**: Uses Tokio for async operations with the Gmail API
- **Error Types**: Custom error enum in `src/error.rs` wrapping Gmail API, IO, and configuration errors
- **Logging**: Uses the `log` crate with configurable verbosity levels

## Gmail API Setup

### OAuth2 Configuration
1. Create OAuth2 credentials in Google Cloud Console
2. Download credential JSON file to `~/.cull-gmail/client_secret.json`
3. Create config file at `~/.cull-gmail/cull-gmail.toml`:
```toml
credential_file = "client_secret.json"
config_root = "~/.cull-gmail"
rules = "rules.toml"
execute = false  # Start in dry-run mode
```

### Environment Variables
Override config with `APP_` prefixed environment variables:
- `APP_CREDENTIAL_FILE`
- `APP_CLIENT_ID` / `APP_CLIENT_SECRET`
- `APP_EXECUTE`

### Token Storage
OAuth2 tokens are cached in `~/.cull-gmail/gmail1/` and automatically refreshed.

## Testing Strategy

### Test Organization
- **Unit Tests**: 98 tests across core modules (rules, client config, message processing)
- **Integration Tests**: 
  - `tests/cli_integration_tests.rs`: CLI command testing (22 tests)
  - `tests/gmail_client_unit_tests.rs`: Gmail client unit tests (3 tests)
  - `tests/gmail_message_list_integration.rs`: Live Gmail API test (ignored by default)

### Live API Testing
The Gmail integration test requires valid OAuth2 setup and is ignored by default:
```bash
# Run with valid credentials configured
cargo test --test gmail_message_list_integration -- --ignored
```

## CI Alignment

The CI pipeline (CircleCI) runs:
1. `cargo fmt --all -- --check` (formatting)
2. `cargo clippy --workspace --all-targets` (linting)
3. `cargo nextest run --profile ci` (testing with CI profile)
4. Security audit and documentation builds

Match CI locally by running the same commands in sequence.