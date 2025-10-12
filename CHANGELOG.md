<!-- LTex: Enabled=false -->
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.0.7] - 2025-10-12

Summary: Added[23], Build[1], Changed[8], Chore[4], Documentation[3], Fixed[10]

### Added

 - ✨ feat(processor): introduce processor builder
 - ✨ feat(cli): add execute option to processor
 - ✨ feat(processor): add execute flag for dry run
 - ✨ feat(cli): add execute flag to run action
 - ✨ feat(message_list): increase default max results
 - ✨ feat(core): introduce message processor module
 - ✨ feat(cli): add chrono crate as a dependency
 - ✨ feat(config): add eol query function
 - ✨ feat(processor): implement message deletion functionality
 - ✨ feat(cli): implement trash and delete actions
 - ✨ feat(processor): add trash and delete message functionality
 - ✨ feat(processor): add label existence check before processing
 - ✨ feat(config): add retention period to eol rule
 - ✨ feat(config): add date calculation for EOL queries
 - ✨ feat(cli): add option to skip trash actions
 - ✨ feat(cli): add skip-delete flag to cli
 - ✨ feat(cli): add skip action flags to cli
 - ✨ feat(cli): add run command to execute rules
 - ✨ feat(cli): add run cli command
 - ✨ feat(cli): implement rule execution logic
 - ✨ feat(eol_action): add parse method to EolAction
 - ✨ feat(eol_rule): add describe function for eol rule
 - ✨ feat(processor): implement rule processor

### Fixed

 - 🐛 fix(utils): correct string elision boundary calculation
 - 🐛 fix(utils): correct string elision boundary calculation
 - 🐛 fix(error): add specific error for missing label in mailbox - add `LableNotFoundInMailbox` error to handle cases where a label is not found in the mailbox
 - 🐛 fix(error): add error type for no query string calculated
 - 🐛 fix(processor): handle None query in eol_query
 - 🐛 fix(cli): correct count type in add_cli
 - 🐛 fix(message_age): correct data type for message age count
 - 🐛 fix(processor): execute delete messages
 - 🐛 fix(processor): correct typo in error message
 - 🐛 fix(error): correct spelling error in error message

### Changed

 - ♻️ refactor(utils): remove unused `get_start_boundary` function
 - ♻️ refactor(config): make EolRule public
 - ♻️ refactor(trash): separate trash preparation and execution
 - ♻️ refactor(trash): refactor trash command
 - ♻️ refactor(eol_rule): simplify eol_rule tests
 - ♻️ refactor(config): extract common logic to reduce duplication
 - ♻️ refactor(cli): extract action execution to separate function
 - ♻️ refactor(cli): inject config into run command

## [0.0.6] - 2025-10-09

Summary: Added[23], Changed[26], Chore[12], Fixed[7]

### Added

 - ✨ feat(cli): add delete subcommand
 - ✨ feat(cli): add delete subcommand
 - ✨ feat(gh-release): add delete module
 - ✨ feat(delete): implement batch delete functionality
 - ✨ feat(message_list): add label support
 - ✨ feat(message): add label support to message listing
 - ✨ feat(rules_cli): implement add command for managing retention rules
 - ✨ feat(cli): add remove label subcommand
 - ✨ feat(cli): add list labels subcommand
 - ✨ feat(label): implement add label command
 - ✨ feat(config): add functionality to set action on rule
 - ✨ feat(cli): add action subcommand
 - ✨ feat(config_cli): implement action subcommand
 - ✨ feat(config): add remove label from rule
 - ✨ feat(config): add label functionality to rules
 - ✨ feat(error): add RuleNotFound error
 - ✨ feat(config): add get_rule function to retrieve existing rules
 - ✨ feat(cli): implement commands dispatching
 - ✨ feat(label_cli): implement remove label subcommand
 - ✨ feat(label_cli): implement label listing subcommand
 - ✨ feat(label): implement add label subcommand
 - ✨ feat(cli): implement label subcommand
 - ✨ feat(config): add cli config - introduce cli config with clap - add subcommand rules and label

### Fixed

 - 🐛 fix(config): correct typo in eol_cmd module name
 - 🐛 fix(eol_rule): correct grammar in rule descriptions
 - 🐛 fix(config): correct grammar in EolRule display
 - 🐛 fix(remove_cli): handle rule not found when removing label
 - 🐛 fix(label_cli): fix add label logic
 - 🐛 fix(cli): correct output format for label list
 - 🐛 fix(label_cli): display labels by rule id

