# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.0.4] - 2025-10-07

### Added

- ✨ add message_ids to MessageList struct(pr [#28])
- ✨ implement trash functionality(pr [#29])
- ✨ create message summary struct(pr [#30])

### Changed

- ♻️ refactor(message)-improve subject logging with early returns(pr [#25])
- ♻️ refactor(labels)-improve label listing and mapping(pr [#26])
- ♻️ refactor(message)-rename message to message_list(pr [#27])

### Fixed

- 🐛 trash: correct log message in trash module(pr [#31])

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
[0.0.4]: https://github.com/jerus-org/cull-gmail/compare/v0.0.3...v0.0.4
[0.0.3]: https://github.com/jerus-org/cull-gmail/compare/v0.0.2...v0.0.3
[0.0.2]: https://github.com/jerus-org/cull-gmail/compare/v0.0.1...v0.0.2
[0.0.1]: https://github.com/jerus-org/cull-gmail/releases/tag/v0.0.1
