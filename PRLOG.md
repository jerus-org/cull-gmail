# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- ✨ enhance retention policy configuration(pr [#77])
- improve documentation, tests, error handling, and formatting(pr [#78])

### Changed

- 📝 docs(readme)-improve library and CLI documentation(pr [#76])

## [0.0.10] - 2025-10-16

### Added

- ✨ add config file support(pr [#72])
- ✨ introduce client configuration(pr [#73])
- ✨ implement config builder pattern for ClientConfig(pr [#74])
- ✨ add default subcommand for rule execution(pr [#75])

### Changed

- ♻️ refactor(rule_processor)-remove unused delete functions(pr [#66])
- ♻️ refactor(message_list)-rename messages_list to list_messages(pr [#67])
- ♻️ refactor(cli)-restructure cli commands for better organization(pr [#68])
- ♻️ refactor-rename Config to Rules(pr [#69])
- ♻️ refactor(cli)-remove config from run args(pr [#70])
- ♻️ refactor(rules)-remove credentials config(pr [#71])

### Fixed

- 🐛 rule_processor: update Gmail API scope(pr [#65])

## [0.0.9] - 2025-10-14

### Added

- ✨ add configuration options for message listing(pr [#64])

## [0.0.8] - 2025-10-14

### Changed

- ♻️ refactor(gmail)-rename labels.rs to gmail_client.rs(pr [#63])

## [0.0.7] - 2025-10-12

### Added

- ✨ add run cli command(pr [#59])
- ✨ increase default max results(pr [#60])
- ✨ add execute flag to run action(pr [#62])

### Fixed

- 🐛 utils: correct string elision boundary calculation(pr [#61])

## [0.0.6] - 2025-10-09

### Added

- ✨ implement commands dispatching(pr [#49])
- ✨ add label functionality to rules(pr [#51])
- ✨ add remove label from rule(pr [#52])
- ✨ implement action subcommand(pr [#54])
- ✨ implement batch delete functionality(pr [#57])

### Changed

- ♻️ refactor(cli)-consolidate rules and labels under config subcommand(pr [#48])
- ♻️ refactor(cli)-rename command to sub_command for clarity(pr [#55])
- ♻️ refactor(project)-move main.rs to cli directory(pr [#56])

### Fixed

- 🐛 label_cli: display labels by rule id(pr [#50])
- 🐛 config: correct grammar in EolRule display(pr [#53])

## [0.0.5] - 2025-10-08

### Added

- ✨ feat(cli): add config subcommand for end-of-life rules(pr [#34])
- ✨ implement configuration file handling(pr [#36])
- ✨ add function to list rules(pr [#39])
- ✨ implement add command(pr [#40])
- ✨ enhance rule management with BTreeMap(pr [#41])
- ✨ add support for labels to retention rules(pr [#42])
- ✨ add delete flag for retention rules(pr [#43])
- ✨ add remove command to rules cli(pr [#44])
- ✨ implement trace logging for configuration(pr [#46])

### Changed

- 📝 docs(PRLOG)-update PRLOG.md(pr [#33])
- 📝 docs(PRLOG)-update PRLOG with unreleased changes(pr [#35])

### Fixed

- 🐛 ui: correct grammar errors in eol command and trash messages(pr [#37])
- 🐛 error: refine error message for rule selector(pr [#45])
- 🐛 error: improve error message for missing labels(pr [#47])

## [0.0.4] - 2025-10-07

### Added

- ✨ add message_ids to MessageList struct(pr [#28])
- ✨ implement trash functionality(pr [#29])
- ✨ create message summary struct(pr [#30])

### Changed

- ♻️ refactor(message)-improve subject logging with early returns(pr [#25])
- ♻️ refactor(labels)-improve label listing and mapping(pr [#26])
- ♻️ refactor(message)-rename message to message_list(pr [#27])

## [0.0.3] - 2025-10-04

### Added

- ✨ add query support to list messages(pr [#22])
- ✨ add label listing subcommand(pr [#23])

### Changed

- ♻️ refactor(labels)-simplify labels struct initialization(pr [#21])
- ♻️ refactor(cli)-rename list subcommand to message(pr [#24])

## [0.0.2] - 2025-10-03

### Added

- ✨ add command line interface with logging(pr [#12])
- ✨ add list subcommand(pr [#13])
- ✨ implement list api to retrieve gmail messages(pr [#14])
- ✨ implement list functionality for Gmail API(pr [#15])
- ✨ add max results option to list command(pr [#18])
- ✨ add pagination to list command(pr [#19])
- ✨ add label filtering to list command(pr [#20])

### Changed

- 🔧 chore(release)-update PRLOG replacements for release process(pr [#9])
- 🔧 chore(ci)-remove hardcoded version from CircleCI config(pr [#10])
- 🔧 chore(config)-update Cargo.toml with lints and library settings(pr [#11])
- Delete-client(pr [#17])

### Fixed

- 🐛 list: remove debug print statement(pr [#16])

## [0.0.1] - 2025-09-30

### Changed

- ✨ feat(lib)-add addition function with test(pr [#2])
- 👷 ci(config)-add version parameter to save_next_version job(pr [#4])
- 👷 ci(circleci)-fix version string format in config(pr [#5])
- 👷 ci(circleci)-add condition to version retrieval step(pr [#6])
- 🔧 chore(config)-comment out unused pre-release replacements(pr [#7])
- 📝 docs(CHANGELOG)-add initial changelog file(pr [#8])

[#2]: https://github.com/jerus-org/cull-gmail/pull/2
[#4]: https://github.com/jerus-org/cull-gmail/pull/4
[#5]: https://github.com/jerus-org/cull-gmail/pull/5
[#6]: https://github.com/jerus-org/cull-gmail/pull/6
[#7]: https://github.com/jerus-org/cull-gmail/pull/7
[#8]: https://github.com/jerus-org/cull-gmail/pull/8
[#9]: https://github.com/jerus-org/cull-gmail/pull/9
[#10]: https://github.com/jerus-org/cull-gmail/pull/10
[#11]: https://github.com/jerus-org/cull-gmail/pull/11
[#12]: https://github.com/jerus-org/cull-gmail/pull/12
[#13]: https://github.com/jerus-org/cull-gmail/pull/13
[#14]: https://github.com/jerus-org/cull-gmail/pull/14
[#15]: https://github.com/jerus-org/cull-gmail/pull/15
[#16]: https://github.com/jerus-org/cull-gmail/pull/16
[#17]: https://github.com/jerus-org/cull-gmail/pull/17
[#18]: https://github.com/jerus-org/cull-gmail/pull/18
[#19]: https://github.com/jerus-org/cull-gmail/pull/19
[#20]: https://github.com/jerus-org/cull-gmail/pull/20
[#21]: https://github.com/jerus-org/cull-gmail/pull/21
[#22]: https://github.com/jerus-org/cull-gmail/pull/22
[#23]: https://github.com/jerus-org/cull-gmail/pull/23
[#24]: https://github.com/jerus-org/cull-gmail/pull/24
[#25]: https://github.com/jerus-org/cull-gmail/pull/25
[#26]: https://github.com/jerus-org/cull-gmail/pull/26
[#27]: https://github.com/jerus-org/cull-gmail/pull/27
[#28]: https://github.com/jerus-org/cull-gmail/pull/28
[#29]: https://github.com/jerus-org/cull-gmail/pull/29
[#30]: https://github.com/jerus-org/cull-gmail/pull/30
[#34]: https://github.com/jerus-org/cull-gmail/pull/34
[#35]: https://github.com/jerus-org/cull-gmail/pull/35
[#36]: https://github.com/jerus-org/cull-gmail/pull/36
[#37]: https://github.com/jerus-org/cull-gmail/pull/37
[#39]: https://github.com/jerus-org/cull-gmail/pull/39
[#40]: https://github.com/jerus-org/cull-gmail/pull/40
[#41]: https://github.com/jerus-org/cull-gmail/pull/41
[#42]: https://github.com/jerus-org/cull-gmail/pull/42
[#43]: https://github.com/jerus-org/cull-gmail/pull/43
[#44]: https://github.com/jerus-org/cull-gmail/pull/44
[#45]: https://github.com/jerus-org/cull-gmail/pull/45
[#46]: https://github.com/jerus-org/cull-gmail/pull/46
[#47]: https://github.com/jerus-org/cull-gmail/pull/47
[#48]: https://github.com/jerus-org/cull-gmail/pull/48
[#49]: https://github.com/jerus-org/cull-gmail/pull/49
[#50]: https://github.com/jerus-org/cull-gmail/pull/50
[#51]: https://github.com/jerus-org/cull-gmail/pull/51
[#52]: https://github.com/jerus-org/cull-gmail/pull/52
[#53]: https://github.com/jerus-org/cull-gmail/pull/53
[#54]: https://github.com/jerus-org/cull-gmail/pull/54
[#55]: https://github.com/jerus-org/cull-gmail/pull/55
[#56]: https://github.com/jerus-org/cull-gmail/pull/56
[#57]: https://github.com/jerus-org/cull-gmail/pull/57
[#59]: https://github.com/jerus-org/cull-gmail/pull/59
[#60]: https://github.com/jerus-org/cull-gmail/pull/60
[#61]: https://github.com/jerus-org/cull-gmail/pull/61
[#62]: https://github.com/jerus-org/cull-gmail/pull/62
[#63]: https://github.com/jerus-org/cull-gmail/pull/63
[#64]: https://github.com/jerus-org/cull-gmail/pull/64
[#65]: https://github.com/jerus-org/cull-gmail/pull/65
[#66]: https://github.com/jerus-org/cull-gmail/pull/66
[#67]: https://github.com/jerus-org/cull-gmail/pull/67
[#68]: https://github.com/jerus-org/cull-gmail/pull/68
[#69]: https://github.com/jerus-org/cull-gmail/pull/69
[#70]: https://github.com/jerus-org/cull-gmail/pull/70
[#71]: https://github.com/jerus-org/cull-gmail/pull/71
[#72]: https://github.com/jerus-org/cull-gmail/pull/72
[#73]: https://github.com/jerus-org/cull-gmail/pull/73
[#74]: https://github.com/jerus-org/cull-gmail/pull/74
[#75]: https://github.com/jerus-org/cull-gmail/pull/75
[#76]: https://github.com/jerus-org/cull-gmail/pull/76
[#77]: https://github.com/jerus-org/cull-gmail/pull/77
[#78]: https://github.com/jerus-org/cull-gmail/pull/78
[Unreleased]: https://github.com/jerus-org/cull-gmail/compare/v0.0.10...HEAD
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
