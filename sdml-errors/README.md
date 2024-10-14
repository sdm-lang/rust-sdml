# Package sdml-errors

Rust Library containing the error and diagnostic types for the Simple Domain Modeling Language (SDML).

[![Crates.io](https://img.shields.io/crates/v/sdml_errors.svg)](https://crates.io/crates/sdml_errors)
[![Docs.rs](https://img.shields.io/docsrs/sdml-errors.svg)](https://docs.rs/sdml_errors)

This package is part of the Rust SDML project and specifically defines the error and diagnostic types for the project.
The project's intent is to provide an idiomatic implementation of the in-memory model, parser, generators, and the CLI tool.

The following figure demonstrates this package in the broader project context.

![Package Overview](https://raw.githubusercontent.com/sdm-lang/rust-sdml/refs/heads/main/doc/overview.png)

## Changes

### Version 0.3.1

- Feature: add `LanguageTagError` variant for external error type.
- Feature: add `Template` variant for external error type..
  - put this behind a feature, it's not used in core, parser, etc.

### Version 0.3.0

- Build: align version number with `sdml-core` supporting the latest grammar.

### Version 0.1.6

- Feature: add a new variant in `Error`, `GeneratorError` for use by the `sdml-generator` crate.

### Version 0.1.5

- Feature: add new diagnostic for `IdentifierNotPreferredCase` where an identifier is not in the preferred case style for
  its usage.
  - Add new code to `ErrorCode`.
  - Add new function `identifier_not_preferred_case`.
  - Add new enum `IdentifierCaseConvention` used to identify the case style to enforce.

### Version 0.1.4

- Feature: improved a number of diagnostic help messages.
- Feature: added shared `UseColor` type.

### Version 0.1.3

- Feature: add new diagnostic for `DeprecatedTermUsed` where an identifier includes a term listed in a supplied `TermSet`.
  - Add new code to `ErrorCode`.
  - Add new function `deprecated_term_used`.

### Version 0.1.2

- Feature: add new diagnostic for `DoubleUnderscoredIdentifier` where an identifier include two (or more) consecutive
  underscore characters.
  - Add new code to `ErrorCode`.
  - Add new function `double_underscored_identifier`.

### Version 0.1.1

- Feature: add new diagnostic for `PropertyReferenceNotProperty` where the property name in a member does not resolve to a
  property definition.
  - Add new code to `ErrorCode`.
  - Add new function `property_reference_not_property`.

### Version 0.1.0

Initial Release.

- Error Handling:
  - Copy `error` module from `sdml_core`, rename as `errors`.
  - Remove diagnostics from the existing `Error` type.
- Diagnostics:
  - Copy `diagnostics` module.
  - Create new `diagnostics::codes` module and `ErrorCode` enum.
  - Create new `diagnostics::functions` module and functions for each `ErrorCode`.
- Diagnostic Reporting:
  - Create new `diagnostics::reporter` module.
  - Create new `Reporter` trait.
  - Create a `StandardStreamReporter` to emit colored and structured errors to the console.
  - Create a `BailoutReporter` that will turn the first diagnostic it is given into an error.
