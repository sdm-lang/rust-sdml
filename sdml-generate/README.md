# Crate sdlm_generate

Rust Library of artifact generators from the Simple Domain Modeling Language (SDML).

[![crates.io](https://img.shields.io/crates/v/sdml_generate.svg)](https://crates.io/crates/sdml_generate)
[![docs.rs](https://docs.rs/sdml_generate/badge.svg)](https://docs.rs/sdml_generate)

## Changes

**Version 0.2.0**

* Feature: Update to latest grammar.
  * Remove Value Variant numeric values
  * Update formal constraints
  * Add type classes

**Version 0.1.8**

* Feature: Add mapping type to the *s-expr* and *UML* generators.
* Build: Update to latest `tree-sitter-sdml` to pick up changes in highlighting.
* Build: Update to latest `sdml-core` to pick up changes in `Cardinality::to_uml_string`.

**Version 0.1.7**

* Build: Update with recent model changes and fixes.
* Fix: Clean-up the UML output.

**Version 0.1.6**

* Feature: Updated org-mode and UML generators with a number of model changes from sdml-core.

**Version 0.1.5**

* Feature: Updated with a number of model changes from sdml-core.

**Version 0.1.4**

Previously part of a single crate [sdml](https://crates.io/crates/sdml).