### Changed

 - ♻️ refactor(trash): encapsulate message list operations
 - ♻️ refactor(cli): improve delete command structure
 - ♻️ refactor(trash): encapsulate message list
 - ♻️ refactor(delete): rename struct and methods for deleting messages
 - ♻️ refactor(trash): streamline label handling in trash listing
 - ♻️ refactor(utils): improve config directory handling
 - ♻️ refactor(labels): simplify error handling in labels module
 - ♻️ refactor(trash): simplify error handling and label management
 - ♻️ refactor(cli): move rm_cli to new directory
 - ♻️ refactor(cli): move rules_cli to config_cli
 - ♻️ refactor(cli): rename label_cli module
 - ♻️ refactor(cli): rename action_cli module
 - ♻️ refactor(cli): rename trash_cli to cli
 - ♻️ refactor(cli): rename message_cli to cli
 - ♻️ refactor(cli): move label_cli to cli directory
 - ♻️ refactor(cli): move config_cli to cli directory
 - ♻️ refactor(cli): move main.rs to cli folder - move main.rs to cli folder for better structure
 - ♻️ refactor(project): move main.rs to cli directory
 - ♻️ refactor(cli): rename command to sub_command for clarity
 - ♻️ refactor(core): rename eol_cmd module to eol_action
 - ♻️ refactor(core): rename eol_cmd to eol_action - clarifies the file's purpose as defining actions related to EOL handling rather than just commands
 - ♻️ refactor(config): make EolRule fields public
 - ♻️ refactor(cli): restructure rules CLI
 - ♻️ refactor(cli): rename add_cli to rules_cli
 - ♻️ refactor(cli): rename rm_cli to rules_cli
 - ♻️ refactor(cli): consolidate rules and labels under config subcommand

## [0.0.5] - 2025-10-08

Summary: Added[28], Build[1], Changed[6], Chore[16], Documentation[5], Fixed[10]

### Added

 - ✨ feat(cli): implement trace logging for configuration
 - ✨ feat(rules_cli): implement rule removal
 - ✨ feat(lib): introduce Result type alias for error handling
 - ✨ feat(error): add custom error types for rule selection
 - ✨ feat(config): enhance rule management and label handling
 - ✨ feat(rules_cli): implement rm_cli subcommand
 - ✨ feat(rules_cli): add remove command to rules cli
 - ✨ feat(rules_cli): add option to immediately delete rules
 - ✨ feat(config): add delete flag for retention rules
 - ✨ feat(rules_cli): add optional label for retention rules
 - ✨ feat(config): add labels method to EolRule
 - ✨ feat(config): add support for labels to retention rules
 - ✨ feat(config): add retention attribute to EolRule
 - ✨ feat(config): enhance rule management with BTreeMap
 - ✨ feat(rules_cli): implement add command
 - ✨ feat(retention): add message age enum creation
 - ✨ feat(rules): add subcommand for rule management
 - ✨ feat(config): add result type to list_rules function
 - ✨ feat(config): implement display for eolrule struct
 - ✨ feat(config): add function to list rules
 - ✨ feat(config): implement configuration file management
 - ✨ feat(retention): introduce message age enum
 - ✨ feat(config): add EolRule struct for managing end-of-life rules
 - ✨ feat(retention): implement data retention policy
 - ✨ feat(cli): load configuration for message command
 - ✨ feat(lib): add config and retention modules
 - ✨ feat(eol_cmd): introduce EolCmd enum for message disposal
 - ✨ feat(build): add toml dependency

### Fixed

 - 🐛 fix(rm_cli): rule removal save
 - 🐛 fix(config): improve rule removal and logging
 - 🐛 fix(error): improve error message for missing labels
 - 🐛 fix(error): refine error message for rule selector
 - 🐛 fix(eol_rule): correct rule description in to_string method
 - 🐛 fix(rules): fix config_cli.run to return a Result
 - 🐛 fix(config): correct pluralization of time periods in EolRule display
 - 🐛 fix(message_age): correct retention label formatting
 - 🐛 fix(ui): correct grammar errors in eol command and trash messages
 - 🐛 fix(error): refine error handling with granular variants

### Changed

 - ♻️ refactor(config): use string keys for rules in config
 - ♻️ refactor(config): enhance EolRule for label management
 - ♻️ refactor(config): rename EolCmd to EolAction for clarity
 - ♻️ refactor(core): rename EolCmd to EolAction
 - ♻️ refactor(cli): restructure cli commands and config handling
 - ♻️ refactor(cli): rename config_cli to rules_cli

## [0.0.4] - 2025-10-07

Summary: Added[9], Changed[7], Chore[8]

### Added

 - ✨ feat(message_list): create message summary struct
 - ✨ feat(utils): implement string elision trait
 - ✨ feat(message_list): improve message handling and logging
 - ✨ feat(trash): implement trash functionality
 - ✨ feat(trash): add trash cli
 - ✨ feat(cli): add trash command
 - ✨ feat(message_list): enhance message list functionality and debugging
 - ✨ feat(lib): add trash module for  moving  messages to trash
 - ✨ feat(message_list): add message_ids to MessageList struct

### Changed

 - ♻️ refactor(trash): improve trash operation logging
 - ♻️ refactor(message): rename Message to MessageList
 - ♻️ refactor(core): rename message module to message_list
 - ♻️ refactor(message): rename message to message_list
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

[Unreleased]: https://github.com/jerus-org/cull-gmail/compare/v0.0.6...HEAD
[0.0.6]: https://github.com/jerus-org/cull-gmail/compare/v0.0.5...v0.0.6
[0.0.5]: https://github.com/jerus-org/cull-gmail/compare/v0.0.4...v0.0.5
[0.0.4]: https://github.com/jerus-org/cull-gmail/compare/v0.0.3...v0.0.4
[0.0.3]: https://github.com/jerus-org/cull-gmail/compare/v0.0.2...v0.0.3
[0.0.2]: https://github.com/jerus-org/cull-gmail/compare/v0.0.1...v0.0.2
[0.0.1]: https://github.com/jerus-org/cull-gmail/releases/tag/v0.0.1

