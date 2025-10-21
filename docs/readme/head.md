![cull-gmail — Generate a change log based on the git commits compatible with keep-a-changelog and using conventional commits to categorize commits][splash]

[splash]: https://raw.githubusercontent.com/jerus-org/cull-gmail/main/assets/splash.svg

[![Rust 1.88+][version-badge]][version-url]
[![circleci-badge]][circleci-url]
[![Crates.io][crates-badge]][crates-url]
[![Docs][docs-badge]][docs-url]
[![MIT licensed][mit-badge]][mit-url]
[![BuyMeaCoffee][bmac-badge]][bmac-url]
[![GitHubSponsors][ghub-badge]][ghub-url]

[crates-badge]: https://img.shields.io/crates/v/cull-gmail.svg
[crates-url]: https://crates.io/crates/gen-changlog
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/jerusdp/cull-gmail/blob/main/LICENSE-MIT
[apache-badge]: https://img.shields.io/badge/license-APACHE-blue.svg
[apache-url]: https://github.com/jerusdp/cull-gmail/blob/main/LICENSE-APACHE
[circleci-badge]: https://dl.circleci.com/status-badge/img/gh/jerus-org/cull-gmail/tree/main.svg?style=svg
[circleci-url]: https://dl.circleci.com/status-badge/redirect/gh/jerus-org/cull-gmail/tree/main
[version-badge]: https://img.shields.io/badge/rust-1.88+-orange.svg
[version-url]: https://www.rust-lang.org
[docs-badge]:  https://docs.rs/cull-gmail/badge.svg
[docs-url]:  https://docs.rs/cull-gmail
[bmac-badge]: https://badgen.net/badge/icon/buymeacoffee?color=yellow&icon=buymeacoffee&label
[bmac-url]: https://buymeacoffee.com/jerusdp
[ghub-badge]: https://img.shields.io/badge/sponsor-30363D?logo=GitHub-Sponsors&logoColor=#white
[ghub-url]: https://github.com/sponsors/jerusdp

The `cull-gmail` provides a software library and command line program to enable the culling of emails using the Gmail API.

## Quick Start

Get started with cull-gmail in minutes using the built-in setup command:

1. **Get OAuth2 credentials** from [Google Cloud Console](https://console.cloud.google.com/)
2. **Initialize cull-gmail** with guided setup:
   ```bash
   # Interactive setup (recommended)
   cull-gmail init --interactive --credential-file ~/Downloads/client_secret.json
   
   # Or preview first
   cull-gmail init --dry-run
   ```
3. **Verify your setup**:
   ```bash
   cull-gmail labels
   ```

## Main Features

- **Easy initialization**: Guided setup with OAuth2 credential validation and secure file handling
- **Flexible configuration**: Support for file-based config, environment variables, and ephemeral tokens
- **Safety first**: Dry-run mode by default, interactive confirmations, and timestamped backups
- **Label management**: List and inspect Gmail labels for rule planning
- **Message operations**: Query, filter, and perform batch operations on Gmail messages  
- **Rule-based automation**: Configure retention rules with time-based filtering and automated actions
- **Token portability**: Export/import OAuth2 tokens for containerized and CI/CD environments

### Running the optional Gmail integration test

An optional, ignored integration test exercises the Gmail API end-to-end (networked). It is ignored by default and will not run in CI.

Steps to run locally:

1. Ensure you have valid OAuth client credentials configured for the library (see `ClientConfig::builder()` usage in docs).
2. Run the test explicitly with the ignored flag:

```bash
cargo test --test gmail_message_list_integration -- --ignored
```

Notes:
- The test performs a lightweight listing (max 10 messages) and should be safe, but it still uses your Gmail account.
- Do not run this in CI; it is intended only for local verification.

