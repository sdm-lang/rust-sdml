# Package sdml-json

Simple Domain Modeling Language (SDML) JSON I/O.

[[https://crates.io/crates/sdml_json][https://img.shields.io/crates/v/sdml_json.svg]]
[[https://docs.rs/sdml_json][https://img.shields.io/docsrs/sdml-json.svg]]

This package is part of the Rust SDML project and specifically defines the JSON
mapping for SDML modules. The project's intent is to provide an idiomatic
implementation of the in-memory model, parser, generators, and the CLI tool.

The following figure demonstrates this package in the broader project context.

![Package Overview](https://raw.githubusercontent.com/sdm-lang/rust-sdml/refs/heads/main/doc/overview-generate.png)

## Changes

### Version 0.4.1

* Update version to be consistent with new core language.

### Version 0.1.0

* Initial Release
  * Copied 1:1 JSON from crate `sdml-generate`.
  * Used crate `objio` for the interface rather than the `Generator` trait from the
    `sdml-generate` crate.
  * Copied context generation from crate `sdml-tera`.
