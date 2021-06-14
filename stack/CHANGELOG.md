# Changelog

All notable changes in this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2018-09-14
### Changed
  - `Machine::jump` now just does a jump and does not manipulate the stack frame.
    If you were using `jump` before you probably now want to use `call`.
  - `Machine::call` adds a stack frame and calls `jump`.
  - `Machine::run` is now a method instead of an associated function.

## [0.3.0] - 2017-11-22
### Added
  - `Code` now implements `From<Builder>`.
  - `Builder` now attempts to deduplicate data using `PartialEq`.

### Removed
  - `Code::from_builder` has been removed.
  - `Machine::from_builder` has been removed.

## [0.2.0] - 2017-11-14
### Added
  - I invented a project.
