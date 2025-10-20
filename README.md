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

## Main Features

- login to get authorization
- backup the mailbox 
- filtered lists of the contents
- move email matching a filtered list to trash
## cull-gmail Library Documentation

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

The `cull-gmail` library provides types to enable the culling of emails using the Gmail API including the following steps:
- login to get authorization
- backup the mailbox 
- filtered lists of the contents
- move email matching a filtered list to trash

### Installation

Add the library to your program's `Cargo.toml` using `cargo add`:

```bash
$ cargo add cull-gmail
```

Or by configuring the dependencies manually in `Cargo.toml`:

```toml
[dependencies]
cull-gmail = "0.0.10"
```

## cull-gmail CLI

A command line program to cull emails from Gmail using the Gmail API. The tool has sub-commands to for authorization, planning and executing the move of select email to the Gmail trash folder from which they will be automatically deleted after thirty days. 

### Installation

Install cull-gmail using Cargo:

```bash
cargo install cull-gmail
```

## License

By contributing to cull-gmail, you agree that your contributions will be licensed under the MIT License. This means:

- You grant permission for your contributions to be used, modified, and distributed under the terms of the MIT License
- You confirm that you have the right to submit the code under this license
- You understand that your contributions will become part of the project and available to all users under the MIT License

## Contribution

Thank you for your interest in contributing to cull-gmail! We welcome contributions from the community and appreciate your help in making this project better.

Further details can be found in the [contribution document](CONTRIBUTING.md).
