# Package sdml-generate

Rust Library of artifact generators from the Simple Domain Modeling Language
(SDML).

[![License-Apache_2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![License-MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/v/sdml_generate.svg)](https://crates.io/crates/sdml_generate)
[![Docs.rs](https://img.shields.io/docsrs/sdml-generate.svg)](https://docs.rs/sdml_generate)

This package is part of the Rust SDML project and specifically defines the
model-to-*other* generators for SDML modules. The project's intent is to provide
an idiomatic implementation of the in-memory model, parser, generators, and the
CLI tool.

The following figure demonstrates this package in the broader project context.

![Package Overview](https://raw.githubusercontent.com/sdm-lang/rust-sdml/refs/heads/main/doc/overview-generate.png)

## Changes

### Version 0.3.2

* Style: Changed cargo file to use license key instead of license-file.

### Version 0.3.1

* Fix: update dev-dependency for `sdml-parse`.

### Version 0.3.0

* Feature: updates to support the latest grammar, see `sdml-core`.
* Refactor: update generates which use the walker/visitor internally.
* Refactor: drop the old style /write/-style traits and unify into a single
  `Generator` trait.
* Feature: more flexible s-expression generator with more Lisp-y output.
* Test: added a set of macros to test generator output for the set of test
  examples.
  * Added expected output for RDF turtle.
  * Updated existing dependency-tree tests to use these macros.

### Version 0.2.13

* Feature: added new command `doc-book` to create a more complex documentation
  output for a collection of modules.
  * Refactor: added more traits for documentation generation allowing existing
    `doc` code to be reused by `doc-book`.
  * Refactor: added a `common` module for low-level formatting traits.
* Build: bump version of `sdml-errors` and `sdml-core`.

### Version 0.2.12

* Build: update dependency from `sdml_error` to `sdml-errors`.
* Build: bump version of `sdml-core`.

### Version 0.2.11

* Feature: add initial implementation for `DiagramContentFilter`.
  * Add: module and member import filters.
  * Add: local definition filter.
  * Add: association filter.

This release does not use the content filter yet, it is to elicit feedback on
the provided filters.

### Version 0.2.10

* Build: upgrade to `sdml_core` version `0.2.14` and the new `ModelStore` trait.

### Version 0.2.9

* Fix: formatting of annotations was broken for the view command.

### Version 0.2.8

* Feature: adapted to new `HeaderValue` type in core.

### Version 0.2.7

* Feature: Document generation for org-mode now includes the RDF version of a
  module and the dependency graph.
* Fix: The trait function `write_to_string_in_format` was previously calling
  `write`, *not* `write_in_format`.
* Fix: Rustdoc for `convert::source` fixed to turn off colorization.
* Build: Using core `0.2.10` for new stdlib names.

### Version 0.2.6

* Feature: Add color output for RDF source generation.
  1. Rename module `console` to `color`.
  2. Add new `color::rdf` module for helper functions.
  3. Rewrite `convert::rdf` to output colorized listings.
* Feature: Add version URLs into the dependency tree test cases.
* Feature: Removed indirect dependencies from Cargo.toml.

### Version 0.2.5

* Feature: Add `ModuleCache` as parameter to methods on the `GenerateToFile` and
  `GenerateToWriter` traits.
* Feature: Add new `source` generator to show file source, including elided
  versions.
* Feature: Complete dependency generation *logic* -- work to be done on the API.
  * Fix: The depth tests for dependency generation was applied incorrectly.
  * Fix: Correct logic for module dependencies not in the cache.
  * Fix: GraphViz error, edges defined as `-->` should be `->`.
  * Add: Colorize output, with new `console` module to manage global color flag.
  * Add: Add version URI to node output.
* Feature: Add output of RDF definitions to the s-expression generator.
* Feature: Add bare-bones output of RDF definitions to the RDF generator.
* Feature: Add more output to the RDF generator.
* Test: Start new test suites with dependency tree generator.
  * Add a copy of all test examples from `sdml_parse`.
  * Adjust the `test_examples.rs` file so that the macros can take multiple
    generators for testing.
  * Add `generate_dependency_tree` for all `import_*` test cases.

### Version 0.2.4

* Feature: add new stdlib modules with standard layout.
* Feature: minor refactor of cache and loader.

### Version 0.2.3

* Feature: Update to latest grammar for version URIs and RDF definitions.
  * Add support for base URI on modules.
  * Add support for version info and URI on modules.
  * Add support for version URI on module import.
  * Parse RDF definitions for classes and properties.

### Version 0.2.2

* Feature: Remove member groups.

### Version 0.2.1

* Fix: replace `"-"` with `"__"` as qualified identifier replacement.

### Version 0.2.0

* Feature: Update to latest grammar.
  * Remove `ValueVariant` numeric values.
  * Update formal constraints.
  * Add type classes.

### Version 0.1.8

* Feature: Add mapping type to the *s-expr* and *UML* generators.
* Build: Update to latest `tree-sitter-sdml` to pick up changes in highlighting.
* Build: Update to latest `sdml-core` to pick up changes in
  `Cardinality::to_uml_string`.

### Version 0.1.7

* Build: Update with recent model changes and fixes.
* Fix: Clean-up the UML output.

### Version 0.1.6

* Feature: Updated org-mode and UML generators with a number of model changes
  from `sdml-core`.

### Version 0.1.5

* Feature: Updated with a number of model changes from `sdml-core`.

### Version 0.1.4

Previously part of a single crate [sdml](https://crates.io/crates/sdml).
