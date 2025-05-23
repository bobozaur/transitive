# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [1.2.0] - 2025-05-23

### Added

- [#15](https://github.com/bobozaur/transitive/pull/15): Check that at least two distinct types are in the type list.

### Fixed

- [#14](https://github.com/bobozaur/transitive/pull/14): Disallow type lists with only one type.

## [1.1.0] - 2025-03-13

### Changed

- Allow more complex types in the macro attribute's type path.

### Added

- [#12](https://github.com/bobozaur/transitive/pull/12): Improves type parameters parsing to allow complex types,
  such as the ones with generics.

## [1.0.1] - 2024-05-02

### Changed

- Clear out derive macro documentation.

## [1.0.0] - 2024-05-02

### Added

- [#6](https://github.com/bobozaur/transitive/issues/6): Added generics support on the derived type.

### Changed

- Updated dependencies.
- Removed usage of `darling`.
- Improved path direction parsing.

## [0.5.0] - 2023-07-03

### Added

- [#8](https://github.com/bobozaur/transitive/issues/8): Added the ability to specify custom error types for fallible conversions.

### Changed

- Updated dependencies.
- Refactored library using `darling`.
- Merged the two `TransitiveFrom` and `TransitiveTryFrom` derive macros into a single `Transitive` macro (the behavior is now controlled through the attribute modifiers).
- Dropped the `all` macro attribute modifier.

## [0.4.3] - 2023-03-09

First "feature complete" release.
