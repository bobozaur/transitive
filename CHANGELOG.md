
# Change Log
All notable changes to this project will be documented in this file.
 
The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).
 
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