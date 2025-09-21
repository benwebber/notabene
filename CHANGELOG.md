# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

* Add rule to check unreleased section position
* Add rule to check release order
8 Add rule to check release has a date
8 Add rule to check release date adheres to ISO 8601
* Add rule to check duplicate release version
* Add rule to check for undefined link references
* Add trait-based API
* Add `RuleSet`
* Add `Linter` to public API
* Add `--select` and `--ignore` flags to `check`
* Add support for `nb.toml` file

### Changed

* **BREAKING:** Redesign the library API. Significant changes:
    * Rename `Changelog` to `OwnedChangelog`
    * Add borrowed version of the changelog, `ParsedChangelog` is now part of the public API
    * Rename `parse_str` to `parse`
    * Rename `span::Index` to `span::Locator`
    * Make `Diagnostic` generic over `Span` and `Position`
* Run checks in single pass
* Change default output mode to full
* Renumber code sequence for initial rules

### Removed

* Remove `renderer` from public API
* Remove `parse_str` from public API
* Remove `parse_file` from public API

[Unreleased]: https://github.com/benwebber/notabene/compare/v0.1.0...HEAD
