<!-- LTex: Enabled=false -->
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.0.2] - 2025-10-03

Summary: Added[26], Build[6], Changed[6], Chore[16], Continuous Integration[1], Documentation[1], Fixed[3], Security[1]

### Added

 - ✨ feat(list): add label filtering to list command
 - ✨ feat(core): add Labels struct
 - ✨ feat(labels): create labels module to manage Gmail labels
 - ✨ feat(list): add label filtering capability
 - ✨ feat(list): add pagination support for listing messages
 - ✨ feat(error): add error type for invalid paging mode
 - ✨ feat(list): add pagination to list command
 - ✨ feat(list): add max results option to list command
 - ✨ feat(list): export DEFAULT_MAX_RESULTS constant
 - ✨ feat(error): enhance error handling for configuration issues
 - ✨ feat(core): add utils module
 - ✨ feat(utils): create assure_config_dir_exists function
 - ✨ feat(error): introduce custom error enum for cull-gmail
 - ✨ feat(gmail): implement list functionality for Gmail API
 - ✨ feat(lib): add error module and export it
 - ✨ feat(list): implement list api to retrieve gmail messages
 - ✨ feat(list): integrate List struct for message listing
 - ✨ feat(list): export List struct in lib.rs
 - ✨ feat(credential): implement credential loading and conversion
 - ✨ feat(core): add client and credential modules
 - ✨ feat(cli): add list subcommand
 - ✨ feat(gmail): add gmail client
 - ✨ feat(list): add list module - creates a new list module
 - ✨ feat(cli): implement list subcommand
 - ✨ feat(cli): add command line interface with logging
 - ✨ feat(main): add initial main function with hello world

### Fixed

 - 🐛 fix(main): exit process with error code on failure
 - 🐛 fix(list): remove debug print statement
 - 🐛 fix(credential): fix the config directory

### Changed

 - ♻️ refactor(list): improve max results handling
 - ♻️ refactor(gmail): remove unused client file
 - ♻️ refactor(lib): restructure module exports and visibility
 - ♻️ refactor(main): improve error handling and logging
 - ♻️ refactor(list): improve error handling and config loading
 - ♻️ refactor(list): refactor list command to accept credential file

### Security

 - 🔧 chore(deps): remove unused dependencies

## [0.0.1] - 2025-09-30

Summary: Added[4], Build[3], Chore[21], Continuous Integration[4], Documentation[7]

### Added

 - ✨ feat(lib): add addition function with test
 - ✨ feat(assets): add new logo and splash screen
 - ✨ feat(vscode): add custom dictionary entry for ltex
 - ✨ feat(project): add initial Cargo.toml for cull-gmail tool

[Unreleased]: https://github.com/jerus-org/cull-gmail/compare/v0.0.1...HEAD
[0.0.1]: https://github.com/jerus-org/cull-gmail/releases/tag/v0.0.1

