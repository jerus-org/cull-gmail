<!-- LTex: Enabled=false -->
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.1] - 2026-02-13

Summary: Chore[1], Continuous Integration[2]

## [0.1.0] - 2026-02-13

Summary: Added[6], Build[1], Changed[1], Chore[50], Continuous Integration[12], Fixed[62]

### Added

 - feat: add security improvements to CI (#142)
 - feat: add security improvements to CI
 - feat!: migrate to circleci-toolkit v4.2.1
 - ✨ feat(cli): restructure rules config CLI
 - ✨ feat(cli): enhance rules configuration
 - ✨ feat(cli): add optional rules path argument to cli

### Fixed

 - fix(deps): update rust crate toml to v1 (#153)
 - fix(deps): update rust crate toml to v1
 - fix(deps): update rust crate tempfile to 3.25.0 (#152)
 - fix(deps): update rust crate tempfile to 3.25.0
 - fix(deps): update rust crate lazy-regex to 3.6.0 (#151)
 - fix(deps): update rust crate lazy-regex to 3.6.0
 - fix(deps): update dependency toolkit to v4.4.2 (#150)
 - fix(deps): update dependency toolkit to v4.4.2
 - fix(deps): update rust crate toml to 0.9.12 (#149)
 - fix(deps): update rust crate toml to 0.9.12
 - fix(deps): update rust crate predicates to 3.1.4 (#148)
 - fix(deps): update rust crate predicates to 3.1.4
 - fix(deps): update rust crate httpmock to 0.8.3 (#147)
 - fix(deps): update rust crate httpmock to 0.8.3
 - fix(deps): update rust crate clap to 4.5.58 (#144)
 - fix(deps): update rust crate clap to 4.5.58
 - fix(deps): update rust crate env_logger to 0.11.9 (#145)
 - fix(deps): update rust crate env_logger to 0.11.9
 - fix(deps): update rust crate flate2 to 1.1.9 (#146)
 - fix(deps): update rust crate flate2 to 1.1.9
 - fix(deps): resolve rustls crypto provider conflict (#143)
 - fix(deps): resolve rustls crypto provider conflict
 - fix(deps): update bytes and time for security
 - fix(deps): update rust crate thiserror to 2.0.18 (#141)
 - fix(deps): update rust crate thiserror to 2.0.18
 - fix(deps): update rust crate lazy-regex to 3.5.1 (#140)
 - fix(deps): update rust crate lazy-regex to 3.5.1
 - fix(deps): update rust crate hyper-rustls to 0.27.7 (#139)
 - fix(deps): update rust crate hyper-rustls to 0.27.7
 - fix(deps): update rust crate flate2 to 1.1.8 (#138)
 - fix(deps): update rust crate flate2 to 1.1.8
 - fix(deps): update rust crate clap to 4.5.54 (#137)
 - fix(deps): update rust crate clap to 4.5.54
 - fix(deps): update rust crate chrono to 0.4.43 (#136)
 - fix(deps): update rust crate chrono to 0.4.43
 - fix(deps): update tokio packages (#132)
 - fix(deps): update tokio packages
 - fix(deps): update rust crate tempfile to 3.24.0 (#131)
 - fix(deps): update rust crate tempfile to 3.24.0
 - fix(deps): update rust crate dialoguer to 0.12.0 (#130)
 - fix(deps): update rust crate dialoguer to 0.12.0
 - fix(deps): update rust crate toml to 0.9.11 (#127)
 - fix(deps): update rust crate toml to 0.9.11
 - fix(deps): update rust crate assert_cmd to 2.1.2 (#129)
 - fix(deps): update rust crate assert_cmd to 2.1.2
 - fix(deps): update rust crate serde_json to 1.0.149 (#126)
 - fix(deps): update rust crate serde_json to 1.0.149
 - fix(deps): update rust crate log to 0.4.29 (#125)
 - fix(deps): update rust crate log to 0.4.29
 - fix: replace deprecated cargo_bin method with macro
 - fix: upgrade google-gmail1 to v7 to resolve security advisory
 - fix(deps): update rust crate temp-env to 0.3.6
 - fix(deps): update rust crate predicates to 3.1.3
 - fix(deps): update rust crate lazy-regex to 3.4.2
 - fix(deps): update rust crate httpmock to 0.8.2
 - fix(deps): update rust crate futures to 0.3.31
 - fix(deps): update rust crate flate2 to 1.1.5
 - fix(deps): update rust crate config to 0.15.19
 - fix(deps): update rust crate clap to 4.5.53
 - fix(deps): update rust crate base64 to 0.22.1
 - fix(deps): update rust crate assert_fs to 1.1.3
 - 🐛 fix(client): fix config root parsing

### Changed

 - ♻️ refactor(cli): rename rule management subcommands for clarity

## [0.0.16] - 2025-10-30

Summary: Added[3], Changed[4], Chore[10], Documentation[2], Fixed[7]

### Added

 - ✨ feat(rules): support multiple actions per label
 - ✨ feat(rule_processor): implement batch operations for message deletion and trashing
 - ✨ feat(rule_processor): add initialise_message_list to processor

### Fixed

 - 🐛 fix(rule_processor): enhance logging for chunk processing
 - 🐛 fix(cli): correct rule execution order for trash and delete
 - 🐛 fix(gmail): use GMAIL_DELETE_SCOPE for batch delete
 - 🐛 fix(cli): correct logging level
 - 🐛 fix(eol_rule): correct calculate_for_date and add logging
 - 🐛 fix(rules): correct grammar and improve date calculation
 - 🐛 fix(gmail): handle batch delete errors

### Changed

 - ♻️ refactor(rules): execute rules by action
 - ♻️ refactor(gmail): consolidate batch processing logic
 - ♻️ refactor(rule_processor): enhance Gmail message handling with chunk processing
 - ♻️ refactor(core): rename initialise_message_list to initialise_lists

## [0.0.15] - 2025-10-26

Summary: Changed[1], Chore[2], Documentation[1], Fixed[3]

### Fixed

 - 🐛 fix(cli): fix log messages with empty arguments
 - 🐛 fix(cli): prevent dry-run from crashing
 - 🐛 fix(rule_processor): fix batch_trash and batch_delete signatures

### Changed

 - ♻️ refactor(message_list): allow pre/post text in log_messages

## [0.0.14] - 2025-10-23

Summary: Added[2], Chore[7], Fixed[2]

### Added

 - ✨ feat(cli): add token and auth uri config options
 - ✨ feat(config): load application secret with logging

### Fixed

 - 🐛 fix(config): reduce log verbosity
 - 🐛 fix(config): improve config logging format

## [0.0.13] - 2025-10-22

Summary: Added[1], Chore[2], Fixed[5]

### Added

 - ✨ feat(cli): enhance configuration loading with logging

### Fixed

 - 🐛 fix(cli): load config file only if it exists
 - 🐛 fix(cli): fix config file loading
 - 🐛 fix(client_config): print config for debugging
 - 🐛 fix(cli): correct spelling errors in documentation
 - 🐛 fix(cli): load config file as optional

## [0.0.12] - 2025-10-22

Summary: Added[6], Build[1], Changed[2], Chore[7], Documentation[1], Fixed[6], Testing[2]

### Added

 - ✨ feat: integrate configurable rules path throughout CLI
 - ✨ feat: add get_rules_from() to load rules from custom path
 - ✨ feat: add configurable rules directory support to Rules and InitCli
 - 🏗️ feat(init): implement plan and apply operations
 - ✨ feat(cli): scaffold InitCli subcommand and clap wiring
 - 🔐 feat: Add token export/import for ephemeral environments

### Fixed

 - 🐛 fix(ci): correct default test runner value
 - 🔧 fix: address clippy warnings after refactoring
 - 🐛 fix: allow init command to run without existing config file
 - 🐛 fix: replace hardcoded paths in tests with temp directories for CI compatibility
 - 🔧 fix: address clippy warnings and improve code formatting
 - 🔧 fix: Resolve clippy warnings and formatting issues

### Changed

 - ♻️ refactor: reduce cognitive complexity of plan_operations and execute_operation
 - ♻️ refactor: extract mock credential file creation into helper function

## [0.0.11] - 2025-10-20

Summary: Added[7], Changed[7], Chore[13], Continuous Integration[5], Documentation[24], Fixed[7], Testing[12]

### Added

 - ✨ feat(test): add junit report
 - ✨ feat(ci): introduce nextest test runner
 - ✨ feat(retention): enhance message age with parsing and validation
 - ✨ feat(retention): implement retention policy configuration
 - ✨ feat(error): add invalid message age error
 - ✨ feat(retention): introduce message age specification
 - ✨ feat(retention): enhance retention policy configuration

### Fixed

 - 🐛 fix(rule_processor): correct spelling of "behaviour"
 - ✅ fix(message-list): improve idioms (avoid redundant clone, extend labels, safer message extraction)
 - ✅ fix(clippy): move tests module to file end to satisfy items_after_test_module lint
 - 🐛 fix(retention): fix debug string formatting in retention struct
 - 🐛 fix(cli): correct error mapping in add_cli
 - 🐛 fix(rules): handle message age creation error
 - 🐛 fix(build): correct readme generation script

### Changed

 - ♻️ refactor: remove redundant credential module
 - ♻️ refactor(message-list): introduce GmailService abstraction and refactor to use it; fix borrows and lifetimes
 - ♻️ refactor(message-list): extract helper to append messages from ListMessagesResponse and add unit test
 - ♻️ refactor(rule_processor): extract process_label and add internal ops trait for unit testing
 - ♻️ refactor(rule_processor): add TRASH_LABEL, correct Gmail scopes, early returns, and improve idioms
 - refactor(rules): apply idiomatic patterns and resolve clippy warnings
 - refactor(rules): replace unwrap() with explicit error handling and propagate errors safely

## [0.0.10] - 2025-10-16

Summary: Added[11], Changed[15], Chore[12], Fixed[3]

### Added

 - ✨ feat(cli): add default subcommand for rule execution
 - ✨ feat(config): implement config builder pattern for ClientConfig
 - ✨ feat(cli): load configurations from toml file
 - ✨ feat(client_config): add config root parsing with regex
 - ✨ feat(utils): add test utils module
 - ✨ feat(deps): add lazy-regex crate
 - ✨ feat(dependencies): add lazy-regex dependency
 - ✨ feat(config): add ConfigRoot enum for flexible path handling
 - ✨ feat(core): add client config
 - ✨ feat(config): introduce client configuration
 - ✨ feat(cli): add config file support

### Fixed

 - 🐛 fix(gmail): fix token persistence path
 - 🐛 fix(config): resolve credential file path issue
 - 🐛 fix(rule_processor): update Gmail API scope

### Changed

 - ♻️ refactor(cli): extract action execution into a function
 - ♻️ refactor(cli): rename get_config to get_rules
 - ♻️ refactor(cli): extract rule execution to separate function
 - ♻️ refactor(config): improve ConfigRoot to handle different root types
 - ♻️ refactor(utils): improve config directory creation
 - ♻️ refactor(cli): use ClientConfig struct for gmail client
 - ♻️ refactor(gmail): use client config for gmail client
 - ♻️ refactor(rules): remove credentials config
 - ♻️ refactor(cli): remove config from run args
 - ♻️ refactor(eol_rule): improve labels handling
 - ♻️ refactor(cli): remove redundant Rules import
 - ♻️ refactor: rename Config to Rules
 - ♻️ refactor(cli): restructure cli commands for better organization
 - ♻️ refactor(message_list): rename messages_list to list_messages
 - ♻️ refactor(rule_processor): remove unused delete functions

## [0.0.9] - 2025-10-14

Summary: Added[5], Changed[3], Chore[2], Fixed[2]

### Added

 - ✨ feat(gmail_client): add date to message summary
 - ✨ feat(gmail): enhance message metadata retrieval
 - ✨ feat(cli): enhance cli subcommand ordering and grouping
 - ✨ feat(cli): add message list subcommand
 - ✨ feat(cli): add configuration options for message listing

### Fixed

 - 🐛 fix(gmail_client): resolve ownership issue in message summary
 - 🐛 fix(gmail): display message date and subject

### Changed

 - ♻️ refactor(cli): rename run_cli to rules_cli
 - ♻️ refactor(cli): consolidate message handling and remove delete command
 - ♻️ refactor(cli): refactor message handling and remove trash command

## [0.0.8] - 2025-10-14

Summary: Added[14], Changed[42], Chore[3], Documentation[2], Fixed[5]

### Added

 - ✨ feat(cli): create message trait to share list parameters
 - ✨ feat(cli): add message trait for cli subcommands
 - ✨ feat(cli): implement batch actions for trashing and deleting
 - ✨ feat(rule_processor): implement rule processing for Gmail
 - ✨ feat(gmail_client): add execute flag and EolRule
 - ✨ feat(processor): add execute flag to GmailClient
 - ✨ feat(gmail_client): add rule field to GmailClient struct - Add rule field to GmailClient struct to store EolAction.
 - ✨ feat(eol_action): add clone derive to eolaction enum
 - ✨ feat(message_list): enhance message list trait with documentation and functionalities
 - ✨ feat(core): add message management structs
 - ✨ feat(gmail_client): integrate message summary
 - ✨ feat(gmail): create gmail client struct
 - ✨ feat(gmail): add get messages functionality
 - ✨ feat(error): add NoLabelsFound error

### Fixed

 - 🐛 fix(cli): correct label adding to use non-async function
 - 🐛 fix(rule_processor): fix label creation and message retrieval
 - 🐛 fix(cli): fix rule execution and client handling
 - 🐛 fix(trash): fix trash command with new gmail client
 - 🐛 fix(cli): fix delete command

### Changed

 - ♻️ refactor(cli): streamline message retrieval and parameter setting
 - ♻️ refactor(cli): extract parameter setting logic
 - ♻️ refactor(message_list): rename run to get_messages
 - ♻️ refactor(cli): remove unused `Delete` import
 - ♻️ refactor(cli): remove unused Delete, Trash trait - Remove Delete and Trash traits from cull_gmail - Use RuleProcessor instead of Delete and Trash traits
 - ♻️ refactor(core): remove processor.rs
 - ♻️ refactor(message): remove delete functionality
 - ♻️ refactor(core): restructure modules for clarity
 - ♻️ refactor(processor): implement RuleProcessor trait for GmailClient
 - ♻️ refactor(cli): rename Processor to RuleProcessor
 - ♻️ refactor(cli): use mutable client for subcommands
 - ♻️ refactor(core): rename Processor to RuleProcessor
 - ♻️ refactor(message_cli): simplify message processing
 - ♻️ refactor(delete): streamline delete command execution
 - ♻️ refactor(gmail_client): change MessageSummary's visibility
 - ♻️ refactor(processor): simplify trash_messages function
 - ♻️ refactor(core): remove unused trash module
 - ♻️ refactor(trash): refactor trash module to trait implementation
 - ♻️ refactor(message_list): remove client parameter from add_labels
 - ♻️ refactor(delete): restructure delete functionality
 - ♻️ refactor(core): remove unused Delete module - Delete module is no longer needed.
 - ♻️ refactor(processor): consolidate message operations in GmailClient
 - ♻️ refactor(gmail_client): move message_summary to gmail_client
 - ♻️ refactor(message_list): implement MessageList trait for GmailClient
 - ♻️ refactor(cli): use GmailClient instead of credential file
 - ♻️ refactor(cli): use client for trash subcommand
 - ♻️ refactor(cli): use gmail client in run_cli
 - ♻️ refactor(cli): pass client to run command
 - ♻️ refactor(processor): use reference for GmailClient in processor builder
 - ♻️ refactor(cli): use client instance for message subcommand
 - ♻️ refactor(cli): use GmailClient for MessageList
 - ♻️ refactor(cli): use GmailClient in delete_cli
 - ♻️ refactor(cli): use gmail client for label operations
 - ♻️ refactor(trash): use GmailClient instead of credential string
 - ♻️ refactor(delete): use GmailClient for message list creation
 - ♻️ refactor(message_list): update add_labels function to accept &GmailClient
 - ♻️ refactor(gmail): improve gmail client structure
 - ♻️ refactor(processor): use GmailClient instead of credential_file
 - ♻️ refactor(cli): remove unused credential file
 - ♻️ refactor(message_list): use gmail client for label retrieval
 - ♻️ refactor(core): rename labels module to gmail_client
 - ♻️ refactor(gmail): rename labels.rs to gmail_client.rs

## [0.0.7] - 2025-10-12

Summary: Added[23], Build[1], Changed[8], Chore[5], Documentation[3], Fixed[10]

### Added

 - ✨ feat(processor): introduce processor builder
 - ✨ feat(cli): add execute option to processor
 - ✨ feat(processor): add execute flag for dry run
 - ✨ feat(cli): add execute flag to run action
 - ✨ feat(message_list): increase default max results
 - ✨ feat(cli): add skip action flags to cli
 - ✨ feat(cli): add skip-delete flag to cli
 - ✨ feat(cli): add option to skip trash actions
 - ✨ feat(config): add date calculation for EOL queries
 - ✨ feat(config): add retention period to eol rule
 - ✨ feat(processor): add label existence check before processing
 - ✨ feat(processor): add trash and delete message functionality
 - ✨ feat(cli): implement trash and delete actions
 - ✨ feat(processor): implement message deletion functionality
 - ✨ feat(config): add eol query function
 - ✨ feat(cli): add chrono crate as a dependency
 - ✨ feat(core): introduce message processor module
 - ✨ feat(processor): implement rule processor
 - ✨ feat(eol_rule): add describe function for eol rule
 - ✨ feat(cli): implement rule execution logic
 - ✨ feat(eol_action): add parse method to EolAction
 - ✨ feat(cli): add run command to execute rules
 - ✨ feat(cli): add run cli command

### Fixed

 - 🐛 fix(utils): correct string elision boundary calculation
 - 🐛 fix(utils): correct string elision boundary calculation
 - 🐛 fix(error): correct spelling error in error message
 - 🐛 fix(processor): correct typo in error message
 - 🐛 fix(processor): execute delete messages
 - 🐛 fix(message_age): correct data type for message age count
 - 🐛 fix(cli): correct count type in add_cli
 - 🐛 fix(processor): handle None query in eol_query
 - 🐛 fix(error): add error type for no query string calculated
 - 🐛 fix(error): add specific error for missing label in mailbox - add `LableNotFoundInMailbox` error to handle cases where a label is not found in the mailbox

### Changed

 - ♻️ refactor(utils): remove unused `get_start_boundary` function
 - ♻️ refactor(cli): extract action execution to separate function
 - ♻️ refactor(config): extract common logic to reduce duplication
 - ♻️ refactor(eol_rule): simplify eol_rule tests
 - ♻️ refactor(trash): refactor trash command
 - ♻️ refactor(trash): separate trash preparation and execution
 - ♻️ refactor(config): make EolRule public
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

[Unreleased]: https://github.com/jerus-org/cull-gmail/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/jerus-org/cull-gmail/compare/v0.0.16...v0.1.0
[0.0.16]: https://github.com/jerus-org/cull-gmail/compare/v0.0.15...v0.0.16
[0.0.15]: https://github.com/jerus-org/cull-gmail/compare/v0.0.14...v0.0.15
[0.0.14]: https://github.com/jerus-org/cull-gmail/compare/v0.0.13...v0.0.14
[0.0.13]: https://github.com/jerus-org/cull-gmail/compare/v0.0.12...v0.0.13
[0.0.12]: https://github.com/jerus-org/cull-gmail/compare/v0.0.11...v0.0.12
[0.0.11]: https://github.com/jerus-org/cull-gmail/compare/v0.0.10...v0.0.11
[0.0.10]: https://github.com/jerus-org/cull-gmail/compare/v0.0.9...v0.0.10
[0.0.9]: https://github.com/jerus-org/cull-gmail/compare/v0.0.8...v0.0.9
[0.0.8]: https://github.com/jerus-org/cull-gmail/compare/v0.0.7...v0.0.8
[0.0.7]: https://github.com/jerus-org/cull-gmail/compare/v0.0.6...v0.0.7
[0.0.6]: https://github.com/jerus-org/cull-gmail/compare/v0.0.5...v0.0.6
[0.0.5]: https://github.com/jerus-org/cull-gmail/compare/v0.0.4...v0.0.5
[0.0.4]: https://github.com/jerus-org/cull-gmail/compare/v0.0.3...v0.0.4
[0.0.3]: https://github.com/jerus-org/cull-gmail/compare/v0.0.2...v0.0.3
[0.0.2]: https://github.com/jerus-org/cull-gmail/compare/v0.0.1...v0.0.2
[0.0.1]: https://github.com/jerus-org/cull-gmail/releases/tag/v0.0.1

