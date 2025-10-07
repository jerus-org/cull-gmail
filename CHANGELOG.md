<!-- LTex: Enabled=false -->
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.0.4] - 2025-10-07

Summary: Added[9], Changed[7], Chore[7]

### Added

 - ✨ feat(message_list): create message summary struct
 - ✨ feat(message_list): improve message handling and logging
 - ✨ feat(utils): implement string elision trait
 - ✨ feat(trash): implement trash functionality
 - ✨ feat(trash): add trash cli
 - ✨ feat(cli): add trash command
 - ✨ feat(message_list): enhance message list functionality and debugging
 - ✨ feat(lib): add trash module for  moving  messages to trash
 - ✨ feat(message_list): add message_ids to MessageList struct

### Changed

 - ♻️ refactor(trash): improve trash operation logging
 - ♻️ refactor(message): rename message to message_list
 - ♻️ refactor(message): rename Message to MessageList
 - ♻️ refactor(core): rename message module to message_list
 - ♻️ refactor(labels): remove unused code
 - ♻️ refactor(labels): improve label listing and mapping
 - ♻️ refactor(message): improve subject logging with early returns

## [0.0.3] - 2025-10-04

Summary: Added[7], Changed[6], Chore[5], Fixed[1]

### Added

 - ✨ feat(message): implement message listing functionality
 - ✨ feat(cli): add label listing subcommand
 - ✨ feat(labels): add show option to display labels
 - ✨ feat(cli): add label command-line interface
 - ✨ feat(cli): add query option to list command
 - ✨ feat(list): add query support to list messages - allow users to filter messages using a query string - implement set_query method to set the query - add query parameter to the Gmail API call
 - ✨ feat(list): add label filtering to list command

### Fixed

 - 🐛 fix(list): fix label creation logic

### Changed

 - ♻️ refactor(cli): rename list subcommand to message
 - ♻️ refactor(cli): rename list_cli to message_cli
 - 🔥 refactor(core): remove list module
 - ♻️ refactor(core): rename list module to message
 - ♻️ refactor(labels): simplify labels struct initialization
 - ♻️ refactor(labels): simplify and optimize label retrieval - rename function name `add_label` to `add_labels` - add the function of adding multiple labels at once - optimize code for streamlined operation

## [0.0.2] - 2025-10-03

Summary: Added[26], Build[6], Changed[6], Chore[17], Continuous Integration[1], Documentation[1], Fixed[3], Security[1]

### Added

 - ✨ feat(list): add label filtering to list command
 - ✨ feat(list): add label filtering capability
 - ✨ feat(core): add Labels struct
 - ✨ feat(labels): create labels module to manage Gmail labels
 - ✨ feat(list): add pagination to list command
 - ✨ feat(list): add pagination support for listing messages
 - ✨ feat(error): add error type for invalid paging mode
 - ✨ feat(list): add max results option to list command
 - ✨ feat(list): export DEFAULT_MAX_RESULTS constant
 - ✨ feat(error): enhance error handling for configuration issues
 - ✨ feat(core): add utils module
 - ✨ feat(utils): create assure_config_dir_exists function
 - ✨ feat(gmail): implement list functionality for Gmail API
 - ✨ feat(lib): add error module and export it
 - ✨ feat(error): introduce custom error enum for cull-gmail
 - ✨ feat(list): implement list api to retrieve gmail messages
 - ✨ feat(list): integrate List struct for message listing
 - ✨ feat(list): export List struct in lib.rs
 - ✨ feat(cli): add list subcommand
 - ✨ feat(core): add client and credential modules
 - ✨ feat(list): add list module - creates a new list module
 - ✨ feat(credential): implement credential loading and conversion
 - ✨ feat(gmail): add gmail client
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
 - ♻️ refactor(list): improve error handling and config loading
 - ♻️ refactor(list): refactor list command to accept credential file
 - ♻️ refactor(main): improve error handling and logging

### Security

 - 🔧 chore(deps): remove unused dependencies

## [0.0.1] - 2025-09-30

Summary: Added[4], Build[3], Chore[21], Continuous Integration[4], Documentation[7]

### Added

 - ✨ feat(lib): add addition function with test
 - ✨ feat(assets): add new logo and splash screen
 - ✨ feat(vscode): add custom dictionary entry for ltex
 - ✨ feat(project): add initial Cargo.toml for cull-gmail tool

[Unreleased]: https://github.com/jerus-org/cull-gmail/compare/v0.0.3...HEAD
[0.0.3]: https://github.com/jerus-org/cull-gmail/compare/v0.0.2...v0.0.3
[0.0.2]: https://github.com/jerus-org/cull-gmail/compare/v0.0.1...v0.0.2
[0.0.1]: https://github.com/jerus-org/cull-gmail/releases/tag/v0.0.1

