# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Breaking Changes

- `Signed::from(isize)` and `Unsigned::from(usize)` are now implemented. This is
  considered a breaking change because TryFrom for these types now is
  infallible, which could break code that depended on the previous Error
  associated type.
- `Variable::encode_variable` has had its `destination` paramter changed from
  `&mut Write` to `Write`, because `io::Write` is implemented automatically for
  `&mut Write` references. This change should not cause a code breakage for many
  users.

### Changes

- Primitives are now encoded and decoded directly, instead of being converted
  through `Signed`/`Unsigned`. This is an optimization that allows users who are
  directly encoding specific kinds of integers to avoid conversions to/from
  `u128`/`i128` internally.

## v1.0.1

### Added

- Added `isize` and `usize` implementations for Variable.
- Signed::try_from(isize) and Unsigned::try_from(usize) are now implemented.

## v1.0.0

First stable release.
