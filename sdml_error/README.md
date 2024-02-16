# Crate sdlm_error

Rust Library containing the error and diagnostic types for the Simple Domain Modeling Language (SDML).

[![crates.io](https://img.shields.io/crates/v/sdml_generate.svg)](https://crates.io/crates/sdml_generate)
[![docs.rs](https://docs.rs/sdml_generate/badge.svg)](https://docs.rs/sdml_generate)

This package is part of the Rust SDML project and specifically defines the error and diagnostic types for the project.
The project's intent is to provide an idiomatic implementation of the in-memory model, parser, generators, and the CLI tool.

The following figure demonstrates this package in the broader project context.

```
                            ╭───────╮
                            │  CLI  │
                       ╔══  │ crate │  ══╗
                       ║    ╰───────╯    ║
┌╌╌╌╌╌╌╌╌┐             V                 V
┆        ┆       ╭──────────╮       ╭──────────╮       Formatted Source
┆ source ┆  ══>  │  parse   │  ══>  │ generate │  ══>  RDF Representation 
┆  file  ┆    ╭──│  crate   │───────│   crate  │──╮    Documentation
┆        ┆    │  ╰──────────╯       ╰──────────╯  │    Diagrams
└╌╌╌╌╌╌╌╌┘    │             core crate            │──╮
              ╰───────────────────────────────────╯  │
 ┌───────┐             ⋀          error crate        │
 │ other │             ║  ╌╌╌╌╌╌╌╌╌╌╌╌╌╌─────────────╯
 │ tools │  ═══════════╝
 └───────┘
```

## Changes

**Version 0.1.1**

- Feature: add new diagnostic for `PropertyReferenceNotProperty` where the property name in a member does not resolve to
  a property definition.
  - Add new code to `ErrorCode`
  - Add new function `property_reference_not_property`

**Version 0.1.0**

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
