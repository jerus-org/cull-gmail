# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.4] - 2026-02-14

### Changed

- 👷 ci(circleci)-update release workflow configuration(pr [#163])

## [0.1.3] - 2026-02-14

### Changed

- 👷 ci(circleci)-fix release tag format in CircleCI config(pr [#161])
- 👷 ci(circleci)-use toolkit to update prlog(pr [#162])

## [0.1.2] - 2026-02-14

### Changed

- 🔧 chore(ci)-add version argument to pcu command(pr [#160])

## [0.1.1] - 2026-02-13

### Changed

- 👷 ci(circleci)-fix order of version command arguments(pr [#159])

## [0.1.0] - 2026-02-13

### Added

- ✨ add optional rules path argument to cli(pr [#110])
- ✨ enhance rules configuration(pr [#112])
- BREAKING: migrate to circleci-toolkit v4.2.1(pr [#134])
- add security improvements to CI(pr [#142])

### Changed

- ♻️ refactor(cli)-rename rule management subcommands for clarity(pr [#113])
- 🔧 chore(dependencies)-clap test and tidy ups(pr [#114])
- ci-enable update_pcu in label job to test fix(pr [#135])
- 👷 ci(circleci)-add release configuration for automated deployment(pr [#154])
- 👷 ci(circleci)-update release workflow configuration(pr [#155])
- 👷 ci(circleci)-simplify nextsv version calculation(pr [#156])
- 👷 ci(circleci)-add kdeets installation step to release workflow(pr [#157])
- 👷 ci(circleci)-enhance tag generation logic(pr [#158])

### Fixed

- 🐛 client: fix config root parsing(pr [#111])
- deps: update rust crate assert_fs to 1.1.3(pr [#115])
- deps: update rust crate base64 to 0.22.1(pr [#116])
- deps: update rust crate clap to 4.5.53(pr [#117])
- deps: update rust crate config to 0.15.19(pr [#118])
- deps: update rust crate flate2 to 1.1.5(pr [#119])
- deps: update rust crate futures to 0.3.31(pr [#120])
- deps: update rust crate httpmock to 0.8.2(pr [#121])
- deps: update rust crate lazy-regex to 3.4.2(pr [#122])
- deps: update rust crate predicates to 3.1.3(pr [#123])
- deps: update rust crate temp-env to 0.3.6(pr [#124])
- deps: update rust crate log to 0.4.29(pr [#125])
- deps: update rust crate serde_json to 1.0.149(pr [#126])
- deps: update rust crate assert_cmd to 2.1.2(pr [#129])
- deps: update rust crate toml to 0.9.11(pr [#127])
- deps: update rust crate dialoguer to 0.12.0(pr [#130])
- deps: update rust crate tempfile to 3.24.0(pr [#131])
- deps: update tokio packages(pr [#132])
- deps: update rust crate chrono to 0.4.43(pr [#136])
- deps: update rust crate clap to 4.5.54(pr [#137])
- deps: update rust crate flate2 to 1.1.8(pr [#138])
- deps: update rust crate hyper-rustls to 0.27.7(pr [#139])
- deps: update rust crate lazy-regex to 3.5.1(pr [#140])
- deps: update rust crate thiserror to 2.0.18(pr [#141])
- deps: resolve rustls crypto provider conflict(pr [#143])
- deps: update rust crate flate2 to 1.1.9(pr [#146])
- deps: update rust crate env_logger to 0.11.9(pr [#145])
- deps: update rust crate clap to 4.5.58(pr [#144])
- deps: update rust crate httpmock to 0.8.3(pr [#147])
- deps: update rust crate predicates to 3.1.4(pr [#148])
- deps: update rust crate toml to 0.9.12(pr [#149])
- deps: update dependency toolkit to v4.4.2(pr [#150])
- deps: update rust crate lazy-regex to 3.6.0(pr [#151])
- deps: update rust crate tempfile to 3.25.0(pr [#152])
- deps: update rust crate toml to v1(pr [#153])

## [0.0.16] - 2025-10-30

### Added

- ✨ add initialise_message_list to processor(pr [#101])
- ✨ implement batch operations for message deletion and trashing(pr [#106])
- ✨ support multiple actions per label(pr [#107])

### Changed

- ♻️ refactor(core)-rename initialise_message_list to initialise_lists(pr [#102])

### Fixed

- 🐛 gmail: handle batch delete errors(pr [#103])
- 🐛 rules: correct grammar and improve date calculation(pr [#104])
- 🐛 gmail: use GMAIL_DELETE_SCOPE for batch delete(pr [#105])
- 🐛 cli: correct rule execution order for trash and delete(pr [#108])
- 🐛 rule_processor: enhance logging for chunk processing(pr [#109])

## [0.0.15] - 2025-10-26

### Changed

- ♻️ refactor(message_list)-allow pre/post text in log_messages(pr [#100])

## [0.0.14] - 2025-10-23

### Added

- ✨ load application secret with logging(pr [#95])
- ✨ add token and auth uri config options(pr [#98])

### Changed

- ✨ init-add --skip-rules to suppress rules.toml creation for ephemeral environments(pr [#97])

### Fixed

- 🐛 config: improve config logging format(pr [#96])
- 🐛 config: reduce log verbosity(pr [#99])

## [0.0.13] - 2025-10-22

### Fixed

- 🐛 cli: load config file as optional(pr [#94])

## [0.0.12] - 2025-10-22

### Added

- 🔐 Add token export/import for ephemeral environments(pr [#87])
- ✨ guided setup to create config, rules, and OAuth2 tokens(pr [#90])

### Changed

- 📘 Add WARP.md developer guidance file(pr [#89])
- ✨ Add configurable rules directory support(pr [#91])
- 📦 build(ci)-upgrade circleci-toolkit orb to v2.13.5(pr [#93])

### Fixed

- 🐛 ci: correct default test runner value(pr [#92])

## [0.0.11] - 2025-10-20

### Added

- ✨ enhance retention policy configuration(pr [#77])
- improve documentation, tests, error handling, and formatting(pr [#78])
- ✨ improve docs, tests, idioms, and CI enforcement(pr [#79])
- ✨ introduce nextest test runner(pr [#82])

### Changed

- 📝 docs(readme)-improve library and CLI documentation(pr [#76])
- 🧰 chore(message-list)-ensure rustdoc compliance and test coverage(pr [#80])
- 📝 docs(gmail_client)-add comprehensive documentation and unit testing(pr [#81])
- 📝 docs(eol_action)-add comprehensive documentation and unit testing with safety enhancements(pr [#83])
- ♻️ refactor-remove redundant credential module(pr [#84])
- 📝 refactor(client_config)-enhance module with comprehensive docs and testing(pr [#85])
- 📚 docs(cli)-comprehensive documentation and formatting for CLI modules(pr [#86])

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
[#79]: https://github.com/jerus-org/cull-gmail/pull/79
[#80]: https://github.com/jerus-org/cull-gmail/pull/80
[#81]: https://github.com/jerus-org/cull-gmail/pull/81
[#82]: https://github.com/jerus-org/cull-gmail/pull/82
[#83]: https://github.com/jerus-org/cull-gmail/pull/83
[#84]: https://github.com/jerus-org/cull-gmail/pull/84
[#85]: https://github.com/jerus-org/cull-gmail/pull/85
[#86]: https://github.com/jerus-org/cull-gmail/pull/86
[#87]: https://github.com/jerus-org/cull-gmail/pull/87
[#89]: https://github.com/jerus-org/cull-gmail/pull/89
[#90]: https://github.com/jerus-org/cull-gmail/pull/90
[#91]: https://github.com/jerus-org/cull-gmail/pull/91
[#92]: https://github.com/jerus-org/cull-gmail/pull/92
[#93]: https://github.com/jerus-org/cull-gmail/pull/93
[#94]: https://github.com/jerus-org/cull-gmail/pull/94
[#95]: https://github.com/jerus-org/cull-gmail/pull/95
[#96]: https://github.com/jerus-org/cull-gmail/pull/96
[#97]: https://github.com/jerus-org/cull-gmail/pull/97
[#98]: https://github.com/jerus-org/cull-gmail/pull/98
[#99]: https://github.com/jerus-org/cull-gmail/pull/99
[#100]: https://github.com/jerus-org/cull-gmail/pull/100
[#101]: https://github.com/jerus-org/cull-gmail/pull/101
[#102]: https://github.com/jerus-org/cull-gmail/pull/102
[#103]: https://github.com/jerus-org/cull-gmail/pull/103
[#104]: https://github.com/jerus-org/cull-gmail/pull/104
[#105]: https://github.com/jerus-org/cull-gmail/pull/105
[#106]: https://github.com/jerus-org/cull-gmail/pull/106
[#107]: https://github.com/jerus-org/cull-gmail/pull/107
[#108]: https://github.com/jerus-org/cull-gmail/pull/108
[#109]: https://github.com/jerus-org/cull-gmail/pull/109
[#110]: https://github.com/jerus-org/cull-gmail/pull/110
[#111]: https://github.com/jerus-org/cull-gmail/pull/111
[#112]: https://github.com/jerus-org/cull-gmail/pull/112
[#113]: https://github.com/jerus-org/cull-gmail/pull/113
[#114]: https://github.com/jerus-org/cull-gmail/pull/114
[#115]: https://github.com/jerus-org/cull-gmail/pull/115
[#116]: https://github.com/jerus-org/cull-gmail/pull/116
[#117]: https://github.com/jerus-org/cull-gmail/pull/117
[#118]: https://github.com/jerus-org/cull-gmail/pull/118
[#119]: https://github.com/jerus-org/cull-gmail/pull/119
[#120]: https://github.com/jerus-org/cull-gmail/pull/120
[#121]: https://github.com/jerus-org/cull-gmail/pull/121
[#122]: https://github.com/jerus-org/cull-gmail/pull/122
[#123]: https://github.com/jerus-org/cull-gmail/pull/123
[#124]: https://github.com/jerus-org/cull-gmail/pull/124
[#134]: https://github.com/jerus-org/cull-gmail/pull/134
[#125]: https://github.com/jerus-org/cull-gmail/pull/125
[#126]: https://github.com/jerus-org/cull-gmail/pull/126
[#129]: https://github.com/jerus-org/cull-gmail/pull/129
[#135]: https://github.com/jerus-org/cull-gmail/pull/135
[#127]: https://github.com/jerus-org/cull-gmail/pull/127
[#130]: https://github.com/jerus-org/cull-gmail/pull/130
[#131]: https://github.com/jerus-org/cull-gmail/pull/131
[#132]: https://github.com/jerus-org/cull-gmail/pull/132
[#136]: https://github.com/jerus-org/cull-gmail/pull/136
[#137]: https://github.com/jerus-org/cull-gmail/pull/137
[#138]: https://github.com/jerus-org/cull-gmail/pull/138
[#139]: https://github.com/jerus-org/cull-gmail/pull/139
[#140]: https://github.com/jerus-org/cull-gmail/pull/140
[#141]: https://github.com/jerus-org/cull-gmail/pull/141
[#142]: https://github.com/jerus-org/cull-gmail/pull/142
[#143]: https://github.com/jerus-org/cull-gmail/pull/143
[#146]: https://github.com/jerus-org/cull-gmail/pull/146
[#145]: https://github.com/jerus-org/cull-gmail/pull/145
[#144]: https://github.com/jerus-org/cull-gmail/pull/144
[#147]: https://github.com/jerus-org/cull-gmail/pull/147
[#148]: https://github.com/jerus-org/cull-gmail/pull/148
[#149]: https://github.com/jerus-org/cull-gmail/pull/149
[#150]: https://github.com/jerus-org/cull-gmail/pull/150
[#151]: https://github.com/jerus-org/cull-gmail/pull/151
[#152]: https://github.com/jerus-org/cull-gmail/pull/152
[#153]: https://github.com/jerus-org/cull-gmail/pull/153
[#154]: https://github.com/jerus-org/cull-gmail/pull/154
[#155]: https://github.com/jerus-org/cull-gmail/pull/155
[#156]: https://github.com/jerus-org/cull-gmail/pull/156
[#157]: https://github.com/jerus-org/cull-gmail/pull/157
[#158]: https://github.com/jerus-org/cull-gmail/pull/158
[#159]: https://github.com/jerus-org/cull-gmail/pull/159
[#160]: https://github.com/jerus-org/cull-gmail/pull/160
[#161]: https://github.com/jerus-org/cull-gmail/pull/161
[#162]: https://github.com/jerus-org/cull-gmail/pull/162
[#163]: https://github.com/jerus-org/cull-gmail/pull/163
[0.1.4]: https://github.com/jerus-org/cull-gmail/compare/v0.1.3...v0.1.4
[0.1.3]: https://github.com/jerus-org/cull-gmail/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/jerus-org/cull-gmail/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/jerus-org/cull-gmail/compare/v0.1.0...v0.1.1
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
