# Crate sdlm_parse

Rust Parser for the Simple Domain Modeling Language (SDML).

[![crates.io](https://img.shields.io/crates/v/sdml_parse.svg)](https://crates.io/crates/sdml_parse)
[![docs.rs](https://docs.rs/sdml_parse/badge.svg)](https://docs.rs/sdml_parse)

## Changes

**Version 0.2.2**

* Feature: Update to latest grammar for version URIs and RDF definitions.
  * Add support for base URI on modules
  * Add support for version info and URI on modules
  * Add support for version URI on module import
  * Parse RDF definitions for classes and properties.

**Version 0.2.1**

* Feature: Remove member groups.

**Version 0.2.0**

* Feature: Update to latest grammar.
  * Remove Value Variant numeric values
  * Update formal constraints
  * Add type classes

**Version 0.1.9**

* Added support for `mapping_type` and `mapping_value` rules.

**Version 0.1.8**

* Removed macros from the resolver model.

**Version 0.1.7**

* Updated parser with grammar changes in tree-sitter-sdml v0.1.29.
* Added all the test cases from core.


**Version 0.1.6**

* Updated parser with grammar changes in tree-sitter-sdml v0.1.26.

**Version 0.1.5**

* Created a `stdlib` module and moved all the SDML and relevant RDF files into it.
* Updated `tree-sitter-sdml` to version `0.1.21` with updated constraints.

**Version 0.1.4**

Previously part of a single crate [sdml](https://crates.io/crates/sdml).
