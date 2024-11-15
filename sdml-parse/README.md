# Package sdml-parse

Rust Parser for the Simple Domain Modeling Language (SDML).

[![Crates.io](https://img.shields.io/crates/v/sdml_parse.svg)](https://crates.io/crates/sdml_parse)
[![Docs.rs](https://img.shields.io/docsrs/sdml-parse.svg)](https://docs.rs/sdml_parse)

This package is part of the Rust SDML project and specifically implements a parser from SDML surface syntax to the
in-memory model representation. The project's intent is to provide an idiomatic implementation of the in-memory model,
parser, generators, and the CLI tool.

The following figure demonstrates this package in the broader project context.

![Package Overview](https://raw.githubusercontent.com/sdm-lang/rust-sdml/refs/heads/main/doc/overview-parse.png)

## Changes

### Version 0.3.1

* Feature: additional grammar support for definitions and import renames.

### Version 0.3.0

* Feature: updates to support the latest grammar, see `sdml-core`.

### Version 0.2.13-0.2.14

* Build: update dependency from `sdml_error` to `sdml-errors`.
* Build: bump version of `sdml-core`.

### Version 0.2.12

* Fix: update all test cases with latest API changes.
  * Add file ID into all test example "ron" files.
  * Add module import Span into all test example "ron" files.
  * Use new HeaderValue in relevant test example "ron" files.
  * Add use of ModuleStore trait.

### Version 0.2.11

* Build: upgrade to `sdml_core` version `0.2.14` and the new `ModelStore` trait.

### Version 0.2.10

* Fix: Handle tree-sitter `ERROR` nodes correctly when they cause the top-level rule to fail.

### Version 0.2.9

* Build: Using `sdml_core` version `0.2.11` for updated validation.
* Fix: minor changes found by better validation.

### Version 0.2.8

* Build: Using `sdml_core` version `0.2.10` for new stdlib names.

### Version 0.2.7

* Fix: Cardinality parser set incorrect default values.
  * Fix: For min/max it should be `one` and not `zero_or_one` as the default to match the `DEFAULT_CARDINALITY` constant in the model.
  * Fix: For ordering/uniqueness the default if not parsed should be `None` not `Some(Default::default())`.
  * Update: the `with_` constructors on `Cardinality` to take option types.

### Version 0.2.6

* Build: Removed indirect dependencies from Cargo.toml.

### Version 0.2.5

* Test: Update test cases for:
  * The new `source_file` field on `Module`.
  * The new `ModuleLoader` API requiring a `ModuleCache`.

### Version 0.2.4

* Fix: parsing new `rdf_def` rules now works correctly.
  * Updated test cases.

### Version 0.2.3

* Feature: add new stdlib modules with standard layout.
* Feature: minor refactor of cache and loader.

### Version 0.2.2

* Feature: Update to latest grammar for version URIs and RDF definitions.
  * Add support for base URI on modules.
  * Add support for version info and URI on modules.
  * Add support for version URI on module import.
  * Parse RDF definitions for classes and properties.

### Version 0.2.1

* Feature: Remove member groups.

### Version 0.2.0

* Feature: Update to latest grammar.
  * Remove Value Variant numeric values.
  * Update formal constraints.
  * Add type classes.

### Version 0.1.9

* Added support for `mapping_type` and `mapping_value` rules.

### Version 0.1.8

* Removed macros from the resolver model.

### Version 0.1.7

* Updated parser with grammar changes in `tree-sitter-sdml` version `0.1.29`.
* Added all the test cases from core.

### Version 0.1.6

* Updated parser with grammar changes in `tree-sitter-sdml` version `0.1.26`.

### Version 0.1.5

* Created a `stdlib` module and moved all the SDML and relevant RDF files into it.
* Updated `tree-sitter-sdml` to version `0.1.21` with updated constraints.

### Version 0.1.4

Previously part of a single crate [sdml](https://crates.io/crates/sdml).
