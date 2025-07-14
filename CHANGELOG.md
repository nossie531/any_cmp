# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.5.1] - 2025-07-14

### Changed

- Polish documentation.

## [0.5.0] - 2025-06-23

### Added

- Add `prelude` module (Although this crate is very small).

### Changed

- Rust edition is updated to 2024.

### Removed

- Remove `upcast` module ([Trait upcasting] is a good substitute).

[Trait Upcasting]: https://blog.rust-lang.org/2025/04/03/Rust-1.86.0/#trait-upcasting

## [0.4.1] - 2024-01-15

### Changed

- Minor refactoring.

## [0.4.0] - 2023-11-05

### Added

- Add box upcasting methods (ex: `as_any_eq_box`).

### Fixed

- Fix forgotten implementation.

## [0.3.0] - 2023-10-30

### Added

- Add `must_use` annotations to several locations.

## [0.2.0] - 2023-09-06

### Changed

- Module `upcast` specs have slightly changed.

## [0.1.0] - 2023-09-05

- First release.

[0.5.1]: https://github.com/nossie531/any_cmp/compare/v0.5.0...v0.5.1
[0.5.0]: https://github.com/nossie531/any_cmp/compare/v0.4.0...v0.5.0
[0.4.1]: https://github.com/nossie531/any_cmp/compare/v0.4.0...v0.4.1
[0.4.0]: https://github.com/nossie531/any_cmp/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/nossie531/any_cmp/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/nossie531/any_cmp/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/nossie531/any_cmp/releases/tag/v0.1.0
